#[derive(thiserror::Error, Debug)]
pub enum ClicError {
    #[error("Too many consecutive id creation retry attempts")]
    TooManyIDRetries,
    #[error("please ensure $HOME environment variable is set")]
    MissingHomeDir,
    #[error("record id does not exist in clic")]
    NonExistentId(String),
    #[error("no gist id: please initialize sync w/ init-web ")]
    NoGistId
}
