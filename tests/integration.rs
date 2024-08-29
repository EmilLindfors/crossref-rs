use std::borrow::Cow;

use simd_json::{base::ValueAsScalar, BorrowedValue};

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use mimalloc::MiMalloc;
    use crossref_rs::query::journals::{JournalResultControl, Journals};
    use crossref_rs::query::ResultControl;
    use crossref_rs::response::MessageType;
    use crossref_rs::{
        CrossrefBuilder, FieldQuery, Type, WorkResultControl, WorksFilter, WorksIdentQuery,
        WorksQuery,
    };
    use reqwest::Url;
    use simd_json::base::{ValueAsArray, ValueAsObject, ValueAsScalar};

    use crate::{ErrorMessageType, MyMessageBody, MyMessageHeader, MyMessageQuery, MyMessageType};
     #[global_allocator]
    static GLOBAL: MiMalloc = MiMalloc;

    #[tokio::test]
    async fn test_journal_by_name() {
        let client = CrossrefBuilder::default().build().unwrap();
        let response = client
            .works(
                WorksQuery::empty()
                    .field_query(FieldQuery::container_title("Economic Geography"))
                    .result_control(WorkResultControl::Standard(ResultControl::Rows(1))),
            )
            .await;
        println!("{:?}", response);
        assert!(response.is_ok());
        let work = response.unwrap().items.into_iter().next().unwrap();

        assert!(work
            .container_title
            .unwrap()
            .contains(&"Economic Geography".to_string()));
    }

    #[tokio::test]
    async fn test_journal_by_issn() {
        let client = CrossrefBuilder::default().build().unwrap();
        let jorunal = client.journal("0013-0095").await;
        println!("{:?}", jorunal);
        assert!(jorunal.is_ok());
    }

    #[tokio::test]
    async fn test_work_by_doi() {
        let client = CrossrefBuilder::default().build().unwrap();
        let work = client.work("10.5555/12345678").await;
        println!("{:?}", work);
        assert!(work.is_ok());
    }

    #[tokio::test]
    async fn test_works_by_author() {
        let client = CrossrefBuilder::default().build().unwrap();
        let response = client
            .works(
                WorksQuery::empty()
                    .field_query(FieldQuery::author("Emil Lindfors"))
                    .result_control(WorkResultControl::Standard(ResultControl::Rows(1))),
            )
            .await;
        println!("{:?}", response);
        assert!(response.is_ok());
        let work = response.unwrap().items.into_iter().next().unwrap();

        assert!(work.author.unwrap().iter().any(|a| a
            .family
            .as_ref()
            .unwrap()
            .contains(&"Lindfors".to_string())));
    }

    #[tokio::test]
    async fn combined_query() {
        _ = tracing_subscriber::fmt::init();
        let client = CrossrefBuilder::default().build().unwrap();
        let span = tracing::info_span!("combined_query");
        let _guard = span.enter();
        let response = client
            .journal_works(WorksIdentQuery {
                id: "0013-0095".to_string(),
                query: WorksQuery::empty()
                    //.field_query(FieldQuery::container_title("Economic Geography"))
                    .filter(WorksFilter::Type(Type::JournalArticle))
                    .sort(crossref_rs::Sort::Created)
                    .order(crossref_rs::Order::Desc)
                    .result_control(WorkResultControl::Standard(ResultControl::RowsOffset {
                        rows: 10,
                        offset: 20,
                    })),
            })
            .await;
        println!("{:?}", response);
        assert!(response.is_ok());
        let work = response.unwrap().items.into_iter().next().unwrap();

        assert!(work
            .container_title
            .unwrap()
            .contains(&"Economic Geography".to_string()));
    }

    #[tokio::test]
    async fn journal_query() {
        
        let client = CrossrefBuilder::default().build().unwrap();
        let control = Some(JournalResultControl::new_from_limit(10));
        let response = client
            .journals("Economic Geography".to_string(), control)
            .await;
        println!("{:?}", response);
        assert!(response.is_ok());
        let journal = response.unwrap().items.into_iter().next().unwrap();

        assert!(journal.title.contains(&"Economic Geography".to_string()));
    }
    #[tokio::test]
    async fn stream() -> Result<(), ErrorMessageType> {
        use crate::MessageError;
        use futures_util::StreamExt;
        use mimalloc::MiMalloc;
        use reqwest::Method;

        let mut request = reqwest::Request::new(
            Method::GET,
            Url::parse("https://api.crossref.org/journals/?query=Economic+Geography&rows=10")
                .unwrap(),
        );
        request.headers_mut().insert(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        request.headers_mut().insert(
            reqwest::header::USER_AGENT,
            format!("mailto:etli@hvl.no").parse().unwrap(),
        );

        let mut stream = reqwest::Client::new()
            .execute(request)
            .await
            .unwrap()
            .bytes_stream();

        let mut buffer = Vec::new();

        while let Some(chunk) = stream.next().await {
            //println!("{:?}", chunk);
            let chunk = chunk.unwrap();

            buffer.extend_from_slice(&chunk);

            loop {
                let res = simd_json::to_borrowed_value(&mut buffer);

                if let Ok(v) = res {
                    match v {
                        simd_json::BorrowedValue::Object(o) => {
                            match o.get("status").map(|v| v.as_str().unwrap()).unwrap() {
                                "ok" => {
                                    let message_type =
                                        MyMessageType::try_from(o.get("message-type")).unwrap();
                                    let message_version: &str = o
                                        .get("message-version")
                                        .map(|v| v.as_str().unwrap())
                                        .unwrap();
                                    let header = MyMessageHeader::new(
                                        message_type,
                                        Cow::Borrowed(message_version),
                                    );
                                    println!("{:?}", header);
                                    let message = o.get("message").unwrap();
                                    let message_object = message.as_object().unwrap();
                                    let items_per_page = message_object
                                        .get("items-per-page")
                                        .and_then(|v| v.as_i32())
                                        .ok_or(MessageError::ParseError)
                                        .unwrap();
                                    let total_results = message_object
                                        .get("total-results")
                                        .and_then(|v| v.as_i32())
                                        .ok_or(MessageError::ParseError)
                                        .unwrap();

                                    let query = message_object
                                        .get("query")
                                        .and_then(|v| Some(MyMessageQuery::try_from(v).unwrap()))
                                        .unwrap();
                                    //println!("{:?}", query);
                                    //println!("{:?}", items_per_page);
                                    //println!("{:?}", total_results);

                                    let body = message_object.get("items").unwrap();
                                    let body_array = body.as_array().unwrap();
                                    //for item in body_array {
                                    //    let object = item.as_object().unwrap();
                                    //    let title = object.get("title").unwrap().as_str().unwrap();
                                    //    //println!("{:?}", title);
                                    //}
                                }
                                "error" => {
                                    let error_type =
                                        ErrorMessageType::try_from(o.get("message-type")).unwrap();
                                    return Err(error_type);
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                    break;
                } else {
                    break;
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum MyMessageError {
    RouteNotFound,
}

#[derive(Debug)]
pub enum ErrorMessageType {
    RouteNotFound,
}

impl TryFrom<Option<&BorrowedValue<'_>>> for ErrorMessageType {
    type Error = MessageError;

    fn try_from(value: Option<&BorrowedValue<'_>>) -> Result<Self, Self::Error> {
        if let Some(value) = value {
            match value {
                BorrowedValue::String(s) => match s {
                    Cow::Borrowed("route-not-found") => Ok(ErrorMessageType::RouteNotFound),
                    _ => Err(MessageError::MessageTypeNotSupported),
                },
                _ => Err(MessageError::ParseError),
            }
        } else {
            Err(MessageError::ParseError)
        }
    }
}

#[derive(Debug)]
pub struct MyMessageHeader<'a> {
    message_type: MyMessageType,
    message_version: Cow<'a, str>,
}

impl<'a> MyMessageHeader<'a> {
    pub fn new(message_type: MyMessageType, message_version: Cow<'a, str>) -> Self {
        Self {
            message_type,
            message_version,
        }
    }
}
#[derive(Debug)]
pub struct MyMessageQuery<'a> {
    search_terms: Cow<'a, str>,
    start_index: i32,
}

impl<'a> TryFrom<&'a BorrowedValue<'a>> for MyMessageQuery<'a> {
    type Error = MessageError;

    fn try_from(value: &'a BorrowedValue<'a>) -> Result<Self, Self::Error> {
        if let BorrowedValue::Object(o) = value {
            let search_terms = o
                .get("search-terms")
                .and_then(|v| v.as_str())
                .ok_or(MessageError::ParseError)?;
            let start_index = o
                .get("start-index")
                .and_then(|v| v.as_i32())
                .ok_or(MessageError::ParseError)?;

            Ok(MyMessageQuery {
                search_terms: Cow::Borrowed(search_terms),
                start_index,
            })
        } else {
            Err(MessageError::ParseError)
        }
    }
}

#[derive(Debug)]
pub struct MyMessageBody<'a> {
    items_per_page: i32,
    total_results: i32,
    query: MyMessageQuery<'a>,
}

impl<'a> TryFrom<&'a BorrowedValue<'a>> for MyMessageBody<'a> {
    type Error = MessageError;

    fn try_from(value: &'a BorrowedValue<'a>) -> Result<Self, Self::Error> {
        if let BorrowedValue::Object(o) = value {
            let items_per_page = o
                .get("items-per-page")
                .and_then(|v| v.as_i32())
                .ok_or(MessageError::ParseError)?;
            let total_results = o
                .get("total-results")
                .and_then(|v| v.as_i32())
                .ok_or(MessageError::ParseError)?;

            let query = o
                .get("query")
                .and_then(|v| Some(MyMessageQuery::try_from(v).unwrap()))
                .unwrap();
            Ok(MyMessageBody {
                items_per_page,
                total_results,
                query,
            })
        } else {
            Err(MessageError::ParseError)
        }
    }
}

#[derive(Debug)]
pub enum MessageError {
    MessageTypeNotSupported,
    ParseError,
}

#[derive(Debug)]
pub enum MyMessageType {
    Journal,
    JournalList,
    Book,
    WorkAgency,
}

impl TryFrom<Option<&BorrowedValue<'_>>> for MyMessageType {
    type Error = MessageError;
    fn try_from(value: Option<&BorrowedValue<'_>>) -> Result<Self, Self::Error> {
        if let Some(value) = value {
            match value {
                BorrowedValue::String(s) => match s {
                    Cow::Borrowed("journal") => Ok(MyMessageType::Journal),
                    Cow::Borrowed("journal-list") => Ok(MyMessageType::JournalList),
                    Cow::Borrowed("book") => Ok(MyMessageType::Book),
                    Cow::Borrowed("work-agency") => Ok(MyMessageType::WorkAgency),
                    _ => Err(MessageError::MessageTypeNotSupported),
                },
                _ => Err(MessageError::ParseError),
            }
        } else {
            Err(MessageError::ParseError)
        }
    }
}
