# The AWS SDK for Rust [![Docs](https://img.shields.io/badge/docs-v0.0.26--alpha-blue)](https://awslabs.github.io/aws-sdk-rust/) ![MSRV](https://img.shields.io/badge/msrv-1.54-red)

This repo contains the new AWS SDK for Rust (the SDK) and its [public roadmap](https://github.com/awslabs/aws-sdk-rust/projects/1)

**Please Note: The SDK is currently released as an alpha and is intended strictly for feedback purposes only. Do not use this SDK for production workloads.**

The SDK is code generated from [Smithy models](https://awslabs.github.io/smithy/) that represent each AWS service. The code used to generate the SDK can be found in [smithy-rs](https://github.com/awslabs/smithy-rs).

## Getting Started with the SDK

> Examples are availble for many services and operations, check out the [examples folder](https://github.com/awslabs/aws-sdk-rust/tree/main/examples).

The SDK provides one crate per AWS service. You must add [Tokio](https://crates.io/crates/tokio) as a dependency within your Rust project to execute asynchronous code.

1. Create a new Rust project: `cargo new sdk-example`
2. Add dependencies to DynamoDB and Tokio to your **Cargo.toml** file:

    ```toml
    [dependencies]
    aws-config = "0.0.26-alpha"
    aws-sdk-dynamodb = "0.0.26-alpha"
    tokio = { version = "1", features = ["full"] }
    ```

3. Provide your AWS credentials with the default credential provider chain, which currently looks in:
   - Environment variables: `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, and `AWS_REGION`
   - Web Identity Token credentials from the environment or container (including EKS)
   - The default credentials files located in `~/.aws/config` and `~/.aws/credentials` (location can vary per platform)
   - ECS Container Credentials (IAM roles for tasks)
   - EC2 Instance Metadata Service (IAM Roles attached to instance)
**Note:** SSO is not supported yet.

4. Make a request using DynamoDB

```rust
use aws_sdk_dynamodb::{Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let req = client.list_tables().limit(10);
    let resp = req.send().await?;
    println!("Current DynamoDB tables: {:?}", resp.table_names);
    Ok(())
}
```

### Prerequisites

In order to use the SDK, you must already have Rust and Cargo installed. If you don't, [these instructions](https://doc.rust-lang.org/book/ch01-01-installation.html) describe how to install Rust and Cargo.

## Using the SDK

Until the SDK is released, we will be adding information about using the SDK to the [Guide](https://github.com/awslabs/aws-sdk-rust/blob/main/Guide.md). Feel free to suggest additional sections for the guide by opening an issue and describing what you are trying to do. 

## Getting Help

* [GitHub discussions](https://github.com/awslabs/aws-sdk-rust/discussions) - For ideas, RFCs & general questions
* [GitHub issues](https://github.com/awslabs/aws-sdk-rust/issues/new/choose) – For bug reports & feature requests
* [Generated Docs (latest version)](https://awslabs.github.io/aws-sdk-rust/)
* [Usage examples](https://github.com/awslabs/aws-sdk-rust/tree/main/examples)

## Feedback and Contributing

### Feedback 

The SDK uses **GitHub Issues** to track feature requests and issues with the SDK. In addition, we use **GitHub Projects** to provide users with a high level view of our roadmap and the features we're actively working on. 

You can provide feedback or report a bug  by submitting a **GitHub issue**. This is the preferred mechanism to give feedback so that other users can engage in the conversation, +1 issues, etc. Issues you open will be evaluated for our roadmap in the Developer Preview launch.

### Contributing

If you are interested in contributing to the SDK, please take a look at [CONTRIBUTING](CONTRIBUTING.md)

## Supported Rust Versions (MSRV)

The SDK currently requires a minimum of Rust 1.53, and is not guaranteed to build on compiler versions earlier than that. While we are still in alpha, we will be keeping the minimum compiler version two releases behind the latest stable release where possible (so if the latest stable is 1.55, we will be on 1.53). However, we are not making any guarantees around this at present. Increases in minimum required Rust version will be called out in the Release Notes for new releases of the SDK.

## Additional Resources

- Design docs - Design documentation for the SDK lives in the [design folder of smithy-rs](https://github.com/awslabs/smithy-rs/tree/main/design).
- Runtime / Handwritten code: The Rust Runtime code that underpins the SDK can be accessed [here](https://github.com/awslabs/smithy-rs/tree/main/rust-runtime) and [here](https://github.com/awslabs/smithy-rs/tree/main/aws/rust-runtime). This code is copied into this repo as part of code generation.
- [Code Examples](https://github.com/awslabs/aws-sdk-rust/tree/main/examples)
- [API reference documentation (rustdoc)](https://awslabs.github.io/aws-sdk-rust/)
## Security

See [CONTRIBUTING](CONTRIBUTING.md#security-issue-notifications) for more information.

## License

This project is licensed under the Apache-2.0 License.
