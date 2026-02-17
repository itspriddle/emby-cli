use std::net::UdpSocket;
use std::time::Duration;

use crate::cli::FindServerArgs;
use crate::error::Result;
use crate::format::table;

const DISCOVERY_PORT: u16 = 7359;
const DISCOVERY_MSG: &[u8] = b"who is EmbyServer?";

#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DiscoveryResponse {
    address: Option<String>,
    id: Option<String>,
    name: Option<String>,
}

pub fn run(args: &FindServerArgs) -> Result<()> {
    let timeout = Duration::from_secs(args.timeout);

    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;
    socket.set_read_timeout(Some(timeout))?;
    socket.send_to(DISCOVERY_MSG, ("255.255.255.255", DISCOVERY_PORT))?;

    let mut servers: Vec<DiscoveryResponse> = Vec::new();
    let mut buf = [0u8; 4096];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((len, _addr)) => {
                if let Ok(resp) = serde_json::from_slice::<DiscoveryResponse>(&buf[..len]) {
                    // Deduplicate by server ID
                    let id = resp.id.as_deref().unwrap_or("");
                    if !servers.iter().any(|s| s.id.as_deref().unwrap_or("") == id) {
                        servers.push(resp);
                    }
                }
            }
            Err(e)
                if e.kind() == std::io::ErrorKind::WouldBlock
                    || e.kind() == std::io::ErrorKind::TimedOut =>
            {
                break;
            }
            Err(e) => return Err(e.into()),
        }
    }

    if servers.is_empty() {
        println!("No Emby servers found on the local network");
        return Ok(());
    }

    let rows: Vec<Vec<String>> = servers
        .iter()
        .map(|s| {
            vec![
                s.name.as_deref().unwrap_or("").to_string(),
                s.address.as_deref().unwrap_or("").to_string(),
                s.id.as_deref().unwrap_or("").to_string(),
            ]
        })
        .collect();

    println!("{}", table::build_table(&["Name", "Address", "ID"], rows));

    Ok(())
}
