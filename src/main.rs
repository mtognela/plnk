use std::env::args;
use std::env::var;
use std::fmt;
use std::fmt::Formatter;
use std::fs;
use std::fs::read_to_string;
use std::fs::rename;
use std::fs::write;
use std::io::{Write, BufRead, BufReader};
use std::path::Path;
use std::process::exit;
use serde::Deserialize;
use std::fmt::Display;

const HOSTS_FILE: &str = "/etc/hosts";
const HOSTS_BACKUP: &str = "/etc/hosts.backup";
const URL_TO_REDIRECT: &str = "127.0.0.1";
const PLNK_PARTIAL_PATH: &str = ".config/plnk/config.toml";
const HOME_ENV: &str = "HOME"; 
const PLNK_MARKER: &str = "# plnk url blocking";

#[derive(Deserialize)]
struct Config {
    blocked_domains: Vec<String>,
}

fn die<T: Display>(msg: T) -> ! {
    eprintln!("Error: {}", msg);
    exit(1);
}

#[derive(Debug)]
enum PlnkError<T: Display> {
    Config(T),
    HostsError(T),
    PermissionError(T),
    BackupError(T),
    IllegalState(T),
    Io(T),
}

impl<T: Display> Display for PlnkError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PlnkError::Config(msg) => write!(f, "Config error: {}", msg),
            PlnkError::HostsError(msg) => write!(f, "Hosts file error: {}", msg),
            PlnkError::PermissionError(msg) => write!(f, "Permission denied: {}", msg),
            PlnkError::BackupError(msg) => write!(f, "Backup error: {}", msg),
            PlnkError::IllegalState(msg) => write!(f, "Illegal state: {}", msg),
            PlnkError::Io(msg) => write!(f, "Io error: {}", msg),
        }
    }
}


fn backup_hosts() -> Result<(), PlnkError<String>> {
    let content = read_to_string(HOSTS_FILE)
        .map_err(|_| PlnkError::BackupError("cannot read host file".to_string()))?;
    
    write(HOSTS_BACKUP, content)
        .map_err(|_| PlnkError::BackupError("cannot create backup file".to_string()))?;
    
    Ok(())
}


fn restore_hosts() -> Result<(), PlnkError<String>> {
    if !Path::new(HOSTS_BACKUP).exists() {
        return Err(PlnkError::BackupError("Cannot read host file".to_string()));
    }

    rename(HOSTS_BACKUP, HOSTS_FILE)
        .map_err(|_| PlnkError::BackupError("Cannot create backup file".to_string()))?;

    println!("URLs unblocked successfully");
    Ok(())
}

fn is_blocked_line(line: &str, domains: &[String]) -> bool {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 2 || parts[0] != URL_TO_REDIRECT {
        return false;
    }

    let host = parts[1];
    
    domains.iter().any(|d| d == host)
}

fn check_already_blocked(domains: &[String]) -> Result<bool, PlnkError<String>> {
    let file = fs::File::open(HOSTS_FILE)
        .map_err(|_| PlnkError::HostsError("cannot read hosts file".to_string()))?;

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line_current = line
            .map_err(|_| PlnkError::HostsError("Error reading line from hosts file".to_string()))?;

        if is_blocked_line(&line_current, domains) {
            return Ok(true);
        }
    }

    Ok(false)
}

fn block_domains(domains: &[String]) -> Result<(), PlnkError<String>> {
    if domains.is_empty() {
        return Err(PlnkError::IllegalState("No domain to block".to_string()))?;
    }

    if check_already_blocked(domains)? {
        return Err(PlnkError::IllegalState("URLs already blocked".to_string()))?;
    }

    backup_hosts()?;

    let mut hosts_file = fs::OpenOptions::new()
        .append(true)
        .open(HOSTS_FILE)
        .map_err(|_| PlnkError::HostsError("Cannot write to hosts file".to_string()))?;

    writeln!(hosts_file, "\n{}", PLNK_MARKER)
        .map_err(|_| PlnkError::HostsError("Failed to write marker".to_string()))?;

    for domain in domains {
        writeln!(hosts_file, "{} {}", URL_TO_REDIRECT, domain)
            .map_err(|_| PlnkError::HostsError("Failed to write domain entry".to_string()))?;
    }

    println!("URLs blocked successfully");
    Ok(())
}

fn usage() {
    eprintln!("Usage: plnk [OPTION]");
    eprintln!("Block or unblock URLs by modifying /etc/hosts");
    eprintln!("");
    eprintln!("Options:");
    eprintln!("  u,            Restore original hosts file");
    eprintln!("  h, help       Show this help message");
    eprintln!("  (no args)     Block URLs from config");
    exit(1)
}

fn check_root() -> Result<(), PlnkError<String>> {
    #[cfg(unix)]
    {
        if let Err(_) = fs::OpenOptions::new().append(true).open(HOSTS_FILE) {
            return Err(PlnkError::PermissionError("must run as root".to_string()));
        }
        Ok(())
    }
}

fn load_config() -> Result<Config, PlnkError<String>> {
    let config_home_path = var(HOME_ENV)
        .map_err(|_| PlnkError::Config(format!("Environment variable {} not set", HOME_ENV)))?;

    let config_path = format!("{}/{}", config_home_path, PLNK_PARTIAL_PATH);
    
    let content = read_to_string(&config_path)
        .map_err(|_| PlnkError::Config(format!("Failed to read config file: {}", config_path)))?;

    let config: Config = toml::from_str(&content)
        .map_err(|err| PlnkError::Io(format!("Failed to parse config: {}", err)))?;

    for domain in &config.blocked_domains {
        if domain.trim().is_empty() {
            return Err(PlnkError::Config("Empty domain found in config".to_string()));
        }
    }

    Ok(config)
}

fn run() -> Result<(), PlnkError<String>> {
    check_root()?;


    let args: Vec<String> = args().collect();

    match args.get(1).map(|s: &String| s.as_str()) {
        Some("u") => {
            restore_hosts()?;
        }
        Some("h") | Some("help") => {
            usage();
        }
        Some(unknown) => {
            eprintln!("Unknown option: {}", unknown);
            usage();
            return Err(PlnkError::IllegalState("Invalid argument".to_string()));
        }
        None => {
            let config = load_config()?;
            block_domains(&config.blocked_domains)?;
        }
    }
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        die(&err);
    }
}
