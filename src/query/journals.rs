use crate::error::{ErrorKind, Result};
use crate::query::works::{WorksCombiner, WorksFilter, WorksIdentQuery, WorksQuery};
use crate::query::{Component, CrossrefQuery, CrossrefRoute, ResourceComponent};
use crate::WorkResultControl;

#[derive(Debug, Clone)]
pub struct JournalResultControl {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub sample: Option<bool>,
    pub sort: Option<String>,
}

impl JournalResultControl {
    pub fn new(limit: Option<usize>, offset: Option<usize>, sample: Option<bool>, sort: Option<String>) -> Self {
        JournalResultControl {
            limit,
            offset,
            sample,
            sort,
        }
    }

    pub fn new_from_limit(limit: usize) -> Self {
        JournalResultControl {
            limit: Some(limit),
            offset: None,
            sample: None,
            sort: None,
        }
    }

    pub fn new_from_offset(offset: usize) -> Self {
        JournalResultControl {
            limit: None,
            offset: Some(offset),
            sample: None,
            sort: None,
        }
    }

    pub fn new_from_sample(sample: bool) -> Self {
        JournalResultControl {
            limit: None,
            offset: None,
            sample: Some(sample),
            sort: None,
        }
    }

    pub fn new_from_sort(sort: String) -> Self {
        JournalResultControl {
            limit: None,
            offset: None,
            sample: None,
            sort: Some(sort),
        }
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn sample(mut self, sample: bool) -> Self {
        self.sample = Some(sample);
        self
    }

    pub fn sort(mut self, sort: String) -> Self {
        self.sort = Some(sort);
        self
    }

    pub fn to_string(&self) -> String {
        let mut rc = String::new();
        if let Some(l) = self.limit {
            rc.push_str(&format!("rows={}&", l));
        }
        if let Some(o) = self.offset {
            rc.push_str(&format!("offset={}&", o));
        }
        if let Some(s) = self.sample {
            rc.push_str(&format!("sample={}&", s));
        }
        if let Some(s) = &self.sort {
            rc.push_str(&format!("sort={}&", s));
        }
        rc.pop(); // remove trailing '&'
        rc
    }
}

impl TryFrom<String> for JournalResultControl {
    type Error = ErrorKind;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        
    
        let mut limit = None;
        let mut offset = None;
        let mut sample = None;
        let mut sort = None;

        let parts = value.split('&').collect::<Vec<&str>>();
        for part in parts {
            let kv = part.split('=').collect::<Vec<&str>>();
            if kv.len() != 2 {
                return Err(ErrorKind::InvalidResultControl {
                    error: value.clone(),
                });
            }
            match kv[0] {
                "rows" => {
                    limit = Some(kv[1].parse().map_err(|e| ErrorKind::InvalidResultControl {
                        error: format!("Invalid limit: {}", e),
                    })?);
                }
                "offset" => {
                    offset = Some(kv[1].parse().map_err(|e| ErrorKind::InvalidResultControl {
                        error: format!("Invalid offset: {}", e),
                    })?);
                }
                "sample" => {
                    sample = Some(kv[1].parse().map_err(|e| ErrorKind::InvalidResultControl {
                        error: format!("Invalid sample: {}", e),
                    })?);
                }
                "sort" => {
                    sort = Some(kv[1].to_string());
                }
                _ => return Err(ErrorKind::InvalidResultControl { error: value.clone() }),
            }
        }

        Ok(JournalResultControl {
            limit,
            offset,
            sample,
            sort,
        })
    }
}



/// constructs the request payload for the `/journals` route
#[derive(Debug, Clone)]
pub enum Journals {
    /// target a specific journal at `/journals/{id}`
    Identifier(String),
    /// target a `Work` for a specific funder at `/journals/{id}/works?query..`
    Works(WorksIdentQuery),
    /// free form query for `/journals?query...`
    Query(String, Option<JournalResultControl>),
}

impl CrossrefRoute for Journals {
    fn route(&self) -> Result<String> {
        match self {
            Journals::Identifier(s) => Ok(format!("{}/{}", Component::Journals.route()?, s)),
            Journals::Query(query, result_control) => {
                let q = query.split(' ').collect::<Vec<&str>>().join("+");
                if let Some(rc) = result_control {
                    if query.is_empty() {
                        Ok(format!("{}/?{}", Component::Journals.route()?, rc.to_string()))
                    } else {
                        Ok(format!("{}/?query={}&{}", Component::Journals.route()?, q, rc.to_string()))
                    }
                } else {
                    Ok(format!("{}/?query={}", Component::Journals.route()?, q))
                }
            }
            Journals::Works(combined) => Self::combined_route(combined),
        }
    }
}

impl CrossrefQuery for Journals {
    fn resource_component(self) -> ResourceComponent {
        ResourceComponent::Journals(self)
    }
}
