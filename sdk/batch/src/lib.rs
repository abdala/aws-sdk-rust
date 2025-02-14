#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <fullname>Batch</fullname>
//! <p>Using Batch, you can run batch computing workloads on the Amazon Web Services Cloud. Batch computing is a common means for
//! developers, scientists, and engineers to access large amounts of compute resources. Batch uses the advantages of
//! this computing workload to remove the undifferentiated heavy lifting of configuring and managing required
//! infrastructure. At the same time, it also adopts a familiar batch computing software approach. Given these
//! advantages, Batch can help you to efficiently provision resources in response to jobs submitted, thus effectively
//! helping you to eliminate capacity constraints, reduce compute costs, and deliver your results more quickly.</p>
//! <p>As a fully managed service, Batch can run batch computing workloads of any scale. Batch automatically
//! provisions compute resources and optimizes workload distribution based on the quantity and scale of your specific
//! workloads. With Batch, there's no need to install or manage batch computing software. This means that you can focus
//! your time and energy on analyzing results and solving your specific problems.</p>

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

pub use config::Config;

mod aws_endpoint;
/// Client and fluent builders for calling the service.
#[cfg(feature = "client")]
pub mod client;
/// Configuration for the service.
pub mod config;
/// Errors that can occur when calling the service.
pub mod error;
mod error_meta;
/// Input structures for operations.
pub mod input;
mod json_deser;
mod json_errors;
mod json_ser;
/// Data structures used by operation inputs/outputs.
pub mod model;
mod no_credentials;
/// All operations that this crate can perform.
pub mod operation;
mod operation_deser;
mod operation_ser;
/// Output structures for operations.
pub mod output;
/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use aws_smithy_http::byte_stream::ByteStream;
pub use aws_smithy_http::result::SdkError;
pub use aws_smithy_types::Blob;
pub use aws_smithy_types::DateTime;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("batch", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_smithy_types::retry::RetryConfig;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[cfg(feature = "client")]
pub use client::Client;
