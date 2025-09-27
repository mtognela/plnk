use std::env;
use std::fs;
use std::io::{self, Write, BufRead, BufReader};
use std::path::Path;
use std::process;
use serde::Deserialize;

const HOSTS_FILE: &str = "/etc/hosts";
const HOSTS_BACKUP: &str = "/etc/hosts.backup";
const URL_TO_REDIRECT: &str = "127.0.0.1";
const CONFIG_FILE: &str = "config.toml";

#[derive(Deserialize)]
struct Config {
    blocked_domains: Vec<String>,
}

fn die(msg: &str) -> ! {
    eprintln!("{}",msg);
    process::exit(1);
}

fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

fn backup_hosts() -> io::Result<()> {
    let content = fs::read_to_string(HOSTS_FILE)
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "cannot read host file"))?;
    
    fs::write(HOSTS_BACKUP, content)
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "cannot create backup file"))?;
    
    Ok(())
}

fn restore_hosts() -> Result<(), String> {
    if !file_exists(HOSTS_BACKUP) {
        return Err("no backup file found".to_string());
    }

    if let Err(_) = fs::rename(HOSTS_BACKUP, HOSTS_FILE) {
        return Err("failed to restore hosts file".to_string());
    }

    println!("urls unblocked");
    Ok(())
}

fn is_blocked_line(line: &str, site_to_block: &Vec<String>) -> bool {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 2 {
        return false;
    }
    let host = parts[1];
    site_to_block.iter().any(|d| d == host)
}

fn check_already_blocked(site_to_block: &Vec<String>) -> bool {
    if let Ok(file) = fs::File::open(HOSTS_FILE) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line_content) = line {
                if is_blocked_line(&line_content, site_to_block) {
                    return true
                }
            }
        }
    }
    false 
}

fn block_sites(sites_to_block: &Vec<String>) -> Result<(), String> {
    if check_already_blocked(sites_to_block) {
        return Err("urls already blocked".to_string());
    }

    // Backup original hosts file
    backup_hosts().map_err(|e| format!("backup failed: {}", e))?;

    // Append blocked domains
    let mut hosts_file = fs::OpenOptions::new()
        .append(true)
        .open(HOSTS_FILE)
        .map_err(|_| format!("cannot write to {}", HOSTS_FILE))?;

    writeln!(hosts_file, "\n# plnk url blocking")
        .map_err(|_| format!("failed to write to {}", HOSTS_FILE))?;

    for domain in sites_to_block {
        writeln!(hosts_file, "{} {}", URL_TO_REDIRECT, domain)
            .map_err(|_| "failed to write to hosts file".to_string())?;
    }

    println!("urls blocked");
    Ok(())
}

fn usage() -> ! {
    let argv0 = env::args().next().unwrap_or_else(|| "program".to_string());
    eprintln!("usage: {} [u]", argv0);
    eprintln!("  u  unblock");
    process::exit(1);
}

fn check_root() -> Result<(), String> {
    #[cfg(unix)]
    {
        if let Err(_) = fs::OpenOptions::new().append(true).open(HOSTS_FILE) {
            return  Err("must run as root".to_string());
        }
        Ok(())
    }
}

fn load_config() -> Result<Config, String> {
    let content = fs::read_to_string(CONFIG_FILE)
        .map_err(|_| format!("Failed to read {}", CONFIG_FILE))?;

    let config: Config = toml::from_str(&content)
        .map_err(|err| format!("Failed to parse {}: {}", CONFIG_FILE, err))?;

    Ok(config)
}

fn main() {
    let config = load_config().unwrap_or_else(|err| die(&err));
    let args: Vec<String> = env::args().collect();

    check_root().unwrap_or_else(|err|die(&err));

    match args.len() {
        2 if args[1] == "u" => {
            if let Err(e) = restore_hosts() {
                die(&e);
            }
        }
        1 => {
            if let Err(e) = block_sites(&config.blocked_domains) {
                die(&e);
            }
        }
        _ => usage(),
    }
}
