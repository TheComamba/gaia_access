use crate::columns::GaiaColumn;

pub struct GaiaQueryBuilder {
    table_name: String,
    columns: Vec<GaiaColumn>,
    conditions: Vec<String>,
}

impl GaiaQueryBuilder {
    pub fn new(table_name: &str) -> Self {
        GaiaQueryBuilder {
            table_name: table_name.to_string(),
            columns: Vec::new(),
            conditions: Vec::new(),
        }
    }

    pub fn select(mut self, mut columns: Vec<GaiaColumn>) -> Self {
        self.columns.append(&mut columns);
        self
    }

    pub fn from(mut self, table_name: &str) -> Self {
        self.table_name = table_name.to_string();
        self
    }

    pub fn where_clause(mut self, condition: &str) -> Self {
        self.conditions.push(condition.to_string());
        self
    }

    pub fn query_string(self) -> String {
        let mut query = format!(
            "SELECT {} FROM {}",
            self.columns
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            self.table_name
        );
        if !self.conditions.is_empty() {
            query.push_str(&format!(" WHERE {}", self.conditions.join(" AND ")));
        }
        query
    }
}
