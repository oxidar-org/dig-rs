# Contributing to dig-rs

Welcome to the dig-rs hack & learn session! This guide will help you get started with contributing to this DNS resolver implementation in Rust.

## 🚀 Getting Started

### Prerequisites
- Rust 1.70+ (install via [rustup.rs](https://rustup.rs/))
- Git
- A text editor or IDE (VS Code with rust-analyzer is recommended)

### Setup
1. Fork and clone the repository
2. Build the project: `cargo build`
3. Run tests: `cargo test`
4. Try it out: `cargo run -- google.com`

## 🎯 Project Overview

dig-rs is a DNS resolver that implements the DNS protocol from scratch. It's designed to be:
- **Educational**: Learn about network protocols and binary data handling
- **Extensible**: Easy to add new features
- **Modern**: Uses Rust 2024 edition with safe, idiomatic code

### Current Features
- A record (IPv4) DNS resolution
- System nameserver auto-detection
- UDP-based DNS queries
- Binary DNS packet parsing
- Command-line interface

## 🌟 How to Contribute

### For Beginners (Good First Issues)

#### 1. **Add More DNS Record Types**
**Difficulty**: ⭐⭐☆☆☆  
**Files to modify**: `src/packet/domain.rs`, `src/packet/route.rs`, `src/client.rs`

Current implementation only supports A records. Add support for:
- **AAAA records** (IPv6 addresses)
- **MX records** (mail exchange)
- **TXT records** (text records)
- **CNAME records** (canonical name)

**Getting started**:
```rust
// In src/packet/domain.rs, add new query types
pub enum QueryType {
    A = 1,
    AAAA = 28,  // Add this
    MX = 15,    // Add this
    TXT = 16,   // Add this
    CNAME = 5,  // Add this
}
```

#### 2. **Improve Output Formatting**
**Difficulty**: ⭐⭐☆☆☆  
**Files to modify**: `src/main.rs`, `src/args.rs`

Add command-line options for different output formats:
- `--json` for JSON output
- `--table` for tabular output
- `--verbose` for detailed information
- `--color` for colored output (use `colored` crate)

**Example**:
```bash
dig-rs --json google.com
dig-rs --table --verbose google.com
```

#### 3. **Add Query Timing**
**Difficulty**: ⭐⭐☆☆☆  
**Files to modify**: `src/client.rs`, `src/main.rs`

Measure and display query response times:
```rust
use std::time::Instant;

let start = Instant::now();
let routes = client.query_aa(args.name())?;
let duration = start.elapsed();
println!("Query time: {:?}", duration);
```

#### 4. **Add Configuration File Support**
**Difficulty**: ⭐⭐⭐☆☆  
**Files to modify**: `src/args.rs`

Support custom nameservers via config file:
- Read from `~/.dig-rs.toml`
- Support multiple nameservers
- Add retry logic

### For Intermediate Contributors

#### 5. **Implement Reverse DNS Lookups**
**Difficulty**: ⭐⭐⭐☆☆  
**Files to modify**: `src/client.rs`, `src/args.rs`

Add PTR record support to resolve IP addresses to domain names:
```bash
dig-rs --reverse 8.8.8.8
```

#### 6. **Add Caching Support**
**Difficulty**: ⭐⭐⭐☆☆  
**Files to create**: `src/cache.rs`

Implement a simple in-memory cache:
- Cache responses by domain name
- Respect TTL values
- Add cache statistics

#### 7. **Batch Query Support**
**Difficulty**: ⭐⭐⭐☆☆  
**Files to modify**: `src/args.rs`, `src/main.rs`

Allow querying multiple domains:
```bash
dig-rs --file domains.txt
dig-rs google.com github.com rust-lang.org
```

#### 8. **Add Different Nameserver Strategies**
**Difficulty**: ⭐⭐⭐☆☆  
**Files to modify**: `src/client.rs`, `src/args.rs`

Implement:
- Round-robin nameserver selection
- Fallback to secondary nameservers
- Custom nameserver via CLI: `dig-rs --nameserver 8.8.8.8 google.com`

### For Advanced Contributors

#### 9. **DNS over HTTPS (DoH) Support**
**Difficulty**: ⭐⭐⭐⭐☆  
**Files to create**: `src/doh.rs`

Add DoH support using providers like Cloudflare (1.1.1.1) or Google (8.8.8.8):
```bash
dig-rs --doh google.com
```

#### 10. **Async/Concurrent Queries**
**Difficulty**: ⭐⭐⭐⭐☆  
**Files to modify**: `src/client.rs`, `Cargo.toml`

Convert to async and support concurrent queries:
- Add `tokio` dependency
- Use `tokio::net::UdpSocket`
- Support parallel domain resolution

#### 11. **DNSSEC Validation**
**Difficulty**: ⭐⭐⭐⭐⭐  
**Files to create**: `src/dnssec.rs`

Implement DNSSEC validation:
- Parse RRSIG records
- Validate signatures
- Build trust chains

#### 12. **Simple DNS Server Implementation**
**Difficulty**: ⭐⭐⭐⭐⭐  
**Files to create**: `src/server.rs`

Create a basic DNS server that can:
- Listen on UDP port 53
- Respond to queries
- Forward unknown queries to upstream servers

## 📋 Development Guidelines

### Code Style
- Follow Rust standard formatting: `cargo fmt`
- No clippy warnings: `cargo clippy`
- Add tests for new features
- Document public APIs

### Testing
- Write unit tests for new functionality
- Run the full test suite: `cargo test`
- Test with real DNS queries when possible

### Documentation
- Update README.md for new features
- Add inline documentation for complex functions
- Include usage examples

### Commit Messages
Use conventional commits format:
- `feat: add AAAA record support`
- `fix: handle malformed DNS responses`
- `docs: update installation instructions`
- `test: add integration tests for MX records`

## 🐛 Finding Issues to Work On

### GitHub Labels
- `good first issue` - Perfect for beginners
- `help wanted` - Community contributions welcome
- `enhancement` - New features
- `bug` - Something isn't working
- `documentation` - Improvements to docs

### Issue Templates
When creating issues, include:
- **Description**: What needs to be done?
- **Acceptance Criteria**: How do we know it's complete?
- **Technical Notes**: Any implementation hints
- **Resources**: Links to relevant documentation

## 🎯 Session Structure

### Phase 1: Setup (15 minutes)
- Clone and build the project
- Run existing tests
- Try the basic functionality

### Phase 2: Explore (30 minutes)
- Read through the code structure
- Understand the DNS packet format
- Pick an issue to work on

### Phase 3: Implement (2+ hours)
- Work on your chosen feature
- Ask for help when needed
- Test your implementation

### Phase 4: Share (30 minutes)
- Demo your feature
- Get feedback from others
- Create a pull request

## 💡 Learning Resources

### DNS Protocol
- [DNS Guide](https://github.com/EmilHernvall/dnsguide) - Excellent tutorial
- [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035) - DNS specification
- [DNS Record Types](https://en.wikipedia.org/wiki/List_of_DNS_record_types)

### Rust Resources
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [binrw Documentation](https://docs.rs/binrw/) - Binary parsing
- [clap Documentation](https://docs.rs/clap/) - CLI argument parsing

### Tools
- [Online Hex Editor](https://hexed.it/) - Analyze DNS packets
- [Wireshark](https://www.wireshark.org/) - Network packet analysis
- [dig command](https://linux.die.net/man/1/dig) - Compare with reference implementation

## 🤝 Getting Help

### During the Session
- Ask questions in the chat/room
- Pair program with others
- Share your screen for debugging

### After the Session
- Create GitHub issues for bugs
- Join the Rust community Discord
- Continue contributing to the project

## 📝 Example Contributions

### Adding a New Record Type
1. **Update domain.rs** - Add new `QueryType` enum variant
2. **Update route.rs** - Add parsing for the new record type
3. **Update client.rs** - Add query method for the new type
4. **Update args.rs** - Add CLI option to request the new type
5. **Add tests** - Test parsing and querying
6. **Update README** - Document the new feature

### Adding Output Formatting
1. **Update args.rs** - Add format options to CLI
2. **Create formatter.rs** - Implement different output formats
3. **Update main.rs** - Use the new formatter
4. **Add tests** - Test different output formats
5. **Update README** - Show usage examples

Remember: **Start small, iterate quickly, and don't be afraid to ask questions!**

Happy hacking! 🦀