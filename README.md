# plnk

`plnk` is a simple command-line tool for blocking and unblocking specific domains on Unix-based systems by modifying the system's `/etc/hosts` file. It redirects blocked domains to `127.0.0.1` and provides easy backup and restoration functionality.

## Features

* Block a list of domains by redirecting them to `127.0.0.1`
* Automatically backs up your `/etc/hosts` file before making changes
* Restore the original `/etc/hosts` file with a single command
* Configurable through a TOML configuration file
* Built-in validation to prevent duplicate blocking
* Root privilege checking for security

## Installation

### Option 1: Install directly from Git (Recommended)

```bash
cargo install --git https://github.com/mtognela/plnk.git
```

This will install `plnk` to your Cargo bin directory (usually `~/.cargo/bin/`), which should be in your PATH.

### Option 2: Build from source

1. Clone the repository:

```bash
git clone https://github.com/mtognela/plnk.git
cd plnk
```

2. Build the project using Rust:

```bash
cargo build --release
```

3. The compiled binary will be available at `target/release/plnk`

> **Note**: Modifying `/etc/hosts` requires root privileges.

## Configuration

`plnk` reads its configuration from a TOML file specified by the `PLNK_CONFIG` environment variable.

### Setting up the config file:

1. Create a `config.toml` file:

```toml
blocked_domains = [
    "facebook.com",
    "www.facebook.com",
    "twitter.com",
    "www.twitter.com",
    "instagram.com",
    "www.instagram.com",
    "youtube.com",
    "www.youtube.com"
]
```

2. Set the environment variable **as root** to point to your config file:

```bash
sudo su
export PLNK_CONFIG=/path/to/your/config.toml
```

Or add it to root's shell profile for persistence:

```bash
sudo su
echo 'export PLNK_CONFIG=/path/to/your/config.toml' >> /root/.bashrc
source /root/.bashrc
```

Alternatively, you can set it when running the command:

```bash
sudo PLNK_CONFIG=/path/to/your/config.toml plnk
```

## Usage

All operations require root privileges:

### Block domains:
```bash
sudo plnk
```

### Unblock domains (restore original hosts file):
```bash
sudo plnk u
```

### Show help:
```bash
plnk h
# or
plnk help
```

## How it works

1. **Blocking**: When you run `plnk` without arguments, it:
   - Checks if you have root privileges
   - Loads the configuration from the file specified by `PLNK_CONFIG`
   - Verifies that domains aren't already blocked
   - Creates a backup of `/etc/hosts` at `/etc/hosts.backup`
   - Appends the blocked domains to `/etc/hosts` with the marker `# plnk url blocking`

2. **Unblocking**: When you run `plnk u`, it:
   - Restores the original `/etc/hosts` file from the backup
   - Removes the backup file

## Error Handling

The tool includes comprehensive error handling for:
- Missing or invalid configuration files
- Permission issues (not running as root)
- File I/O errors
- Invalid domain entries (empty domains)
- Attempting to block already blocked domains

## Requirements

- Rust (for building)
- Unix-based system (Linux, macOS, etc.)
- Root privileges for execution
- TOML configuration file

## Dependencies

- `serde` - for configuration deserialization
- `toml` - for parsing TOML configuration files

## Notes

- The program creates a backup at `/etc/hosts.backup` before making any changes
- If domains are already blocked, the program will exit with an error message
- Restoring will completely replace the current `/etc/hosts` with the backup
- Make sure to include both `domain.com` and `www.domain.com` variants in your config for complete blocking
- The `PLNK_CONFIG` environment variable must be set in the **root user's environment** before running the program
- You can either set it permanently in root's profile or pass it when running the command

## License

MIT License.

## Repository

[https://github.com/mtognela/plnk](https://github.com/mtognela/plnk)