# plnk

`plnk` is a simple command-line tool for blocking and unblocking specific websites on Unix-based systems by modifying the system's `/etc/hosts` file. It is configurable through a `config.toml` file and requires root privileges to run.

## Features

* Block a list of websites by redirecting them to `127.0.0.1`.
* Automatically backs up your `/etc/hosts` file before making changes.
* Restore the original `/etc/hosts` file with a single command.
* Easy configuration through `config.toml`.

## Installation

1. Clone the repository:

```bash
git clone https://github.com/mtognela/plnk.git
cd plnk
```

2. Build the project using Rust:

```bash
cargo build --release
```

3. Run the compiled binary as root to block or unblock websites.

> Note: Modifying `/etc/hosts` requires root privileges.

## Configuration

`plnk` reads a TOML configuration file named `config.toml` in the same directory.
Example:

```toml
blocked_domains = [
    "facebook.com",
    "twitter.com",
    "instagram.com"
]
```

You can list all the domains you want to block in `blocked_domains`.

## Usage

Run as root:

* **Block websites:**

```bash
sudo ./plnk
```

* **Unblock websites (restore hosts file):**

```bash
sudo ./plnk u
```

* **Usage help:**

```bash
./plnk
```

### Notes

* The program creates a backup of `/etc/hosts` at `/etc/hosts.backup` before blocking any sites.
* If websites are already blocked, it will notify you and exit.
* Restoring the hosts file will replace the current `/etc/hosts` with the backup.

## License

MIT License. See [LICENSE](LICENSE) for details.

## Repository

[https://github.com/mtognela/plnk](https://github.com/mtognela/plnk)
