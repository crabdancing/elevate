# sudo

Detect if you are running as root, restart self with sudo if needed

# Example:

First, add sudo to your `Cargo.toml`:

```yaml
[dependencies]
# for now:
sudo = { git = "https://gitlab.com/dns2utf8/sudo.rs.git" }
# once this crate is on crates.io
# sudo = "0.2"
```

In your `main.rs`:

```rust
fn main() -> Result<(), Box<dyn Error>> {
    sudo::escalate_if_needed()?;
    println!("Hello, Root-World!");
    Ok( () )
}
```
