use crate::{
    condition::GaiaCondition,
    error::GaiaError,
    result::{GaiaResponse, GaiaResult},
    traits::{Column, Schema, Table},
};

pub struct GaiaQueryBuilder<S: Schema, T: Table, C: Column> {
    schema: S,
    table: T,
    top: Option<usize>,
    columns: Vec<C>,
    conditions: Vec<GaiaCondition<C>>,
}

impl<S: Schema, T: Table, C: Column> GaiaQueryBuilder<S, T, C> {
    pub fn new(schema: S, table: T) -> Self {
        GaiaQueryBuilder {
            schema,
            table,
            top: None,
            columns: Vec::new(),
            conditions: Vec::new(),
        }
    }

    pub fn select(mut self, mut columns: Vec<C>) -> Self {
        self.columns.append(&mut columns);
        self
    }

    pub fn top(mut self, top: usize) -> Self {
        self.top = Some(top);
        self
    }

    pub fn where_clause(mut self, condition: GaiaCondition<C>) -> Self {
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
        query.push_str(&format!(
            " FROM {}.{}",
            self.schema.string(),
            self.table.string()
        ));
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

    pub fn do_query(self) -> Result<GaiaResult<C>, GaiaError> {
        let response = reqwest::blocking::Client::new()
            .get("https://gea.esac.esa.int/tap-server/tap/sync")
            .query(&[
                ("request", "doQuery"),
                ("lang", "ADQL"),
                ("format", "json"),
                ("query", &self.query_string()),
            ])
            .send()?;
        let text = response.text()?;
        let response: GaiaResponse = serde_json::from_str(&text)?;
        Ok(GaiaResult::new(response, self.columns))
    }
}
