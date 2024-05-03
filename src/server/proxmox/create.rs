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

pub async fn pve_create_kvm(ip: &str, post: &str, key: &str, storage: &str, node_name: &str, _network_name: &str, template: &str, hostname: &str, password: &str) -> Result<(), Box<dyn Error>> {
    let api_token = ""; //root@pam!tokenid=tokensecret
    let base_url = format!("https://{}:{}/api2/json", ip, post);

    let client = Client::new();
    let vm_config = VmConfig {
        vmid: 100,
        ostemplate: format!("{}", template),
        hostname: format!("{}", hostname),
        password: format!("{}", password),
        storage: format!("{}", storage),
    };

    let response = client.post(format!("{}/nodes/{}/qemu", base_url, node_name))
        .bearer_auth(api_token)
        .json(&vm_config)
        .send()
        .await?;

    let response_text = response.text().await?;
    println!("Response: {}", response_text);

    Ok(())
}
