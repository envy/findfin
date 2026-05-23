use std::env;
use std::net::UdpSocket;

const JELLYFIN_DISCOVERY_PORT: u16 = 7359;
const DISCOVERY_MSG: &str = "who is jellyfinserver?";

fn main() {
    let bind_addr = env::var("FINDFIN_BIND_ADDR").unwrap_or_else(|_| "0.0.0.0".to_string());
    let server_url = env::var("FINDFIN_SERVER_URL").expect("FINDFIN_SERVER_URL must be set");
    let server_id = env::var("FINDFIN_SERVER_ID").expect("FINDFIN_SERVER_ID must be set");
    let server_name = env::var("FINDFIN_SERVER_NAME").expect("FINDFIN_SERVER_NAME must be set");

    let response = format!(
        r#"{{"Address":"{}","Id":"{}","Name":"{}"}}"#,
        server_url, server_id, server_name
    );
    let response_bytes = response.as_bytes();

    let bind = format!("{bind_addr}:{JELLYFIN_DISCOVERY_PORT}");
    let socket = UdpSocket::bind(&bind).unwrap_or_else(|e| {
        panic!("failed to bind to {bind}: {e}");
    });

    eprintln!("will respond with: {response}");
    eprintln!("listening on {bind}");

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
            eprintln!("discovery request from {src}");
            if let Err(e) = socket.send_to(response_bytes, src) {
                eprintln!("send error to {src}: {e}");
            }
        }
    }
}
