mod peer;

use std::{io, time::Duration};

const HELP: &str = r#"
Command:
    ip                         Show local ip
    announce | -a  [data]      Announce device
    discover | -d              Discover

    help                       Print this message
"#;

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    let cmd = args.nth(1).unwrap_or("-h".into());
    let ip =  peer::local_ip();
    let port = 9876;

    match cmd.to_lowercase().as_str() {
        "ip" => println!("Local IP: {ip}", ),
        "announce" | "-a" => {
            println!("Announcing Local IP: {ip}");
            let data = args.next().unwrap_or_default();
            peer::announce((ip, port), data)?;
        },
        "discover" | "-d" => {
            println!("Finding...");

            peer::discover(port, Duration::from_secs(3), |src, data| {
                println!("{src}: {}", String::from_utf8_lossy(data));
            })?;
        },
        "help" | "--help" | "-h" | _ => println!("{HELP}")
    }
    Ok(())
}
