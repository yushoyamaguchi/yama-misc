fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .type_attribute("wireguard1.WireGuardRequest", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("wireguard1.Overlay", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("wireguard1.WireGuardConfiguration", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("wireguard1.WireGuardPeer", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("wireguard1.WireGuardResponse", "#[derive(serde::Serialize, serde::Deserialize)]")
        .out_dir("src/wireguard1")
        .compile(&["../proto/wireguard1.proto"], &["../proto"])?;
    Ok(())
}
