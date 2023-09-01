use thiserror::Error;

#[derive(Error, Debug)]
pub enum ODataError {
    #[error("invalid Url")]
    Url(#[from] url::ParseError),
    #[error("invalid OData Url; incomplete path")]
    IncompletePath,
    #[error("the operation is not supported")]
    InvalidOperation,
    #[error("invalid OData query; $top and $skip must be a positive integer")]
    InvalidQueryTopSkip,
    #[error("invalid OData query; incompatible $orderby format")]
    InvalidQueryOrderBy,
}

pub type ODataResult<T> = Result<T, ODataError>;
