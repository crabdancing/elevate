# sudo

[![crates.io](https://img.shields.io/crates/v/sudo?logo=rust)](https://crates.io/crates/sudo/)

Detect if you are running as root, restart self with `sudo` if needed.

The `sudo` program is required to be installed and setup correctly on the target system.

# Example:

First, add sudo to your `Cargo.toml`:

```yaml
[dependencies]
sudo = "0.3"
```

In your `main.rs`:

```rust
fn main() -> Result<(), Box<dyn Error>> {
    sudo::escalate_if_needed()?;
    println!("Hello, Root-World!");
    Ok( () )
}
```

If you are using logging based on the [log](https://crates.io/crates/log) infrastructure you will get 

## Run a program with SUID

```bash
$ cargo run --example suid
2020-04-17 15:13:49,450 INFO  [suid] ① uid: 1000; euid: 1000;
uid=1000(user) gid=1000(user) groups=1000(user),4(adm),27(sudo)
2020-04-17 15:13:49,453 TRACE [sudo] Running as User
2020-04-17 15:13:49,453 DEBUG [sudo] Escalating privileges
[sudo] password for user: 
2020-04-17 15:13:53,529 INFO  [suid] ① uid: 0; euid: 0;
uid=0(root) gid=0(root) groups=0(root)
2020-04-17 15:13:53,532 TRACE [sudo] Running as Root
2020-04-17 15:13:53,532 TRACE [sudo] already running as Root
2020-04-17 15:13:53,532 INFO  [suid] ② uid: 0; euid: 0;
uid=0(root) gid=0(root) groups=0(root)

```

Then give the file to `root` and add the suid flag.

```bash
$ sudo chown root target/debug/examples/suid
$ sudo chmod 4755 target/debug/examples/suid
```

Now run the program again:

```bash
$ target/debug/examples/suid
2020-04-17 15:14:37,199 INFO  [suid] ① uid: 1000; euid: 0;
uid=1000(user) gid=1000(user) euid=0(root) groups=1000(user),4(adm),27(sudo)
2020-04-17 15:14:37,202 TRACE [sudo] Running as Suid
2020-04-17 15:14:37,202 TRACE [sudo] setuid(0)
2020-04-17 15:14:37,202 INFO  [suid] ② uid: 0; euid: 0;
uid=0(root) gid=1000(user) groups=1000(user),4(adm),27(sudo)
```
