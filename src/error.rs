use thiserror::Error;

#[derive(Debug, Error)]
pub enum RtpError {
  #[error("Invalid header field")]
  InvalidHeaderField,
}
