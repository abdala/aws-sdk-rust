// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that
    /// doesn't have permissions to use the action or resource, or specifying an identifier that's not valid.</p>
    ClientException(crate::error::ClientException),
    /// <p>These errors are usually caused by a server issue.</p>
    ServerException(crate::error::ServerException),
    /// An unhandled error occurred.
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ClientException(inner) => inner.fmt(f),
            Error::ServerException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CancelJobError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CancelJobError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CancelJobErrorKind::ClientException(inner) => {
                    Error::ClientException(inner)
                }
                crate::error::CancelJobErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::CancelJobErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateComputeEnvironmentError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CreateComputeEnvironmentError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateComputeEnvironmentErrorKind::ClientException(inner) => {
                    Error::ClientException(inner)
                }
                crate::error::CreateComputeEnvironmentErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::CreateComputeEnvironmentErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateJobQueueError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateJobQueueError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateJobQueueErrorKind::ClientException(inner) => {
                    Error::ClientException(inner)
                }
                crate::error::CreateJobQueueErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::CreateJobQueueErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateSchedulingPolicyError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CreateSchedulingPolicyError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateSchedulingPolicyErrorKind::ClientException(inner) => {
                    Error::ClientException(inner)
                }
                crate::error::CreateSchedulingPolicyErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::CreateSchedulingPolicyErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteComputeEnvironmentError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteComputeEnvironmentError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteComputeEnvironmentErrorKind::ClientException(inner) => {
                    Error::ClientException(inner)
                }
                crate::error::DeleteComputeEnvironmentErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::DeleteComputeEnvironmentErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteJobQueueError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteJobQueueError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteJobQueueErrorKind::ClientException(inner) => {
                    Error::ClientException(inner)
                }
                crate::error::DeleteJobQueueErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::DeleteJobQueueErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteSchedulingPolicyError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteSchedulingPolicyError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteSchedulingPolicyErrorKind::ClientException(inner) => {
                    Error::ClientException(inner)
                }
                crate::error::DeleteSchedulingPolicyErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::DeleteSchedulingPolicyErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeregisterJobDefinitionError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeregisterJobDefinitionError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeregisterJobDefinitionErrorKind::ClientException(inner) => {
                    Error::ClientException(inner)
                }
                crate::error::DeregisterJobDefinitionErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::DeregisterJobDefinitionErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeComputeEnvironmentsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeComputeEnvironmentsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeComputeEnvironmentsErrorKind::ClientException(inner) => {
                    Error::ClientException(inner)
                }
                crate::error::DescribeComputeEnvironmentsErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::DescribeComputeEnvironmentsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeJobDefinitionsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeJobDefinitionsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeJobDefinitionsErrorKind::ClientException(inner) => {
                    Error::ClientException(inner)
                }
                crate::error::DescribeJobDefinitionsErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::DescribeJobDefinitionsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeJobQueuesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeJobQueuesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeJobQueuesErrorKind::ClientException(inner) => {
                    Error::ClientException(inner)
                }
                crate::error::DescribeJobQueuesErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::DescribeJobQueuesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeJobsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeJobsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeJobsErrorKind::ClientException(inner) => {
                    Error::ClientException(inner)
                }
                crate::error::DescribeJobsErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::DescribeJobsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeSchedulingPoliciesError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeSchedulingPoliciesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeSchedulingPoliciesErrorKind::ClientException(inner) => {
                    Error::ClientException(inner)
                }
                crate::error::DescribeSchedulingPoliciesErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::DescribeSchedulingPoliciesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListJobsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListJobsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListJobsErrorKind::ClientException(inner) => {
                    Error::ClientException(inner)
                }
                crate::error::ListJobsErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::ListJobsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListSchedulingPoliciesError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListSchedulingPoliciesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListSchedulingPoliciesErrorKind::ClientException(inner) => {
                    Error::ClientException(inner)
                }
                crate::error::ListSchedulingPoliciesErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::ListSchedulingPoliciesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListTagsForResourceErrorKind::ClientException(inner) => {
                    Error::ClientException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::RegisterJobDefinitionError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::RegisterJobDefinitionError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::RegisterJobDefinitionErrorKind::ClientException(inner) => {
                    Error::ClientException(inner)
                }
                crate::error::RegisterJobDefinitionErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::RegisterJobDefinitionErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::SubmitJobError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::SubmitJobError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::SubmitJobErrorKind::ClientException(inner) => {
                    Error::ClientException(inner)
                }
                crate::error::SubmitJobErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::SubmitJobErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::TagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::TagResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::TagResourceErrorKind::ClientException(inner) => {
                    Error::ClientException(inner)
                }
                crate::error::TagResourceErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::TagResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::TerminateJobError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::TerminateJobError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::TerminateJobErrorKind::ClientException(inner) => {
                    Error::ClientException(inner)
                }
                crate::error::TerminateJobErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::TerminateJobErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UntagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UntagResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UntagResourceErrorKind::ClientException(inner) => {
                    Error::ClientException(inner)
                }
                crate::error::UntagResourceErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::UntagResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateComputeEnvironmentError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdateComputeEnvironmentError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateComputeEnvironmentErrorKind::ClientException(inner) => {
                    Error::ClientException(inner)
                }
                crate::error::UpdateComputeEnvironmentErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::UpdateComputeEnvironmentErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateJobQueueError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateJobQueueError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateJobQueueErrorKind::ClientException(inner) => {
                    Error::ClientException(inner)
                }
                crate::error::UpdateJobQueueErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::UpdateJobQueueErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateSchedulingPolicyError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdateSchedulingPolicyError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateSchedulingPolicyErrorKind::ClientException(inner) => {
                    Error::ClientException(inner)
                }
                crate::error::UpdateSchedulingPolicyErrorKind::ServerException(inner) => {
                    Error::ServerException(inner)
                }
                crate::error::UpdateSchedulingPolicyErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
