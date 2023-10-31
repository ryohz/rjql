pub mod error;
pub mod json;

#[cfg(test)]
mod tests {

    use crate::error::RefError;

    use super::*;

    const JSON_DATA: &str = r#"
        {
            "store": {
                "book": [
                    { "title": "Book 1", "author": "Author 1" },
                    { "title": "Book 2", "author": "Author 2" }
                ],
                "bicycle": {
                    "color": "red",
                    "price": 19.95
                },
                "game": null
            }
        }
    "#;

    #[test]
    fn ref_() {
        let mut j = json::Json::new(&JSON_DATA.to_string());
        let r = j.refer(&"store.book[0].title".to_string()).unwrap();
        assert_eq!("\"Book 1\"", &r);
    }

    #[test]
    fn ref_num() {
        let mut j = json::Json::new(&JSON_DATA.to_string());
        let r = j.refer(&"store.bicycle.price".to_string()).unwrap();
        assert_eq!("19.95", &r)
    }

    #[test]
    fn ref_null() {
        let mut j = json::Json::new(&JSON_DATA.to_string());
        let r = j.refer(&"store.game".to_string()).unwrap();
        assert_eq!("null", &r)
    }

    #[test]
    fn ref_err_not_found() {
        let mut j = json::Json::new(&JSON_DATA.to_string());
        let r = j.refer(&"store.book.math".to_string()).unwrap_err();
        match r {
            RefError::NotFound => assert_eq!(1, 1),
            _ => assert_eq!(1, 2),
        }
    }

    #[test]
    fn ref_err_dot_start_or_end() {
        let mut j = json::Json::new(&JSON_DATA.to_string());
        let r = j.refer(&".store".to_string()).unwrap_err();
        match r {
            RefError::StartOrEndByDot => assert_eq!(1, 1),
            _ => assert_eq!(1, 2),
        }
    }

    #[test]
    fn ref_err_unclosed_bracket() {
        let mut j = json::Json::new(&JSON_DATA.to_string());
        let r = j.refer(&"store.book[0.title".to_string()).unwrap_err();
        match r {
            RefError::UnclosedBracket => assert_eq!(1, 1),
            _ => assert_eq!(1, 2),
        }
    }

    #[test]
    fn ref_err_not_num_bracket() {
        let mut j = json::Json::new(&JSON_DATA.to_string());
        let r = j.refer(&"store.book[a].title".to_string()).unwrap_err();
        match r {
            RefError::NotNumInBracket => assert_eq!(1, 1),
            _ => assert_eq!(1, 2),
        }
    }

    #[test]
    fn ref_err_invalid_end_of_bracket_token() {
        let mut j = json::Json::new(&JSON_DATA.to_string());
        let r = j.refer(&"store.book[1]aaa.title".to_string()).unwrap_err();
        match r {
            RefError::NotEndWithBracket => assert_eq!(1, 1),
            _ => assert_eq!(1, 2),
        }
    }

    #[test]
    fn ref_err_empty_query() {
        let mut j = json::Json::new(&JSON_DATA.to_string());
        let r = j.refer(&"".to_string()).unwrap_err();
        match r {
            RefError::EmptyQuery => assert_eq!(1, 1),
            _ => assert_eq!(1, 2),
        }
    }
}
