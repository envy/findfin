use std::env;
use std::net::UdpSocket;

const JELLYFIN_DISCOVERY_PORT: u16 = 7359;
const DISCOVERY_MSG: &str = "who is jellyfinserver?";

#[derive(serde::Deserialize)]
struct PublicInfo {
    #[serde(alias = "Id")]
    id: String,
    #[serde(alias = "ServerName")]
    server_name: String,
}

fn fetch_public_info(server_url: &str) -> PublicInfo {
    let url = format!("{}/System/Info/Public", server_url.trim_end_matches('/'));
    reqwest::blocking::get(&url)
        .unwrap_or_else(|e| panic!("failed to reach {url}: {e}"))
        .json()
        .unwrap_or_else(|e| panic!("invalid response from {url}: {e}"))
}

fn main() {
    let bind_addr = env::var("FINDFIN_BIND_ADDR").unwrap_or_else(|_| "0.0.0.0".to_string());
    let server_url = env::var("FINDFIN_SERVER_URL").expect("FINDFIN_SERVER_URL must be set");
    let announce_url = env::var("FINDFIN_ANNOUNCE_URL").unwrap_or_else(|_| server_url.clone());

    println!("querying {server_url} for server info...");
    let info = fetch_public_info(&server_url);
    let server_id = info.id;
    let server_name = info.server_name;
    println!("got server id: {server_id}");
    println!("got server name: {server_name}");

    let response = format!(
        r#"{{"Address":"{}","Id":"{}","Name":"{}"}}"#,
        announce_url, server_id, server_name
    );
    let response_bytes = response.as_bytes();

    let bind = format!("{bind_addr}:{JELLYFIN_DISCOVERY_PORT}");
    let socket = UdpSocket::bind(&bind).unwrap_or_else(|e| {
        panic!("failed to bind to {bind}: {e}");
    });

    println!("will respond with: {response}");
    println!("listening on {bind}");

    let mut buf = [0u8; 256];
    loop {
        let (len, src) = match socket.recv_from(&mut buf) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("recv error: {e}");
                continue;
            }
        };

        let msg = match std::str::from_utf8(&buf[..len]) {
            Ok(s) => s.trim(),
            Err(_) => continue,
        };

        if msg.eq_ignore_ascii_case(DISCOVERY_MSG) {
            println!("discovery request from {src}");
            if let Err(e) = socket.send_to(response_bytes, src) {
                eprintln!("send error to {src}: {e}");
            }
        }
    }
}
