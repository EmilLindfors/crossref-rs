
use serde::{Deserialize, Serialize};

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
    pub last_status_check_time: Option<serde_json::Value>,
    pub counts: Counts,
    pub breakdowns: Option<serde_json::Value>, // You can use Value to represent dynamic data
    pub publisher: String,
    pub coverage: Option<serde_json::Value>,
    pub title: String,
    pub subjects: Vec<String>,
    pub coverage_type: Option<serde_json::Value>,
    pub flags:  Vec<String>,
    #[serde(rename = "ISSN")]
    pub issn: Vec<String>,
    pub issn_type: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct IssnType {
    pub value: String,
    #[serde(rename = "type")]
    pub type_: String, // Renamed to type_ to avoid keyword conflict
}

#[derive(Debug, Deserialize, Serialize)]
struct ApiResponse {
    status: String,
    message_type: String,
    message_version: String,
    message: Journal,
}
