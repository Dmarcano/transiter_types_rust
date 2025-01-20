fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "src/";
    std::fs::create_dir_all(path)?;
    let out_dir = std::path::PathBuf::from(path);

    tonic_build::configure()
        .btree_map(&["."])
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .out_dir(out_dir)
        .compile(&vec!["src/public.proto", "src/admin.proto"], &["./proto"])?;
    Ok(())
}
