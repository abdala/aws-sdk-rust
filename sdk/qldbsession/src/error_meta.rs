// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>Returned if the request is malformed or contains an error such as an invalid parameter
    /// value or a missing required parameter.</p>
    BadRequestException(crate::error::BadRequestException),
    /// <p>Returned when the request exceeds the processing capacity of the ledger.</p>
    CapacityExceededException(crate::error::CapacityExceededException),
    /// <p>Returned if the session doesn't exist anymore because it timed out or expired.</p>
    InvalidSessionException(crate::error::InvalidSessionException),
    /// <p>Returned if a resource limit such as number of active sessions is exceeded.</p>
    LimitExceededException(crate::error::LimitExceededException),
    /// <p>Returned when a transaction cannot be written to the journal due to a failure in the
    /// verification phase of <i>optimistic concurrency control</i> (OCC).</p>
    OccConflictException(crate::error::OccConflictException),
    /// <p>Returned when the rate of requests exceeds the allowed throughput.</p>
    RateExceededException(crate::error::RateExceededException),
    /// An unhandled error occurred.
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BadRequestException(inner) => inner.fmt(f),
            Error::CapacityExceededException(inner) => inner.fmt(f),
            Error::InvalidSessionException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::OccConflictException(inner) => inner.fmt(f),
            Error::RateExceededException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::SendCommandError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::SendCommandError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::SendCommandErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::SendCommandErrorKind::CapacityExceededException(inner) => {
                    Error::CapacityExceededException(inner)
                }
                crate::error::SendCommandErrorKind::InvalidSessionException(inner) => {
                    Error::InvalidSessionException(inner)
                }
                crate::error::SendCommandErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::SendCommandErrorKind::OccConflictException(inner) => {
                    Error::OccConflictException(inner)
                }
                crate::error::SendCommandErrorKind::RateExceededException(inner) => {
                    Error::RateExceededException(inner)
                }
                crate::error::SendCommandErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
