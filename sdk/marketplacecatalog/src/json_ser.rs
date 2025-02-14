// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_list_change_sets_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListChangeSetsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.catalog {
        object.key("Catalog").string(var_1);
    }
    if let Some(var_2) = &input.filter_list {
        let mut array_3 = object.key("FilterList").start_array();
        for item_4 in var_2 {
            {
                let mut object_5 = array_3.value().start_object();
                crate::json_ser::serialize_structure_crate_model_filter(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    if let Some(var_6) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    if let Some(var_7) = &input.next_token {
        object.key("NextToken").string(var_7);
    }
    if let Some(var_8) = &input.sort {
        let mut object_9 = object.key("Sort").start_object();
        crate::json_ser::serialize_structure_crate_model_sort(&mut object_9, var_8)?;
        object_9.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_entities_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListEntitiesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_10) = &input.catalog {
        object.key("Catalog").string(var_10);
    }
    if let Some(var_11) = &input.entity_type {
        object.key("EntityType").string(var_11);
    }
    if let Some(var_12) = &input.filter_list {
        let mut array_13 = object.key("FilterList").start_array();
        for item_14 in var_12 {
            {
                let mut object_15 = array_13.value().start_object();
                crate::json_ser::serialize_structure_crate_model_filter(&mut object_15, item_14)?;
                object_15.finish();
            }
        }
        array_13.finish();
    }
    if let Some(var_16) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_16).into()),
        );
    }
    if let Some(var_17) = &input.next_token {
        object.key("NextToken").string(var_17);
    }
    if let Some(var_18) = &input.sort {
        let mut object_19 = object.key("Sort").start_object();
        crate::json_ser::serialize_structure_crate_model_sort(&mut object_19, var_18)?;
        object_19.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_change_set_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartChangeSetInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_20) = &input.catalog {
        object.key("Catalog").string(var_20);
    }
    if let Some(var_21) = &input.change_set {
        let mut array_22 = object.key("ChangeSet").start_array();
        for item_23 in var_21 {
            {
                let mut object_24 = array_22.value().start_object();
                crate::json_ser::serialize_structure_crate_model_change(&mut object_24, item_23)?;
                object_24.finish();
            }
        }
        array_22.finish();
    }
    if let Some(var_25) = &input.change_set_name {
        object.key("ChangeSetName").string(var_25);
    }
    if let Some(var_26) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_26);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_filter(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Filter,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_27) = &input.name {
        object.key("Name").string(var_27);
    }
    if let Some(var_28) = &input.value_list {
        let mut array_29 = object.key("ValueList").start_array();
        for item_30 in var_28 {
            {
                array_29.value().string(item_30);
            }
        }
        array_29.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_sort(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Sort,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_31) = &input.sort_by {
        object.key("SortBy").string(var_31);
    }
    if let Some(var_32) = &input.sort_order {
        object.key("SortOrder").string(var_32.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_change(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Change,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_33) = &input.change_type {
        object.key("ChangeType").string(var_33);
    }
    if let Some(var_34) = &input.entity {
        let mut object_35 = object.key("Entity").start_object();
        crate::json_ser::serialize_structure_crate_model_entity(&mut object_35, var_34)?;
        object_35.finish();
    }
    if let Some(var_36) = &input.details {
        object.key("Details").string(var_36);
    }
    if let Some(var_37) = &input.change_name {
        object.key("ChangeName").string(var_37);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_entity(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Entity,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_38) = &input.r#type {
        object.key("Type").string(var_38);
    }
    if let Some(var_39) = &input.identifier {
        object.key("Identifier").string(var_39);
    }
    Ok(())
}
