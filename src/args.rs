use anyhow::{Context, Ok, Result};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    name: String,
}

impl Args {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn nameserver(&self) -> Result<String> {
        let file = std::fs::read_to_string("/etc/resolv.conf")?;
        let line = file
            .lines()
            .find(|l| l.starts_with("nameserver"))
            .context("missing nameserver entry")?;
        let adr = line
            .split_ascii_whitespace()
            .nth(1)
            .context("resolv.conf bad format")?;
        let adr = if adr.contains(":") {
            adr.to_string()
        } else {
            format!("{adr}:53")
        };
        Ok(adr)
    }
}

pub fn build() -> Args {
    Args::parse()
}
