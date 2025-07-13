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

    let routes = client.query_aa(args.name())?;
    trace!("Found {} routes", routes.len());

    for r in routes {
        println!("{} exists at: {}", r.domain.name(), r.adr);
    }

    Ok(())
}
