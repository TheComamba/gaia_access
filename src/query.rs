use crate::{column::GaiaColumn, condition::GaiaCondition, table::GaiaTable};

pub struct GaiaQueryBuilder {
    table: GaiaTable,
    top: Option<usize>,
    columns: Vec<GaiaColumn>,
    conditions: Vec<GaiaCondition>,
}

impl GaiaQueryBuilder {
    pub fn new(table: GaiaTable) -> Self {
        GaiaQueryBuilder {
            table,
            top: None,
            columns: Vec::new(),
            conditions: Vec::new(),
        }
    }

    pub fn select(mut self, mut columns: Vec<GaiaColumn>) -> Self {
        self.columns.append(&mut columns);
        self
    }

    pub fn top(mut self, top: usize) -> Self {
        self.top = Some(top);
        self
    }

    pub fn where_clause(mut self, condition: GaiaCondition) -> Self {
        self.conditions.push(condition);
        self
    }

    pub fn query_string(&self) -> String {
        let mut query = "SELECT".to_string();
        if let Some(top) = self.top {
            query.push_str(&format!(" TOP {}", top));
        }
        query.push_str(&format!(
            " {}",
            self.columns
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ));
        query.push_str(&format!(" FROM {}", self.table.to_string()));
        if !self.conditions.is_empty() {
            query.push_str(&format!(
                " WHERE {}",
                self.conditions
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join(" AND ")
            ));
        }
        query
    }

    pub fn do_query(&self) {
        let response = reqwest::blocking::Client::new()
            .get("https://gea.esac.esa.int/tap-server/tap/sync")
            .query(&[
                ("request", "doQuery"),
                ("lang", "ADQL"),
                ("format", "json"),
                ("query", &self.query_string()),
            ])
            .send()
            .unwrap();
        println!("{}", response.text().unwrap());
    }
}
