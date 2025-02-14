// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>Indicates that a request to authorize a client with an access user session token is pending.</p>
    AuthorizationPendingException(crate::error::AuthorizationPendingException),
    /// <p>Indicates that the token issued by the service is expired and is no longer valid.</p>
    ExpiredTokenException(crate::error::ExpiredTokenException),
    /// <p>Indicates that an error from the service occurred while trying to process a request.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>Indicates that the <code>clientId</code> or <code>clientSecret</code> in the request is
    /// invalid. For example, this can occur when a client sends an incorrect <code>clientId</code> or
    /// an expired <code>clientSecret</code>.</p>
    InvalidClientException(crate::error::InvalidClientException),
    /// <p>Indicates that the client information sent in the request during registration is invalid.</p>
    InvalidClientMetadataException(crate::error::InvalidClientMetadataException),
    /// <p>Indicates that a request contains an invalid grant. This can occur if a client makes a <a>CreateToken</a> request with an invalid grant type.</p>
    InvalidGrantException(crate::error::InvalidGrantException),
    /// <p>Indicates that something is wrong with the input to the request. For example, a required
    /// parameter might be missing or out of range.</p>
    InvalidRequestException(crate::error::InvalidRequestException),
    /// <p>Indicates that the scope provided in the request is invalid.</p>
    InvalidScopeException(crate::error::InvalidScopeException),
    /// <p>Indicates that the client is making the request too frequently and is more than the service can handle. </p>
    SlowDownException(crate::error::SlowDownException),
    /// <p>Indicates that the client is not currently authorized to make the request. This can happen
    /// when a <code>clientId</code> is not issued for a public client.</p>
    UnauthorizedClientException(crate::error::UnauthorizedClientException),
    /// <p>Indicates that the grant type in the request is not supported by the service.</p>
    UnsupportedGrantTypeException(crate::error::UnsupportedGrantTypeException),
    /// An unhandled error occurred.
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::AuthorizationPendingException(inner) => inner.fmt(f),
            Error::ExpiredTokenException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::InvalidClientException(inner) => inner.fmt(f),
            Error::InvalidClientMetadataException(inner) => inner.fmt(f),
            Error::InvalidGrantException(inner) => inner.fmt(f),
            Error::InvalidRequestException(inner) => inner.fmt(f),
            Error::InvalidScopeException(inner) => inner.fmt(f),
            Error::SlowDownException(inner) => inner.fmt(f),
            Error::UnauthorizedClientException(inner) => inner.fmt(f),
            Error::UnsupportedGrantTypeException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateTokenError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateTokenError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateTokenErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::CreateTokenErrorKind::AuthorizationPendingException(inner) => {
                    Error::AuthorizationPendingException(inner)
                }
                crate::error::CreateTokenErrorKind::ExpiredTokenException(inner) => {
                    Error::ExpiredTokenException(inner)
                }
                crate::error::CreateTokenErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::CreateTokenErrorKind::InvalidClientException(inner) => {
                    Error::InvalidClientException(inner)
                }
                crate::error::CreateTokenErrorKind::InvalidGrantException(inner) => {
                    Error::InvalidGrantException(inner)
                }
                crate::error::CreateTokenErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::CreateTokenErrorKind::InvalidScopeException(inner) => {
                    Error::InvalidScopeException(inner)
                }
                crate::error::CreateTokenErrorKind::SlowDownException(inner) => {
                    Error::SlowDownException(inner)
                }
                crate::error::CreateTokenErrorKind::UnauthorizedClientException(inner) => {
                    Error::UnauthorizedClientException(inner)
                }
                crate::error::CreateTokenErrorKind::UnsupportedGrantTypeException(inner) => {
                    Error::UnsupportedGrantTypeException(inner)
                }
                crate::error::CreateTokenErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::RegisterClientError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::RegisterClientError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::RegisterClientErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::RegisterClientErrorKind::InvalidClientMetadataException(inner) => {
                    Error::InvalidClientMetadataException(inner)
                }
                crate::error::RegisterClientErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::RegisterClientErrorKind::InvalidScopeException(inner) => {
                    Error::InvalidScopeException(inner)
                }
                crate::error::RegisterClientErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StartDeviceAuthorizationError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::StartDeviceAuthorizationError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::StartDeviceAuthorizationErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::StartDeviceAuthorizationErrorKind::InvalidClientException(inner) => {
                    Error::InvalidClientException(inner)
                }
                crate::error::StartDeviceAuthorizationErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::StartDeviceAuthorizationErrorKind::SlowDownException(inner) => {
                    Error::SlowDownException(inner)
                }
                crate::error::StartDeviceAuthorizationErrorKind::UnauthorizedClientException(
                    inner,
                ) => Error::UnauthorizedClientException(inner),
                crate::error::StartDeviceAuthorizationErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
