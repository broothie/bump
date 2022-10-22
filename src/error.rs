#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("no semver pattern found in '{0}'")]
    NoSemverFound(String),
}
