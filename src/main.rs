/*******************************************************************
 * Author:          machinageist
 * Date:            April 20, 2026
 * Description:     Scans for open ports on target domain/IpAddr
 *******************************************************************/
use std::env;
use std::net::ToSocketAddrs;
use std::time::Instant;
use time::OffsetDateTime;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let argnum = args.len();
    let mut handles = vec![];
    let mut ipaddr: Option<std::net::IpAddr> = None;
    let now = Instant::now();
    let date_time = OffsetDateTime::now_local().unwrap();
    let mut port_count = 0;

    println!("Starting mg-scan at {}", date_time);

    if argnum == 2 {
        let domain = args.get(1).expect("Usage: mg-scan <domain>"); 

        let target = format!("{domain}:0");

        match target.to_socket_addrs() {
            Ok(mut addrs) => {
                if let Some(addr) = addrs.next() {
                    ipaddr = Some(addr.ip());
                }
                println!("Host {} is up", &args[1]);
            }
            Err(e) => eprintln!("DNS resolution failed: {e}"),
        }
    }

    let ipaddr = ipaddr.unwrap_or_else(|| {
        eprintln!("Could not resolve host. Usage: mg-scan <domain>");
        std::process::exit(1);
    });

    println!("mg-scan report on host IP: {}", ipaddr);
    for port in 1u16..=65535 {
        let target = format!("{ipaddr}:{port}");
        
        let handle = tokio::spawn(async move {
            match tokio::net::TcpStream::connect(&target).await {
                Ok(_) => Some(port),
                Err(_) => None,
            }
        });
        handles.push(handle);
    }

    println!("-----   ----");
    println!("STATE   PORT");
    println!("-----   ----");
    for handle in handles {
        if let Ok(Some(port)) = handle.await {
            port_count = port_count + 1;
            println!("Open    {port}/tcp");
        }
    }

    println!("\n{} open ports found on {}", port_count, &args[1]);

    println!("\nmg-scan done: IP scanned in: {:?}.", now.elapsed());
}



