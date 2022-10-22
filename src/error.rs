use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("no semver pattern found in '{0}'")]
    NoSemverFound(String),

    #[error("invalid segment count: {0}")]
    InvalidSegmentCount(usize),
}
