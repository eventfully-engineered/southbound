use thiserror::Error;

// TODO: do a pass on error messages and make sure they're decent
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum SouthboundError {
    // #[error("Config error: `{0}`")]
    // ConfigError(#[from] config::ConfigError),
    //
}
