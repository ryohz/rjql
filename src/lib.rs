pub mod error;
pub mod json;

#[cfg(test)]
mod tests {

    use crate::error::Error;

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
        let r = j.refer(&"store.book[0].title".to_string());
        if r.is_err() {
            assert_eq!(1, 2);
        } else {
            let r = r.unwrap();
            assert_eq!("\"Book 1\"", &r);
        }
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
            Error::NotFound => assert_eq!(1, 1),
            _ => assert_eq!(1, 2),
        }
    }

    #[test]
    fn ref_err_dot_start_or_end() {
        let mut j = json::Json::new(&JSON_DATA.to_string());
        let r = j.refer(&".store".to_string()).unwrap_err();
        match r {
            Error::StartOrEndByDot => assert_eq!(1, 1),
            _ => assert_eq!(1, 2),
        }
    }

    #[test]
    fn ref_err_unclosed_bracket() {
        let mut j = json::Json::new(&JSON_DATA.to_string());
        let r = j.refer(&"store.book[0.title".to_string()).unwrap_err();
        match r {
            Error::UnclosedBracket => assert_eq!(1, 1),
            _ => assert_eq!(1, 2),
        }
    }

    #[test]
    fn ref_err_not_num_bracket() {
        let mut j = json::Json::new(&JSON_DATA.to_string());
        let r = j.refer(&"store.book[a].title".to_string()).unwrap_err();
        match r {
            Error::NotNumInBracket => assert_eq!(1, 1),
            _ => assert_eq!(1, 2),
        }
    }

    #[test]
    fn ref_err_invalid_end_of_bracket_token() {
        let mut j = json::Json::new(&JSON_DATA.to_string());
        let r = j.refer(&"store.book[1]aaa.title".to_string()).unwrap_err();
        match r {
            Error::NotEndWithBracket => assert_eq!(1, 1),
            _ => assert_eq!(1, 2),
        }
    }

    #[test]
    fn ref_err_empty_query() {
        let mut j = json::Json::new(&JSON_DATA.to_string());
        let r = j.refer(&"".to_string()).unwrap_err();
        match r {
            Error::EmptyQuery => assert_eq!(1, 1),
            _ => assert_eq!(1, 2),
        }
    }

    #[test]
    fn mod0() {
        let mut j = json::Json::new(&JSON_DATA.to_string());
        let _ = j
            .modify(&"store.game".to_string(), &"hello".to_string())
            .unwrap();
        let r2 = j.refer(&"store.game".to_string()).unwrap();
        assert_eq!("\"hello\"", &r2)
    }

    #[test]
    fn mod1() {
        let mut j = json::Json::new(&JSON_DATA.to_string());
        let _ = j
            .modify(&"store.book[1].title".to_string(), &"hello".to_string())
            .unwrap();
        let r2 = j.refer(&"store.book[1].title".to_string()).unwrap();
        assert_eq!("\"hello\"", &r2)
    }

    #[test]
    fn mod2() {
        let mut j = json::Json::new(&JSON_DATA.to_string());
        let _ = j
            .modify(&"store.book[1].title".to_string(), &"1".to_string())
            .unwrap();
        let r2 = j.refer(&"store.book[1].title".to_string()).unwrap();
        assert_eq!("1", &r2)
    }

    #[test]
    fn mod3() {
        let mut j = json::Json::new(&JSON_DATA.to_string());
        let _ = j
            .modify(&"store.book".to_string(), &"true".to_string())
            .unwrap();
        let r2 = j.refer(&"store.book".to_string()).unwrap();
        assert_eq!("true", &r2)
    }

    #[test]
    fn mod4() {
        let mut j = json::Json::new(&JSON_DATA.to_string());
        let _ = j
            .modify(&"store.test".to_string(), &"test".to_string())
            .unwrap();
        let r2 = j.refer(&"store.test".to_string()).unwrap();
        assert_eq!("\"test\"", &r2)
    }
    #[test]
    fn mod5() {
        let mut j = json::Json::new(&JSON_DATA.to_string());
        let _ = j
            .modify(&"store.test".to_string(), &"test".to_string())
            .unwrap();
        let r2 = j.refer(&"store.test".to_string()).unwrap();
        assert_eq!("\"test\"", &r2)
    }

    #[test]
    fn mod6() {
        let mut j = json::Json::new(&JSON_DATA.to_string());
        let _ = j
            .modify(&"store.game".to_string(), &"[a,1,true]".to_string())
            .unwrap();
        let r2 = j.refer(&"store.game".to_string()).unwrap();
        assert_eq!("[\"a\",1,true]", &r2)
    }

    #[test]
    fn del0() {
        let mut j = json::Json::new(&JSON_DATA.to_string());
        j.delete(&"store.game".to_string()).unwrap();
        let r = j.refer(&"store.game".to_string());
        if r.is_err() {
            match r.unwrap_err() {
                Error::NotFound => assert_eq!(1, 1),
                _ => assert_eq!(1, 2),
            }
        } else {
            assert_eq!(1, 2)
        }
    }

    #[test]
    fn del1() {
        let mut j = json::Json::new(&JSON_DATA.to_string());
        j.delete(&"store.book[0]".to_string()).unwrap();
        let d = r#"
        {
            "store": {
                "book": [
                    { "title": "Book 2", "author": "Author 2" }
                ],
                "bicycle": {
                    "color": "red",
                    "price": 19.95
                },
                "game": null
            }
        }"#;
        let j2 = json::Json::new(&d.to_string());
        assert_eq!(j2.data, j.data)
    }
}
