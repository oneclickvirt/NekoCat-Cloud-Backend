use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct VmConfig {
    vmid: u32,
    ostemplate: String,
    hostname: String,
    password: String,
    storage: String,
}

async fn pve_create_kvm() -> Result<(), Box<dyn Error>> {
    let pool = get_db_pool().await?;
    let api_token = ""; //root@pam!tokenid=tokensecret
    let base_url = "https://your-proxmox-server:8006/api2/json";

    let client = Client::new();
    let vm_config = VmConfig {
        vmid: 100,
        ostemplate: "local:vztmpl/ubuntu-20.04-standard_20.04-1_amd64.tar.gz",
        hostname: "testvm",
        password: "verysecurepassword",
        storage: "local-lvm",
    };

    let response = client.post(format!("{}/nodes/your-node/qemu", base_url))
        .bearer_auth(api_token)
        .json(&vm_config)
        .send()
        .await?;

    let response_text = response.text().await?;
    println!("Response: {}", response_text);

    Ok(())
}
