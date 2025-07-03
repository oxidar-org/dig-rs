mod args;
mod client;

fn main() -> anyhow::Result<()> {
    let args = args::build();
    let nameserver = args.nameserver()?;
    let _client = client::build(&nameserver)?;
    dbg!(args.name());
    dbg!(nameserver);
    Ok(())
}
