fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "src/";
    std::fs::create_dir_all(path)?;
    let out_dir = std::path::PathBuf::from(path);

    tonic_build::configure()
        .btree_map(&["."])
        .type_attribute(".",  "#[derive(serde::Serialize, serde::Deserialize)]")
        // default message vectors to be empty
        .message_attribute(".",  "#[serde(default)]")
        // TODO: Add support for other enums
        .field_attribute(".public_api_types.Stop.type", 
    "#[serde(deserialize_with = \"super::Type::from_str\")]").out_dir(out_dir)
        .compile(&vec!["src/public.proto", "src/admin.proto"], &["./proto"])?;
    Ok(())
}