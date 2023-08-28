use thiserror::Error;

#[derive(Error, Debug)]
pub enum ODataError {
    #[error("invalid Url")]
    Url(#[from] url::ParseError),
    #[error("invalid OData Url; incomplete path")]
    IncompletePath,
    #[error("the operation is not supported")]
    InvalidOperation,
}

pub type ODataResult<T> = Result<T, ODataError>;
