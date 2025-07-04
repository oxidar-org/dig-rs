use log::trace;

use crate::client::DNSResolver;

mod args;
mod client;
mod packet;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args = args::build();
    let nameserver = args.nameserver()?;
    let client = client::connect(&nameserver)?;
    trace!("Connected to nameserver {nameserver}");

    let ips = client.query_aa(args.name())?;
    trace!("Found {} addresses", ips.len());

    for ip in ips {
        println!("Domain exists at: {ip}");
    }

    Ok(())
}
