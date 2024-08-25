#[cfg(test)]
mod tests {
    use crossref_rs::query::journals::Journals;
    use crossref_rs::query::ResultControl;
    use crossref_rs::{
        CrossrefBuilder, FieldQuery, WorkResultControl, WorksIdentQuery, WorksQuery,
    };

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
        let client = CrossrefBuilder::default().build().unwrap();
        let response = client
            .journal_works(WorksIdentQuery {
                id: "0013-0095".to_string(),
                query: WorksQuery::empty()
                    //.field_query(FieldQuery::container_title("Economic Geography"))
                    .sort(crossref_rs::Sort::Created)
                    .order(crossref_rs::Order::Desc)
                    .result_control(WorkResultControl::Standard(ResultControl::RowsOffset
                        {
                            rows: 1,
                            offset: 1,
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
}
