# dig-rs

A DNS resolver implementation in Rust, inspired by the classic `dig` command-line tool. This project provides a simple yet functional DNS client that can query domain names and resolve them to IP addresses.

## Features

- **DNS Resolution**: Query domain names and resolve them to IPv4 addresses
- **System Integration**: Automatically reads nameserver configuration from `/etc/resolv.conf`
- **UDP Communication**: Uses UDP sockets for efficient DNS queries
- **Structured Logging**: Built-in logging support with `env_logger`
- **Binary Protocol**: Implements DNS packet parsing and generation
- **Command-line Interface**: Simple CLI powered by `clap`

## Installation

### From Source

```bash
git clone <repository-url>
cd dig-rs
cargo build --release
```

The binary will be available at `target/release/dig-rs`.

## Usage

### Basic Usage

```bash
# Query a domain name
dig-rs example.com

# Query with debug logging
RUST_LOG=trace dig-rs example.com
```

### Example Output

```
Domain exists at: 93.184.216.34
```

## How It Works

1. **Configuration**: Reads the system's nameserver configuration from `/etc/resolv.conf`
2. **DNS Query**: Constructs a DNS query packet for the specified domain
3. **Network Communication**: Sends the query via UDP to the nameserver
4. **Response Parsing**: Parses the DNS response and extracts IP addresses
5. **Output**: Displays all resolved IP addresses

## Project Structure

- `src/main.rs` - Entry point and main application logic
- `src/args.rs` - Command-line argument parsing and nameserver configuration
- `src/client.rs` - DNS client implementation and network communication
- `src/packet/` - DNS packet structures and serialization/deserialization

## Dependencies

- **anyhow** - Error handling
- **binrw** - Binary reading/writing
- **clap** - Command-line argument parsing
- **env_logger** - Logging functionality
- **log** - Logging facade
- **modular-bitfield** - Bitfield operations for packet structures
- **rand** - Random number generation for DNS transaction IDs

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Debugging

Enable trace logging to see detailed DNS communication:

```bash
RUST_LOG=trace cargo run -- example.com
```

## Technical Details

- **Edition**: Rust 2024
- **Safety**: Unsafe code is forbidden (`#[forbid(unsafe_code)]`)
- **Protocol**: Implements DNS over UDP (RFC 1035)
- **Query Type**: Currently supports A record queries (IPv4 addresses)

## References

- [DNS Guide](https://github.com/EmilHernvall/dnsguide)
- [RFC Domain Names](https://datatracker.ietf.org/doc/html/rfc1035)
- [Wikipedia: DNS](https://en.wikipedia.org/wiki/Domain_Name_System)
- [dig.c](https://github.com/isc-projects/bind9/blob/main/bin/dig/dig.c)

## Tools

- [Online Hex Editor](https://hexed.it/)
- [Local Hex Editor](https://github.com/WerWolv/ImHex)

## License

This project is open source. Please check the license file for more details.

## Contributing

Contributions are welcome! Please feel free to submit issues, feature requests, or pull requests.