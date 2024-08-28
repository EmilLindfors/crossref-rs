use std::{collections::HashMap, hash::Hash};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::error::ErrorKind;

use super::{JournalList, MessageType, QueryResponse};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Counts {
    pub total_dois: i32,
    pub current_dois: i32,
    pub backfile_dois: i32,
}

//#[derive(Debug, Deserialize, Serialize, Clone)]
//#[serde(rename_all = "kebab-case")]
//pub struct Coverage {
//    pub affiliations_current: f32,
//    pub similarity_checking_current: f32,
//    pub descriptions_current: f32,
//    pub ror_ids_current: f32,
//    pub references_backfie: f32,
//    pub funders_backfile: f32,
//    pub licenses_backfile: f32,
//    pub funders_current: f32,
//    pub affiliations_backfile: f32,
//    pub resource_links_backfile: f32,
//    pub orcids_backfile: f32,
//    pub update_policies_current: f32,
//    pub ror_ids_backfile: f32,
//    pub orcids_current: f32,
//    pub similarity_checking_backfile: f32,
//    pub descriptions_backfile: f32,
//    pub award_numbers_backfile: f32,
//    pub update_policies_backfile: f32,
//    pub licenses_current: f32,
//    pub award_numbers_current: f32,
//    pub abstracts_backfile: f32,
//    pub resource_links_current: f32,
//    pub abstracts_current: f32,
//    pub references_current: f32,
//}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Flags {
    pub deposits_abstracts_current: bool,
    pub deposits_orcids_current: bool,
    pub deposits: bool,
    pub deposits_affiliations_backfile: bool,
    pub deposits_update_policies_backfile: bool,
    pub deposits_award_numbers_current: bool,
    pub deposits_resource_links_current: bool,
    pub deposits_ror_ids_current: bool,
    pub deposits_articles: bool,
    pub deposits_affiliations_current: bool,
    pub deposits_funders_current: bool,
    pub deposits_references_backfile: bool,
    pub deposits_ror_ids_backfile: bool,
    pub deposits_abstracts_backfile: bool,
    pub deposits_licenses_backfile: bool,
    pub deposits_award_numbers_backfile: bool,
    pub deposits_descriptions_current: bool,
    pub deposits_references_current: bool,
    pub deposits_resource_links_backfile: bool,
    pub deposits_descriptions_backfile: bool,
    pub deposits_orcids_backfile: bool,
    pub deposits_funders_backfile: bool,
    pub deposits_update_policies_current: bool,
    pub deposits_licenses_current: bool,
}

//#[derive(Debug, Deserialize, Serialize, Clone)]
//#[serde(rename_all = "kebab-case")]
//pub struct CoverageType {
//    pub all: Coverage,
//    pub current: Coverage,
//    pub backfile: Coverage,
//}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Journal {
    pub last_status_check_time: Option<DateTime<Utc>>,
    pub counts: Counts,
    pub breakdowns: Vec<(i64, i64)>, // You can use Value to represent dynamic data
    pub publisher: String,
    pub coverage: Option<serde_json::Value>,
    pub title: String,
    pub subjects: Vec<String>,
    pub coverage_type: Option<serde_json::Value>,
    pub flags: Vec<(String, bool)>,
    #[serde(rename = "ISSN")]
    pub issn: Vec<String>,
    pub issn_type: Vec<IssnType>,
}

impl TryFrom<serde_json::Value> for Journal {
    type Error = ErrorKind;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        match value {
            serde_json::Value::Object(map) => {
                let last_status_check_time = map
                    .get("last-status-check-time")
                    .ok_or(ErrorKind::MissingField {
                        msg: "last-status-check-time".to_string(),
                    })?
                    .as_i64()
                    .ok_or(ErrorKind::InvalidTypeName {
                        name: "last-status-check-time".to_string(),
                    })?;
                let counts = map
                    .get("counts")
                    .ok_or(ErrorKind::MissingField {
                        msg: "counts".to_string(),
                    })?
                    .as_object()
                    .ok_or(ErrorKind::InvalidTypeName {
                        name: "counts".to_string(),
                    })?;
                let mut breakdowns: Vec<(i64, i64)> = map
                    .get("breakdowns")
                    .ok_or(ErrorKind::MissingField {
                        msg: "breakdowns".to_string(),
                    })?
                    .get("dois-by-issued-year")
                    .ok_or(ErrorKind::MissingField {
                        msg: "dois-by-issued-year".to_string(),
                    })?
                    .as_array()
                    .ok_or(ErrorKind::InvalidTypeName {
                        name: "breakdowns".to_string(),
                    })?
                    .iter()
                    .map(|v| {
                        let arr = v.as_array().unwrap();
                        (arr[0].as_i64().unwrap(), arr[1].as_i64().unwrap())
                    })
                    .collect();

                breakdowns.sort_by(|a, b| a.0.cmp(&b.0));

                let publisher = map
                    .get("publisher")
                    .ok_or(ErrorKind::MissingField {
                        msg: "publisher".to_string(),
                    })?
                    .as_str()
                    .ok_or(ErrorKind::InvalidTypeName {
                        name: "publisher".to_string(),
                    })?
                    .to_string();

                let title = map
                    .get("title")
                    .ok_or(ErrorKind::MissingField {
                        msg: "title".to_string(),
                    })?
                    .as_str()
                    .ok_or(ErrorKind::InvalidTypeName {
                        name: "title".to_string(),
                    })?
                    .to_string();

                let subjects = map
                    .get("subjects")
                    .ok_or(ErrorKind::MissingField {
                        msg: "subjects".to_string(),
                    })?
                    .as_array()
                    .ok_or(ErrorKind::InvalidTypeName {
                        name: "subjects".to_string(),
                    })?
                    .iter()
                    .map(|v| v.as_str().unwrap().to_string())
                    .collect();

                let coverage = map.get("coverage").map(|v| v.clone());

                let coverage_type = map.get("coverage-type").map(|v| v.clone());

                let flags = map
                    .get("flags")
                    .ok_or(ErrorKind::MissingField {
                        msg: "flags".to_string(),
                    })?
                    .as_object()
                    .ok_or(ErrorKind::InvalidTypeName {
                        name: "flags".to_string(),
                    })?
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.as_bool().unwrap()))
                    .collect();

                let issn = map
                    .get("ISSN")
                    .ok_or(ErrorKind::MissingField {
                        msg: "ISSN".to_string(),
                    })?
                    .as_array()
                    .ok_or(ErrorKind::InvalidTypeName {
                        name: "ISSN".to_string(),
                    })?
                    .iter()
                    .map(|v| v.as_str().unwrap().to_string())
                    .collect();

                let issn_type = map
                    .get("issn-type")
                    .ok_or(ErrorKind::MissingField {
                        msg: "issn-type".to_string(),
                    })?
                    .as_array()
                    .ok_or(ErrorKind::InvalidTypeName {
                        name: "issn-type".to_string(),
                    })?
                    .iter()
                    .map(|v| serde_json::from_value(v.clone()).unwrap())
                    .collect();

                let last_status_check_time =
                    DateTime::from_timestamp_millis(last_status_check_time);

                Ok(Journal {
                    last_status_check_time,
                    counts: serde_json::from_value(serde_json::Value::Object(counts.clone()))
                        .unwrap(),
                    breakdowns,
                    publisher,
                    coverage,
                    title,
                    subjects,
                    coverage_type,
                    flags,
                    issn,
                    issn_type,
                })
            }
            _ => Err(ErrorKind::InvalidMessageType {
                name: value.to_string(),
            }),
        }
    }
}

impl TryFrom<serde_json::Value> for JournalList {
    type Error = ErrorKind;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        match value {
            serde_json::Value::Object(map) => {
                let total_results = map.get("total-results").unwrap().as_i64().unwrap() as usize;

                let items_per_page = map
                    .get("items-per-page")
                    .map(|v| v.as_i64().unwrap() as usize);

                let query = map
                    .get("query")
                    .map(|v| match v {
                        serde_json::Value::Object(map) => {
                            let start_index =
                                map.get("start-index").unwrap().as_i64().unwrap() as usize;
                            let query = map
                                .get("search_terms")
                                .map(|v| v.as_str().unwrap().to_string());
                            Some(QueryResponse {
                                start_index,
                                search_terms: query,
                            })
                        }
                        _ => None,
                    })
                    .flatten();

                let items = map
                    .get("items")
                    .ok_or(ErrorKind::MissingField {
                        msg: "items".to_string(),
                    })?
                    .as_array()
                    .ok_or(ErrorKind::InvalidTypeName {
                        name: "items".to_string(),
                    })?
                    .iter()
                    .map(|v| Journal::try_from(v.clone()).unwrap())
                    .collect();

                Ok(JournalList {
                    total_results,
                    items_per_page,
                    query,
                    facets: HashMap::new(),
                    items,
                })
            }
            _ => Err(ErrorKind::InvalidMessageType {
                name: value.to_string(),
            }),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct IssnType {
    pub value: String,
    #[serde(rename = "type")]
    pub type_: String, // Renamed to type_ to avoid keyword conflict
}
