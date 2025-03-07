fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../proto/wireguard1.proto")?;
    Ok(())
}