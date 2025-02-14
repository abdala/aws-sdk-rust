// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>An internal error has occurred. Try your call again.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>The value supplied for the input parameter is out of range or not valid.</p>
    InvalidParameterValueException(crate::error::InvalidParameterValueException),
    /// <p>The request exceeds a limit of the service.</p>
    LimitExceededException(crate::error::LimitExceededException),
    /// <p>The request must contain either a valid (registered) Amazon Web Services access key ID
    /// or X.509 certificate.</p>
    MissingAuthenticationToken(crate::error::MissingAuthenticationToken),
    /// <p>The account is not opted in to Compute Optimizer.</p>
    OptInRequiredException(crate::error::OptInRequiredException),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>The request has failed due to a temporary failure of the server.</p>
    ServiceUnavailableException(crate::error::ServiceUnavailableException),
    /// <p>The request was denied due to request throttling.</p>
    ThrottlingException(crate::error::ThrottlingException),
    /// An unhandled error occurred.
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::InvalidParameterValueException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::MissingAuthenticationToken(inner) => inner.fmt(f),
            Error::OptInRequiredException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceUnavailableException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::error::DescribeRecommendationExportJobsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::DescribeRecommendationExportJobsError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DescribeRecommendationExportJobsErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::DescribeRecommendationExportJobsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::DescribeRecommendationExportJobsErrorKind::InvalidParameterValueException(inner) => Error::InvalidParameterValueException(inner),
                crate::error::DescribeRecommendationExportJobsErrorKind::MissingAuthenticationToken(inner) => Error::MissingAuthenticationToken(inner),
                crate::error::DescribeRecommendationExportJobsErrorKind::OptInRequiredException(inner) => Error::OptInRequiredException(inner),
                crate::error::DescribeRecommendationExportJobsErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::DescribeRecommendationExportJobsErrorKind::ServiceUnavailableException(inner) => Error::ServiceUnavailableException(inner),
                crate::error::DescribeRecommendationExportJobsErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::DescribeRecommendationExportJobsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::error::ExportAutoScalingGroupRecommendationsError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::ExportAutoScalingGroupRecommendationsError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::ExportAutoScalingGroupRecommendationsErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::ExportAutoScalingGroupRecommendationsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::ExportAutoScalingGroupRecommendationsErrorKind::InvalidParameterValueException(inner) => Error::InvalidParameterValueException(inner),
                crate::error::ExportAutoScalingGroupRecommendationsErrorKind::LimitExceededException(inner) => Error::LimitExceededException(inner),
                crate::error::ExportAutoScalingGroupRecommendationsErrorKind::MissingAuthenticationToken(inner) => Error::MissingAuthenticationToken(inner),
                crate::error::ExportAutoScalingGroupRecommendationsErrorKind::OptInRequiredException(inner) => Error::OptInRequiredException(inner),
                crate::error::ExportAutoScalingGroupRecommendationsErrorKind::ServiceUnavailableException(inner) => Error::ServiceUnavailableException(inner),
                crate::error::ExportAutoScalingGroupRecommendationsErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::ExportAutoScalingGroupRecommendationsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::error::ExportEBSVolumeRecommendationsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::ExportEBSVolumeRecommendationsError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::ExportEBSVolumeRecommendationsErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::ExportEBSVolumeRecommendationsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::ExportEBSVolumeRecommendationsErrorKind::InvalidParameterValueException(inner) => Error::InvalidParameterValueException(inner),
                crate::error::ExportEBSVolumeRecommendationsErrorKind::LimitExceededException(inner) => Error::LimitExceededException(inner),
                crate::error::ExportEBSVolumeRecommendationsErrorKind::MissingAuthenticationToken(inner) => Error::MissingAuthenticationToken(inner),
                crate::error::ExportEBSVolumeRecommendationsErrorKind::OptInRequiredException(inner) => Error::OptInRequiredException(inner),
                crate::error::ExportEBSVolumeRecommendationsErrorKind::ServiceUnavailableException(inner) => Error::ServiceUnavailableException(inner),
                crate::error::ExportEBSVolumeRecommendationsErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::ExportEBSVolumeRecommendationsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::error::ExportEC2InstanceRecommendationsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::ExportEC2InstanceRecommendationsError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::ExportEC2InstanceRecommendationsErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::ExportEC2InstanceRecommendationsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::ExportEC2InstanceRecommendationsErrorKind::InvalidParameterValueException(inner) => Error::InvalidParameterValueException(inner),
                crate::error::ExportEC2InstanceRecommendationsErrorKind::LimitExceededException(inner) => Error::LimitExceededException(inner),
                crate::error::ExportEC2InstanceRecommendationsErrorKind::MissingAuthenticationToken(inner) => Error::MissingAuthenticationToken(inner),
                crate::error::ExportEC2InstanceRecommendationsErrorKind::OptInRequiredException(inner) => Error::OptInRequiredException(inner),
                crate::error::ExportEC2InstanceRecommendationsErrorKind::ServiceUnavailableException(inner) => Error::ServiceUnavailableException(inner),
                crate::error::ExportEC2InstanceRecommendationsErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::ExportEC2InstanceRecommendationsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::error::ExportLambdaFunctionRecommendationsError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::ExportLambdaFunctionRecommendationsError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::ExportLambdaFunctionRecommendationsErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::ExportLambdaFunctionRecommendationsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::ExportLambdaFunctionRecommendationsErrorKind::InvalidParameterValueException(inner) => Error::InvalidParameterValueException(inner),
                crate::error::ExportLambdaFunctionRecommendationsErrorKind::LimitExceededException(inner) => Error::LimitExceededException(inner),
                crate::error::ExportLambdaFunctionRecommendationsErrorKind::MissingAuthenticationToken(inner) => Error::MissingAuthenticationToken(inner),
                crate::error::ExportLambdaFunctionRecommendationsErrorKind::OptInRequiredException(inner) => Error::OptInRequiredException(inner),
                crate::error::ExportLambdaFunctionRecommendationsErrorKind::ServiceUnavailableException(inner) => Error::ServiceUnavailableException(inner),
                crate::error::ExportLambdaFunctionRecommendationsErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::ExportLambdaFunctionRecommendationsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<crate::error::GetAutoScalingGroupRecommendationsError, R>,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::GetAutoScalingGroupRecommendationsError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetAutoScalingGroupRecommendationsErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::GetAutoScalingGroupRecommendationsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::GetAutoScalingGroupRecommendationsErrorKind::InvalidParameterValueException(inner) => Error::InvalidParameterValueException(inner),
                crate::error::GetAutoScalingGroupRecommendationsErrorKind::MissingAuthenticationToken(inner) => Error::MissingAuthenticationToken(inner),
                crate::error::GetAutoScalingGroupRecommendationsErrorKind::OptInRequiredException(inner) => Error::OptInRequiredException(inner),
                crate::error::GetAutoScalingGroupRecommendationsErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::GetAutoScalingGroupRecommendationsErrorKind::ServiceUnavailableException(inner) => Error::ServiceUnavailableException(inner),
                crate::error::GetAutoScalingGroupRecommendationsErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::GetAutoScalingGroupRecommendationsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetEBSVolumeRecommendationsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetEBSVolumeRecommendationsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetEBSVolumeRecommendationsErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::GetEBSVolumeRecommendationsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::GetEBSVolumeRecommendationsErrorKind::InvalidParameterValueException(inner) => Error::InvalidParameterValueException(inner),
                crate::error::GetEBSVolumeRecommendationsErrorKind::MissingAuthenticationToken(inner) => Error::MissingAuthenticationToken(inner),
                crate::error::GetEBSVolumeRecommendationsErrorKind::OptInRequiredException(inner) => Error::OptInRequiredException(inner),
                crate::error::GetEBSVolumeRecommendationsErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::GetEBSVolumeRecommendationsErrorKind::ServiceUnavailableException(inner) => Error::ServiceUnavailableException(inner),
                crate::error::GetEBSVolumeRecommendationsErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::GetEBSVolumeRecommendationsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetEC2InstanceRecommendationsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetEC2InstanceRecommendationsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetEC2InstanceRecommendationsErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::GetEC2InstanceRecommendationsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::GetEC2InstanceRecommendationsErrorKind::InvalidParameterValueException(inner) => Error::InvalidParameterValueException(inner),
                crate::error::GetEC2InstanceRecommendationsErrorKind::MissingAuthenticationToken(inner) => Error::MissingAuthenticationToken(inner),
                crate::error::GetEC2InstanceRecommendationsErrorKind::OptInRequiredException(inner) => Error::OptInRequiredException(inner),
                crate::error::GetEC2InstanceRecommendationsErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::GetEC2InstanceRecommendationsErrorKind::ServiceUnavailableException(inner) => Error::ServiceUnavailableException(inner),
                crate::error::GetEC2InstanceRecommendationsErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::GetEC2InstanceRecommendationsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::error::GetEC2RecommendationProjectedMetricsError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::GetEC2RecommendationProjectedMetricsError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetEC2RecommendationProjectedMetricsErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::GetEC2RecommendationProjectedMetricsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::GetEC2RecommendationProjectedMetricsErrorKind::InvalidParameterValueException(inner) => Error::InvalidParameterValueException(inner),
                crate::error::GetEC2RecommendationProjectedMetricsErrorKind::MissingAuthenticationToken(inner) => Error::MissingAuthenticationToken(inner),
                crate::error::GetEC2RecommendationProjectedMetricsErrorKind::OptInRequiredException(inner) => Error::OptInRequiredException(inner),
                crate::error::GetEC2RecommendationProjectedMetricsErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::GetEC2RecommendationProjectedMetricsErrorKind::ServiceUnavailableException(inner) => Error::ServiceUnavailableException(inner),
                crate::error::GetEC2RecommendationProjectedMetricsErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::GetEC2RecommendationProjectedMetricsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetEnrollmentStatusError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetEnrollmentStatusError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetEnrollmentStatusErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::GetEnrollmentStatusErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::GetEnrollmentStatusErrorKind::InvalidParameterValueException(
                    inner,
                ) => Error::InvalidParameterValueException(inner),
                crate::error::GetEnrollmentStatusErrorKind::MissingAuthenticationToken(inner) => {
                    Error::MissingAuthenticationToken(inner)
                }
                crate::error::GetEnrollmentStatusErrorKind::ServiceUnavailableException(inner) => {
                    Error::ServiceUnavailableException(inner)
                }
                crate::error::GetEnrollmentStatusErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::GetEnrollmentStatusErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::error::GetEnrollmentStatusesForOrganizationError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::GetEnrollmentStatusesForOrganizationError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetEnrollmentStatusesForOrganizationErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::GetEnrollmentStatusesForOrganizationErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::GetEnrollmentStatusesForOrganizationErrorKind::InvalidParameterValueException(inner) => Error::InvalidParameterValueException(inner),
                crate::error::GetEnrollmentStatusesForOrganizationErrorKind::MissingAuthenticationToken(inner) => Error::MissingAuthenticationToken(inner),
                crate::error::GetEnrollmentStatusesForOrganizationErrorKind::ServiceUnavailableException(inner) => Error::ServiceUnavailableException(inner),
                crate::error::GetEnrollmentStatusesForOrganizationErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::GetEnrollmentStatusesForOrganizationErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::error::GetLambdaFunctionRecommendationsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::GetLambdaFunctionRecommendationsError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetLambdaFunctionRecommendationsErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::GetLambdaFunctionRecommendationsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::GetLambdaFunctionRecommendationsErrorKind::InvalidParameterValueException(inner) => Error::InvalidParameterValueException(inner),
                crate::error::GetLambdaFunctionRecommendationsErrorKind::LimitExceededException(inner) => Error::LimitExceededException(inner),
                crate::error::GetLambdaFunctionRecommendationsErrorKind::MissingAuthenticationToken(inner) => Error::MissingAuthenticationToken(inner),
                crate::error::GetLambdaFunctionRecommendationsErrorKind::OptInRequiredException(inner) => Error::OptInRequiredException(inner),
                crate::error::GetLambdaFunctionRecommendationsErrorKind::ServiceUnavailableException(inner) => Error::ServiceUnavailableException(inner),
                crate::error::GetLambdaFunctionRecommendationsErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::GetLambdaFunctionRecommendationsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetRecommendationSummariesError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetRecommendationSummariesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetRecommendationSummariesErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::GetRecommendationSummariesErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::GetRecommendationSummariesErrorKind::InvalidParameterValueException(inner) => Error::InvalidParameterValueException(inner),
                crate::error::GetRecommendationSummariesErrorKind::MissingAuthenticationToken(inner) => Error::MissingAuthenticationToken(inner),
                crate::error::GetRecommendationSummariesErrorKind::OptInRequiredException(inner) => Error::OptInRequiredException(inner),
                crate::error::GetRecommendationSummariesErrorKind::ServiceUnavailableException(inner) => Error::ServiceUnavailableException(inner),
                crate::error::GetRecommendationSummariesErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::GetRecommendationSummariesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateEnrollmentStatusError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdateEnrollmentStatusError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateEnrollmentStatusErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::UpdateEnrollmentStatusErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::UpdateEnrollmentStatusErrorKind::InvalidParameterValueException(
                    inner,
                ) => Error::InvalidParameterValueException(inner),
                crate::error::UpdateEnrollmentStatusErrorKind::MissingAuthenticationToken(
                    inner,
                ) => Error::MissingAuthenticationToken(inner),
                crate::error::UpdateEnrollmentStatusErrorKind::ServiceUnavailableException(
                    inner,
                ) => Error::ServiceUnavailableException(inner),
                crate::error::UpdateEnrollmentStatusErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::UpdateEnrollmentStatusErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
