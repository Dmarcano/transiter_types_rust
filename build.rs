use tonic_build::Builder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "src/";
    std::fs::create_dir_all(path)?;
    let out_dir = std::path::PathBuf::from(path);

   let mut builder = tonic_build::configure()
        .btree_map(&["."])
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        // default message vectors to be empty
        .message_attribute(".", "#[serde(default)]");
        // TODO: Add support for other enums
        builder = add_enum_serialization(builder);
        builder.out_dir(out_dir)
        .compile(&vec!["src/public.proto", "src/admin.proto"], &["./proto"])?;
    Ok(())
}

pub fn add_enum_serialization(mut builder: Builder) -> Builder {
    builder = builder
        .field_attribute(
            ".public_api_types.Stop.type",
            "#[serde(deserialize_with = \"super::Type::from_str\")]",
        )
        .field_attribute(
            ".public_api_types.Alert.cause",
            "#[serde(deserialize_with = \"super::Cause::from_str\")]",
        )
        .field_attribute(
            ".public_api_types.Alert.effect",
            "#[serde(deserialize_with = \"super::Effect::from_str\")]",
        )
        .field_attribute(
            ".public_api_types.Route.continuous_pickup",
            "#[serde(deserialize_with = \"super::ContinuousPolicy::from_str\")]",
        )
        .field_attribute(
            ".public_api_types.Route.continuous_drop_off",
            "#[serde(deserialize_with = \"super::ContinuousPolicy::from_str\")]",
        )
        .field_attribute(
            ".public_api_types.Route.type",
            "#[serde(deserialize_with = \"super::RouteType::from_str\")]",
        )
        .field_attribute(
            ".public_api_types.System.status",
            "#[serde(deserialize_with = \"super::Status::from_str\")]",
        )
        .field_attribute(
            ".public_api_types.Transfer.type",
            "#[serde(deserialize_with = \"super::TransferType::from_str\")]",
        )
        .field_attribute(
            ".public_api_types.Vehicle.congestion_level",
            "#[serde(deserialize_with = \"super::CongestionLevel::from_str\")]",
        )
        // TODO: Figure out how to deserialize an optional enum
        // .field_attribute(
        //     ".public_api_types.Vehicle.occupancy_status",
        //     "#[serde(deserialize_with = \"super::OccupancyStatus::option_from_str\")]",
        // )
        .field_attribute(
            ".public_api_types.Vehicle.occupancy_status",
            "#[serde(skip_deserializing)]",
        )
        .field_attribute(
            ".public_api_types.Vehicle.current_status",
            "#[serde(skip_deserializing)]",
        );

    builder
}
