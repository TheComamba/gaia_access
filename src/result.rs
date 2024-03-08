use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::column::GaiaColumn;

#[derive(Serialize, Deserialize)]
struct GaiaMetadataLine {
    name: String,
    datatype: String,
    xtype: Option<String>,
    arraysize: Option<String>,
    description: String,
    unit: Option<String>,
    ucd: String,
    utype: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum GaiaCellData {
    String(String),
    Float(f64),
    Null,
}

pub fn get_string(data: &GaiaCellData) -> Option<String> {
    match data {
        GaiaCellData::String(string) => Some(string.clone()),
        _ => None,
    }
}

pub fn get_float(data: &GaiaCellData) -> Option<f64> {
    match data {
        GaiaCellData::Float(float) => Some(*float),
        _ => None,
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct GaiaResponse {
    metadata: Vec<GaiaMetadataLine>,
    data: Vec<Vec<GaiaCellData>>,
}

pub struct GaiaStar {
    pub data: HashMap<GaiaColumn, GaiaCellData>,
}

pub struct GaiaResult {
    pub columns: Vec<GaiaColumn>,
    pub stars: Vec<GaiaStar>,
}

impl GaiaResult {
    pub(super) fn new(response: GaiaResponse, columns: Vec<GaiaColumn>) -> Self {
        let mut stars = Vec::new();
        for row in response.data {
            let mut star = HashMap::new();
            for (i, cell) in row.into_iter().enumerate() {
                star.insert(columns[i], cell);
            }
            stars.push(GaiaStar { data: star });
        }
        GaiaResult { columns, stars }
    }
}
