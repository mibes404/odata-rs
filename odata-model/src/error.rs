use thiserror::Error;

#[derive(Error, Debug)]
pub enum ODataError {
    #[error("invalid Url")]
    Url(#[from] url::ParseError),
    #[error("invalid OData Url; incomplete path")]
    IncompletePath,
}

pub type ODataResult<T> = Result<T, ODataError>;
