use prost_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protoc_path = &protoc_bin_vendored::protoc_bin_path().unwrap();
    std::env::set_var("PROTOC", protoc_path);

    let mut config = prost_build::Config::new();
    config.protoc_arg("--experimental_allow_proto3_optional");

    tonic_build::configure().compile_with_config(
        config,
        &["src/proto/cedar_detect.proto"], &["src/proto"])?;
    Ok(())
}
