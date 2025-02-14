// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle<
    C = aws_smithy_client::erase::DynConnector,
    M = aws_hyper::AwsMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    client: aws_smithy_client::Client<C, M, R>,
    conf: crate::Config,
}

/// An ergonomic service client for `PinpointSMSVoice`.
///
/// This client allows ergonomic access to a `PinpointSMSVoice`-shaped service.
/// Each method corresponds to an endpoint defined in the service's Smithy model,
/// and the request and response shapes are auto-generated from that same model.
///
/// # Using a Client
///
/// Once you have a client set up, you can access the service's endpoints
/// by calling the appropriate method on [`Client`]. Each such method
/// returns a request builder for that endpoint, with methods for setting
/// the various fields of the request. Once your request is complete, use
/// the `send` method to send the request. `send` returns a future, which
/// you then have to `.await` to get the service's response.
///
/// [builder pattern]: https://rust-lang.github.io/api-guidelines/type-safety.html#c-builder
/// [SigV4-signed requests]: https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html
#[derive(std::fmt::Debug)]
pub struct Client<
    C = aws_smithy_client::erase::DynConnector,
    M = aws_hyper::AwsMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<Handle<C, M, R>>,
}

impl<C, M, R> std::clone::Clone for Client<C, M, R> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

#[doc(inline)]
pub use aws_smithy_client::Builder;

impl<C, M, R> From<aws_smithy_client::Client<C, M, R>> for Client<C, M, R> {
    fn from(client: aws_smithy_client::Client<C, M, R>) -> Self {
        Self::with_config(client, crate::Config::builder().build())
    }
}

impl<C, M, R> Client<C, M, R> {
    /// Creates a client with the given service configuration.
    pub fn with_config(client: aws_smithy_client::Client<C, M, R>, conf: crate::Config) -> Self {
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    /// Returns the client's configuration.
    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl<C, M, R> Client<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Constructs a fluent builder for the `CreateConfigurationSet` operation.
    ///
    /// See [`CreateConfigurationSet`](crate::client::fluent_builders::CreateConfigurationSet) for more information about the
    /// operation and its arguments.
    pub fn create_configuration_set(&self) -> fluent_builders::CreateConfigurationSet<C, M, R> {
        fluent_builders::CreateConfigurationSet::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `CreateConfigurationSetEventDestination` operation.
    ///
    /// See [`CreateConfigurationSetEventDestination`](crate::client::fluent_builders::CreateConfigurationSetEventDestination) for more information about the
    /// operation and its arguments.
    pub fn create_configuration_set_event_destination(
        &self,
    ) -> fluent_builders::CreateConfigurationSetEventDestination<C, M, R> {
        fluent_builders::CreateConfigurationSetEventDestination::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `DeleteConfigurationSet` operation.
    ///
    /// See [`DeleteConfigurationSet`](crate::client::fluent_builders::DeleteConfigurationSet) for more information about the
    /// operation and its arguments.
    pub fn delete_configuration_set(&self) -> fluent_builders::DeleteConfigurationSet<C, M, R> {
        fluent_builders::DeleteConfigurationSet::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `DeleteConfigurationSetEventDestination` operation.
    ///
    /// See [`DeleteConfigurationSetEventDestination`](crate::client::fluent_builders::DeleteConfigurationSetEventDestination) for more information about the
    /// operation and its arguments.
    pub fn delete_configuration_set_event_destination(
        &self,
    ) -> fluent_builders::DeleteConfigurationSetEventDestination<C, M, R> {
        fluent_builders::DeleteConfigurationSetEventDestination::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `GetConfigurationSetEventDestinations` operation.
    ///
    /// See [`GetConfigurationSetEventDestinations`](crate::client::fluent_builders::GetConfigurationSetEventDestinations) for more information about the
    /// operation and its arguments.
    pub fn get_configuration_set_event_destinations(
        &self,
    ) -> fluent_builders::GetConfigurationSetEventDestinations<C, M, R> {
        fluent_builders::GetConfigurationSetEventDestinations::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `ListConfigurationSets` operation.
    ///
    /// See [`ListConfigurationSets`](crate::client::fluent_builders::ListConfigurationSets) for more information about the
    /// operation and its arguments.
    pub fn list_configuration_sets(&self) -> fluent_builders::ListConfigurationSets<C, M, R> {
        fluent_builders::ListConfigurationSets::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `SendVoiceMessage` operation.
    ///
    /// See [`SendVoiceMessage`](crate::client::fluent_builders::SendVoiceMessage) for more information about the
    /// operation and its arguments.
    pub fn send_voice_message(&self) -> fluent_builders::SendVoiceMessage<C, M, R> {
        fluent_builders::SendVoiceMessage::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `UpdateConfigurationSetEventDestination` operation.
    ///
    /// See [`UpdateConfigurationSetEventDestination`](crate::client::fluent_builders::UpdateConfigurationSetEventDestination) for more information about the
    /// operation and its arguments.
    pub fn update_configuration_set_event_destination(
        &self,
    ) -> fluent_builders::UpdateConfigurationSetEventDestination<C, M, R> {
        fluent_builders::UpdateConfigurationSetEventDestination::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    //!
    //! Utilities to ergonomically construct a request to the service.
    //!
    //! Fluent builders are created through the [`Client`](crate::client::Client) by calling
    //! one if its operation methods. After parameters are set using the builder methods,
    //! the `send` method can be called to initiate the request.
    //!
    /// Fluent builder constructing a request to `CreateConfigurationSet`.
    ///
    /// Create a new configuration set. After you create the configuration set, you can add one or more event destinations to it.
    #[derive(std::fmt::Debug)]
    pub struct CreateConfigurationSet<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::create_configuration_set_input::Builder,
    }
    impl<C, M, R> CreateConfigurationSet<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `CreateConfigurationSet`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::CreateConfigurationSetOutput,
            aws_smithy_http::result::SdkError<crate::error::CreateConfigurationSetError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::CreateConfigurationSetInputOperationOutputAlias,
                crate::output::CreateConfigurationSetOutput,
                crate::error::CreateConfigurationSetError,
                crate::input::CreateConfigurationSetInputOperationRetryAlias,
            >,
        {
            let input = self.inner.build().map_err(|err| {
                aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
            })?;
            let op = input
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// The name that you want to give the configuration set.
        pub fn configuration_set_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.configuration_set_name(inp);
            self
        }
        /// The name that you want to give the configuration set.
        pub fn set_configuration_set_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_configuration_set_name(input);
            self
        }
    }
    /// Fluent builder constructing a request to `CreateConfigurationSetEventDestination`.
    ///
    /// Create a new event destination in a configuration set.
    #[derive(std::fmt::Debug)]
    pub struct CreateConfigurationSetEventDestination<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::create_configuration_set_event_destination_input::Builder,
    }
    impl<C, M, R> CreateConfigurationSetEventDestination<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `CreateConfigurationSetEventDestination`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::CreateConfigurationSetEventDestinationOutput,
            aws_smithy_http::result::SdkError<
                crate::error::CreateConfigurationSetEventDestinationError,
            >,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::CreateConfigurationSetEventDestinationInputOperationOutputAlias,
                crate::output::CreateConfigurationSetEventDestinationOutput,
                crate::error::CreateConfigurationSetEventDestinationError,
                crate::input::CreateConfigurationSetEventDestinationInputOperationRetryAlias,
            >,
        {
            let input = self.inner.build().map_err(|err| {
                aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
            })?;
            let op = input
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// ConfigurationSetName
        pub fn configuration_set_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.configuration_set_name(inp);
            self
        }
        /// ConfigurationSetName
        pub fn set_configuration_set_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_configuration_set_name(input);
            self
        }
        /// An object that defines a single event destination.
        pub fn event_destination(mut self, inp: crate::model::EventDestinationDefinition) -> Self {
            self.inner = self.inner.event_destination(inp);
            self
        }
        /// An object that defines a single event destination.
        pub fn set_event_destination(
            mut self,
            input: std::option::Option<crate::model::EventDestinationDefinition>,
        ) -> Self {
            self.inner = self.inner.set_event_destination(input);
            self
        }
        /// A name that identifies the event destination.
        pub fn event_destination_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.event_destination_name(inp);
            self
        }
        /// A name that identifies the event destination.
        pub fn set_event_destination_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_event_destination_name(input);
            self
        }
    }
    /// Fluent builder constructing a request to `DeleteConfigurationSet`.
    ///
    /// Deletes an existing configuration set.
    #[derive(std::fmt::Debug)]
    pub struct DeleteConfigurationSet<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::delete_configuration_set_input::Builder,
    }
    impl<C, M, R> DeleteConfigurationSet<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `DeleteConfigurationSet`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::DeleteConfigurationSetOutput,
            aws_smithy_http::result::SdkError<crate::error::DeleteConfigurationSetError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::DeleteConfigurationSetInputOperationOutputAlias,
                crate::output::DeleteConfigurationSetOutput,
                crate::error::DeleteConfigurationSetError,
                crate::input::DeleteConfigurationSetInputOperationRetryAlias,
            >,
        {
            let input = self.inner.build().map_err(|err| {
                aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
            })?;
            let op = input
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// ConfigurationSetName
        pub fn configuration_set_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.configuration_set_name(inp);
            self
        }
        /// ConfigurationSetName
        pub fn set_configuration_set_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_configuration_set_name(input);
            self
        }
    }
    /// Fluent builder constructing a request to `DeleteConfigurationSetEventDestination`.
    ///
    /// Deletes an event destination in a configuration set.
    #[derive(std::fmt::Debug)]
    pub struct DeleteConfigurationSetEventDestination<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::delete_configuration_set_event_destination_input::Builder,
    }
    impl<C, M, R> DeleteConfigurationSetEventDestination<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `DeleteConfigurationSetEventDestination`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::DeleteConfigurationSetEventDestinationOutput,
            aws_smithy_http::result::SdkError<
                crate::error::DeleteConfigurationSetEventDestinationError,
            >,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::DeleteConfigurationSetEventDestinationInputOperationOutputAlias,
                crate::output::DeleteConfigurationSetEventDestinationOutput,
                crate::error::DeleteConfigurationSetEventDestinationError,
                crate::input::DeleteConfigurationSetEventDestinationInputOperationRetryAlias,
            >,
        {
            let input = self.inner.build().map_err(|err| {
                aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
            })?;
            let op = input
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// ConfigurationSetName
        pub fn configuration_set_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.configuration_set_name(inp);
            self
        }
        /// ConfigurationSetName
        pub fn set_configuration_set_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_configuration_set_name(input);
            self
        }
        /// EventDestinationName
        pub fn event_destination_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.event_destination_name(inp);
            self
        }
        /// EventDestinationName
        pub fn set_event_destination_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_event_destination_name(input);
            self
        }
    }
    /// Fluent builder constructing a request to `GetConfigurationSetEventDestinations`.
    ///
    /// Obtain information about an event destination, including the types of events it reports, the Amazon Resource Name (ARN) of the destination, and the name of the event destination.
    #[derive(std::fmt::Debug)]
    pub struct GetConfigurationSetEventDestinations<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::get_configuration_set_event_destinations_input::Builder,
    }
    impl<C, M, R> GetConfigurationSetEventDestinations<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `GetConfigurationSetEventDestinations`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::GetConfigurationSetEventDestinationsOutput,
            aws_smithy_http::result::SdkError<
                crate::error::GetConfigurationSetEventDestinationsError,
            >,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::GetConfigurationSetEventDestinationsInputOperationOutputAlias,
                crate::output::GetConfigurationSetEventDestinationsOutput,
                crate::error::GetConfigurationSetEventDestinationsError,
                crate::input::GetConfigurationSetEventDestinationsInputOperationRetryAlias,
            >,
        {
            let input = self.inner.build().map_err(|err| {
                aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
            })?;
            let op = input
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// ConfigurationSetName
        pub fn configuration_set_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.configuration_set_name(inp);
            self
        }
        /// ConfigurationSetName
        pub fn set_configuration_set_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_configuration_set_name(input);
            self
        }
    }
    /// Fluent builder constructing a request to `ListConfigurationSets`.
    ///
    /// List all of the configuration sets associated with your Amazon Pinpoint account in the current region.
    #[derive(std::fmt::Debug)]
    pub struct ListConfigurationSets<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::list_configuration_sets_input::Builder,
    }
    impl<C, M, R> ListConfigurationSets<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `ListConfigurationSets`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ListConfigurationSetsOutput,
            aws_smithy_http::result::SdkError<crate::error::ListConfigurationSetsError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ListConfigurationSetsInputOperationOutputAlias,
                crate::output::ListConfigurationSetsOutput,
                crate::error::ListConfigurationSetsError,
                crate::input::ListConfigurationSetsInputOperationRetryAlias,
            >,
        {
            let input = self.inner.build().map_err(|err| {
                aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
            })?;
            let op = input
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// A token returned from a previous call to the API that indicates the position in the list of results.
        pub fn next_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(inp);
            self
        }
        /// A token returned from a previous call to the API that indicates the position in the list of results.
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// Used to specify the number of items that should be returned in the response.
        pub fn page_size(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.page_size(inp);
            self
        }
        /// Used to specify the number of items that should be returned in the response.
        pub fn set_page_size(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_page_size(input);
            self
        }
    }
    /// Fluent builder constructing a request to `SendVoiceMessage`.
    ///
    /// Create a new voice message and send it to a recipient's phone number.
    #[derive(std::fmt::Debug)]
    pub struct SendVoiceMessage<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::send_voice_message_input::Builder,
    }
    impl<C, M, R> SendVoiceMessage<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `SendVoiceMessage`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::SendVoiceMessageOutput,
            aws_smithy_http::result::SdkError<crate::error::SendVoiceMessageError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::SendVoiceMessageInputOperationOutputAlias,
                crate::output::SendVoiceMessageOutput,
                crate::error::SendVoiceMessageError,
                crate::input::SendVoiceMessageInputOperationRetryAlias,
            >,
        {
            let input = self.inner.build().map_err(|err| {
                aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
            })?;
            let op = input
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// The phone number that appears on recipients' devices when they receive the message.
        pub fn caller_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.caller_id(inp);
            self
        }
        /// The phone number that appears on recipients' devices when they receive the message.
        pub fn set_caller_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_caller_id(input);
            self
        }
        /// The name of the configuration set that you want to use to send the message.
        pub fn configuration_set_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.configuration_set_name(inp);
            self
        }
        /// The name of the configuration set that you want to use to send the message.
        pub fn set_configuration_set_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_configuration_set_name(input);
            self
        }
        /// An object that contains a voice message and information about the recipient that you want to send it to.
        pub fn content(mut self, inp: crate::model::VoiceMessageContent) -> Self {
            self.inner = self.inner.content(inp);
            self
        }
        /// An object that contains a voice message and information about the recipient that you want to send it to.
        pub fn set_content(
            mut self,
            input: std::option::Option<crate::model::VoiceMessageContent>,
        ) -> Self {
            self.inner = self.inner.set_content(input);
            self
        }
        /// The phone number that you want to send the voice message to.
        pub fn destination_phone_number(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.destination_phone_number(inp);
            self
        }
        /// The phone number that you want to send the voice message to.
        pub fn set_destination_phone_number(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_destination_phone_number(input);
            self
        }
        /// The phone number that Amazon Pinpoint should use to send the voice message. This isn't necessarily the phone number that appears on recipients' devices when they receive the message, because you can specify a CallerId parameter in the request.
        pub fn origination_phone_number(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.origination_phone_number(inp);
            self
        }
        /// The phone number that Amazon Pinpoint should use to send the voice message. This isn't necessarily the phone number that appears on recipients' devices when they receive the message, because you can specify a CallerId parameter in the request.
        pub fn set_origination_phone_number(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_origination_phone_number(input);
            self
        }
    }
    /// Fluent builder constructing a request to `UpdateConfigurationSetEventDestination`.
    ///
    /// Update an event destination in a configuration set. An event destination is a location that you publish information about your voice calls to. For example, you can log an event to an Amazon CloudWatch destination when a call fails.
    #[derive(std::fmt::Debug)]
    pub struct UpdateConfigurationSetEventDestination<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::update_configuration_set_event_destination_input::Builder,
    }
    impl<C, M, R> UpdateConfigurationSetEventDestination<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `UpdateConfigurationSetEventDestination`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::UpdateConfigurationSetEventDestinationOutput,
            aws_smithy_http::result::SdkError<
                crate::error::UpdateConfigurationSetEventDestinationError,
            >,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::UpdateConfigurationSetEventDestinationInputOperationOutputAlias,
                crate::output::UpdateConfigurationSetEventDestinationOutput,
                crate::error::UpdateConfigurationSetEventDestinationError,
                crate::input::UpdateConfigurationSetEventDestinationInputOperationRetryAlias,
            >,
        {
            let input = self.inner.build().map_err(|err| {
                aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
            })?;
            let op = input
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// ConfigurationSetName
        pub fn configuration_set_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.configuration_set_name(inp);
            self
        }
        /// ConfigurationSetName
        pub fn set_configuration_set_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_configuration_set_name(input);
            self
        }
        /// An object that defines a single event destination.
        pub fn event_destination(mut self, inp: crate::model::EventDestinationDefinition) -> Self {
            self.inner = self.inner.event_destination(inp);
            self
        }
        /// An object that defines a single event destination.
        pub fn set_event_destination(
            mut self,
            input: std::option::Option<crate::model::EventDestinationDefinition>,
        ) -> Self {
            self.inner = self.inner.set_event_destination(input);
            self
        }
        /// EventDestinationName
        pub fn event_destination_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.event_destination_name(inp);
            self
        }
        /// EventDestinationName
        pub fn set_event_destination_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_event_destination_name(input);
            self
        }
    }
}
impl<C> Client<C, aws_hyper::AwsMiddleware, aws_smithy_client::retry::Standard> {
    /// Creates a client with the given service config and connector override.
    pub fn from_conf_conn(conf: crate::Config, conn: C) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let timeout_config = conf.timeout_config.as_ref().cloned().unwrap_or_default();
        let sleep_impl = conf.sleep_impl.clone();
        let mut client = aws_hyper::Client::new(conn)
            .with_retry_config(retry_config.into())
            .with_timeout_config(timeout_config);

        client.set_sleep_impl(sleep_impl);
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
impl
    Client<
        aws_smithy_client::erase::DynConnector,
        aws_hyper::AwsMiddleware,
        aws_smithy_client::retry::Standard,
    >
{
    /// Creates a new client from a shared config.
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn new(config: &aws_types::config::Config) -> Self {
        Self::from_conf(config.into())
    }

    /// Creates a new client from the service [`Config`](crate::Config).
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let timeout_config = conf.timeout_config.as_ref().cloned().unwrap_or_default();
        let sleep_impl = conf.sleep_impl.clone();
        let mut client = aws_hyper::Client::https()
            .with_retry_config(retry_config.into())
            .with_timeout_config(timeout_config);

        client.set_sleep_impl(sleep_impl);
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
