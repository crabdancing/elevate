# sudo

[![crates.io](https://img.shields.io/crates/v/sudo?logo=rust)](https://crates.io/crates/sudo/)
[![docs.rs](https://docs.rs/sudo/badge.svg)](https://docs.rs/sudo)

Detect if you are running as root, restart self with `sudo` if needed or setup uid zero when running with the SUID flag set.

## Requirements

* The `sudo` program is required to be installed and setup correctly on the target system.
* Linux or Mac OS X tested
    * It should work on *BSD. However, it is not tested.

# Example:

First, add sudo to your `Cargo.toml`:

```yaml
[dependencies]
elevate = "0.6.1"
```

In your `main.rs`:

```rust
fn main() -> Result<(), Box<dyn Error>> {
    elevate::escalate_if_needed()?;
    println!("Hello, Root-World!");
    Ok( () )
}
```

If you are using logging based on the [log infrastructure](https://crates.io/crates/log) you will get timestamped and formatted output.

## Passing RUST_BACKTRACE

The crate will automatically keep the setting of `RUST_BACKTRACE` intact if it is set to one of the following values:

* `` <- empty string means no pass-through
* `1` or `true` <- standard trace
* `full` <- full trace

```bash
$ RUST_BACKTRACE=full cargo run --example backtrace
2020-07-05 18:10:31,544 TRACE [sudo] Running as User
2020-07-05 18:10:31,544 DEBUG [sudo] Escalating privileges
2020-07-05 18:10:31,544 TRACE [sudo] relaying RUST_BACKTRACE=full
[sudo] Passwort für user:
2020-07-05 18:10:39,238 TRACE [sudo] Running as Root
2020-07-05 18:10:39,238 TRACE [sudo] already running as Root
2020-07-05 18:10:39,238 INFO  [backtrace] entering failing_function
thread 'main' panicked at 'now you see me fail', examples/backtrace.rs:16:5
```

## Keeping part of the environment

You can keep parts of your environment across the sudo barrier.
This enables more configuration options often used in daemons or cloud environments:

```rust
    // keeping all environment variables starting with "EXAMPLE_" or "CARGO"
    elevate::with_env(&["EXAMPLE_", "CARGO"]).expect("sudo failed");
```

**Warning:** This may introduce security problems to your application if untrusted users are able to set these variables.

```bash
$ EXAMPLE_EXEC='$(ls)' EXAMPLE_BTICKS='`ls`' cargo run --example environment
2020-07-07 16:32:11,261 INFO  [environment] ① uid: 1000; euid: 1000;

...

declare -x EXAMPLE_BTICKS="\`ls\`"
declare -x EXAMPLE_EXEC="\$(ls)"
...

[sudo] password for user:

2020-07-07 16:32:11,285 TRACE [sudo] Running as Root
2020-07-07 16:32:11,285 TRACE [sudo] already running as Root
2020-07-07 16:32:11,285 INFO  [environment] ② uid: 0; euid: 0;

...

declare -x EXAMPLE_BTICKS="\`ls\`"
declare -x EXAMPLE_EXEC="\$(ls)"
```

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
