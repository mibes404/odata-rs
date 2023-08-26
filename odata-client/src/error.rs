use thiserror::Error;

#[derive(Error, Debug)]
pub enum ODataClientError {
    #[error("invalid URI")]
    Url(#[from] url::ParseError),
    #[error("invalid HTTP request")]
    Request(#[from] reqwest::Error),
}

pub type ODataClientResult<T> = Result<T, ODataClientError>;
