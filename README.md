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
# symbolic link it to make it system available
sudo ln -sf /home/"$USER"/.cargo/bin/plnk /usr/local/bin/plnk
```

This will install `plnk` to your Cargo bin directory (usually `~/.cargo/bin/`), which should be in your PATH.

## Configuration

`plnk` reads its configuration fryom a TOML file under this path: `/etc/plnk/config.toml`

### Setting up the config file:

#### Create a `config.toml` file under this path: `/etc/plnk/config.toml`:

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

#### Symbolic link it to `/home/$USER/.config/plnk/config.toml`:

```bash
sudo ln -s /etc/plnk/config.toml /home/$USER/.config/plnk/config.toml 
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
   - Loads the configuration from the file specified at `/etc/plnk/config.toml`
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

## Author 

Mattia Tognela \<tognelamattia at protonmail dot com\>

## License

MIT License.

## Repository

[https://github.com/mtognela/plnk](https://github.com/mtognela/plnk)