
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
struct Counts {
    total_dois: i32,
    current_dois: i32,
    backfile_dois: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
struct Coverage {
    affiliations_current: i32,
    similarity_checking_current: i32,
    descriptions_current: i32,
    ror_ids_current: i32,
    references_backfie: i32,
    funders_backfile: i32,
    licenses_backfile: i32,
    funders_current: i32,
    affiliations_backfile: i32,
    resource_links_backfile: i32,
    orcids_backfile: i32,
    update_policies_current: i32,
    ror_ids_backfile: i32,
    orcids_current: i32,
    similarity_checking_backfile: i32,
    descriptions_backfile: i32,
    award_numbers_backfile: i32,
    update_policies_backfile: i32,
    licenses_current: i32,
    award_numbers_current: i32,
    abstracts_backfile: i32,
    resource_links_current: i32,
    abstracts_current: i32,
    references_current: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
struct Flags {
    deposits_abstracts_current: bool,
    deposits_orcids_current: bool,
    deposits: bool,
    deposits_affiliations_backfile: bool,
    deposits_update_policies_backfile: bool,
    deposits_award_numbers_current: bool,
    deposits_resource_links_current: bool,
    deposits_ror_ids_current: bool,
    deposits_articles: bool,
    deposits_affiliations_current: bool,
    deposits_funders_current: bool,
    deposits_references_backfile: bool,
    deposits_ror_ids_backfile: bool,
    deposits_abstracts_backfile: bool,
    deposits_licenses_backfile: bool,
    deposits_award_numbers_backfile: bool,
    deposits_descriptions_current: bool,
    deposits_references_current: bool,
    deposits_resource_links_backfile: bool,
    deposits_descriptions_backfile: bool,
    deposits_orcids_backfile: bool,
    deposits_funders_backfile: bool,
    deposits_update_policies_current: bool,
    deposits_licenses_current: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
struct CoverageType {
    all: Coverage,
    current: Coverage,
    backfile: Coverage,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Journal {
    last_status_check_time: i64,
    counts: Counts,
    breakdowns: serde_json::Value, // You can use Value to represent dynamic data
    publisher: String,
    coverage: Coverage,
    title: String,
    subjects: Vec<String>,
    coverage_type: CoverageType,
    flags: Flags,
    #[serde(rename = "ISSN")]
    issn: Vec<String>,
    issn_type: IssnType,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
struct IssnType {
    value: String,
    #[serde(rename = "type")]
    type_: String, // Renamed to type_ to avoid keyword conflict
}

#[derive(Debug, Deserialize, Serialize)]
struct ApiResponse {
    status: String,
    message_type: String,
    message_version: String,
    message: Journal,
}
