
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Counts {
    pub total_dois: i32,
    pub current_dois: i32,
    pub backfile_dois: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Coverage {
    pub affiliations_current: i32,
    pub similarity_checking_current: i32,
    pub descriptions_current: i32,
    pub ror_ids_current: i32,
    pub references_backfie: i32,
    pub funders_backfile: i32,
    pub licenses_backfile: i32,
    pub funders_current: i32,
    pub affiliations_backfile: i32,
    pub resource_links_backfile: i32,
    pub orcids_backfile: i32,
    pub update_policies_current: i32,
    pub ror_ids_backfile: i32,
    pub orcids_current: i32,
    pub similarity_checking_backfile: i32,
    pub descriptions_backfile: i32,
    pub award_numbers_backfile: i32,
    pub update_policies_backfile: i32,
    pub licenses_current: i32,
    pub award_numbers_current: i32,
    pub abstracts_backfile: i32,
    pub resource_links_current: i32,
    pub abstracts_current: i32,
    pub references_current: i32,
}

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

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct CoverageType {
    pub all: Coverage,
    pub current: Coverage,
    pub backfile: Coverage,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Journal {
    pub last_status_check_time: i64,
    pub counts: Counts,
    pub breakdowns: serde_json::Value, // You can use Value to represent dynamic data
    pub publisher: String,
    pub coverage: Coverage,
    pub title: String,
    pub subjects: Vec<String>,
    pub coverage_type: CoverageType,
    pub flags: Flags,
    #[serde(rename = "ISSN")]
    pub issn: Vec<String>,
    pub issn_type: IssnType,
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
