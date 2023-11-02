use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("No value is found by specified query")]
    NotFound,
    #[error("the query starts or ends by dot(.)")]
    StartOrEndByDot,
    #[error("the query contains an unclosed bracket expression")]
    UnclosedBracket,
    #[error("value in a bracket is not number")]
    NotNumInBracket,
    #[error("a token with brackets ends with a letter other than bracket")]
    NotEndWithBracket,
    #[error("query is empty")]
    EmptyQuery
}
