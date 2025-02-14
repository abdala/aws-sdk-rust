/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

use aws_sdk_s3::model::{
    CompressionType, CsvInput, CsvOutput, ExpressionType, FileHeaderInfo, InputSerialization,
    OutputSerialization, SelectObjectContentEventStream,
};
use aws_sdk_s3::{Client, Config, Credentials, Region};
use aws_smithy_client::dvr::{Event, ReplayingConnection};
use std::error::Error as StdError;

#[tokio::test]
async fn test_success() {
    let events: Vec<Event> =
        serde_json::from_str(include_str!("select-object-content.json")).unwrap();
    let replayer = ReplayingConnection::new(events);

    let region = Region::from_static("us-east-2");
    let credentials = Credentials::new("test", "test", None, None, "test");
    let config = Config::builder()
        .region(region)
        .credentials_provider(credentials)
        .build();
    let client = Client::from_conf_conn(config, replayer.clone());

    let mut output = client
        .select_object_content()
        .bucket("aws-rust-sdk")
        .key("sample_data.csv")
        .expression_type(ExpressionType::Sql)
        .expression("SELECT * FROM s3object s WHERE s.\"Name\" = 'Jane'")
        .input_serialization(
            InputSerialization::builder()
                .csv(
                    CsvInput::builder()
                        .file_header_info(FileHeaderInfo::Use)
                        .build(),
                )
                .compression_type(CompressionType::None)
                .build(),
        )
        .output_serialization(
            OutputSerialization::builder()
                .csv(CsvOutput::builder().build())
                .build(),
        )
        .send()
        .await
        .unwrap();

    let mut received = Vec::new();
    while let Some(event) = output.payload.recv().await.unwrap() {
        match event {
            SelectObjectContentEventStream::Records(records) => {
                received.push(
                    std::str::from_utf8(records.payload.as_ref().unwrap().as_ref())
                        .unwrap()
                        .trim()
                        .to_string(),
                );
            }
            SelectObjectContentEventStream::Stats(stats) => {
                let stats = stats.details.unwrap();
                received.push(format!(
                    "scanned:{},processed:{},returned:{}",
                    stats.bytes_scanned, stats.bytes_processed, stats.bytes_returned
                ))
            }
            SelectObjectContentEventStream::End(_) => {}
            otherwise => panic!("unexpected message: {:?}", otherwise),
        }
    }
    assert_eq!(
        vec![
            "Jane,(949) 555-6704,Chicago,Developer".to_string(),
            "scanned:333,processed:333,returned:39".to_string()
        ],
        received
    );

    // Validate the requests
    replayer
        .validate(&["content-type", "content-length"], validate_body)
        .await
        .unwrap();
}

fn validate_body(expected_body: &[u8], actual_body: &[u8]) -> Result<(), Box<dyn StdError>> {
    let expected = std::str::from_utf8(expected_body).unwrap();
    let actual = std::str::from_utf8(actual_body).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}
