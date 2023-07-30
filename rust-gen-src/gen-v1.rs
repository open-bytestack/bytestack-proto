fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("format/v1/v1.proto")?;
    Ok(())
}
