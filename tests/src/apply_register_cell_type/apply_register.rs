use super::common::*;
use crate::util::{
    accounts::*, constants::*, error::Error, template_common_cell::*, template_generator::*, template_parser::*,
};
use serde_json::{json, Value};

fn before() -> TemplateGenerator {
    init("apply_register")
}

#[test]
fn test_apply_register() {
    let mut template = before();

    push_output_apply_register_cell(
        &mut template,
        json!({
            "data": {
                "height": HEIGHT,
                "timestamp": TIMESTAMP,
            }
        }),
    );

    test_tx(template.as_json())
}

#[test]
fn challenge_apply_register_consuming_cell() {
    let mut template = before();

    // Simulate consuming ApplyRegisterCell.
    push_input_apply_register_cell(
        &mut template,
        json!({
            "data": {
                "account": ACCOUNT_1,
                "height": HEIGHT,
                "timestamp": TIMESTAMP,
            }
        }),
    );

    push_output_apply_register_cell(
        &mut template,
        json!({
            "data": {
                "height": HEIGHT,
                "timestamp": TIMESTAMP,
            }
        }),
    );

    challenge_tx(template.as_json(), Error::InvalidTransactionStructure)
}

#[test]
fn challenge_apply_register_missing_field() {
    let mut template = before();

    push_output_apply_register_cell(
        &mut template,
        json!({
            "data": {
                // Simulate missing some field in data.
                "height": Value::Null,
                "timestamp": TIMESTAMP,
            }
        }),
    );

    challenge_tx(template.as_json(), Error::InvalidCellData)
}

#[test]
fn challenge_apply_register_invalid_height_1() {
    let mut template = before();

    push_output_apply_register_cell(
        &mut template,
        json!({
            "data": {
                // Simulate the height in data is not match with which in HeightCell.
                "height": HEIGHT - 1,
                "timestamp": TIMESTAMP,
            }
        }),
    );

    challenge_tx(template.as_json(), Error::InvalidCellData)
}

#[test]
fn challenge_apply_register_invalid_height_2() {
    let mut template = before();

    push_output_apply_register_cell(
        &mut template,
        json!({
            "data": {
                // Simulate the height in data is not match with which in HeightCell.
                "height": HEIGHT + 1,
                "timestamp": TIMESTAMP,
            }
        }),
    );

    challenge_tx(template.as_json(), Error::InvalidCellData)
}

#[test]
fn challenge_apply_register_invalid_timestamp_1() {
    let mut template = before();

    push_output_apply_register_cell(
        &mut template,
        json!({
            "data": {
                "height": HEIGHT,
                // Simulate the timstamp in data is not match with which in TimestampCell.
                "timestamp": TIMESTAMP - 1,
            }
        }),
    );

    challenge_tx(template.as_json(), Error::InvalidCellData)
}

#[test]
fn challenge_apply_register_invalid_timestamp_2() {
    let mut template = before();

    push_output_apply_register_cell(
        &mut template,
        json!({
            "data": {
                "height": HEIGHT,
                // Simulate the timstamp in data is not match with which in TimestampCell.
                "timestamp": TIMESTAMP + 1,
            }
        }),
    );

    challenge_tx(template.as_json(), Error::InvalidCellData)
}