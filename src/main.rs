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
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "cannot read /etc/hosts"))?;
    
    fs::write(HOSTS_BACKUP, content)
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "cannot create backup file"))?;
    
    Ok(())
}

fn restore_hosts() {
    if !file_exists(HOSTS_BACKUP) {
        die("no backup file found");
    }

    if let Err(_) = fs::rename(HOSTS_BACKUP, HOSTS_FILE) {
        die("failed to restore hosts file");
    }

    println!("urls unblocked");
}

fn is_blocked_line(line: &str, site_to_block: &Vec<String>) -> bool {
    for domain in site_to_block {
        if line.contains(domain) {
            return true;
        }
    }
    false
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

fn block_sites(site_to_block: &Vec<String>) {
    let already_blocked = check_already_blocked(site_to_block);

    if already_blocked {
        println!("url already blocked");
        return;
    }

    // Backup original hosts file
    if let Err(e) = backup_hosts() {
        die(&format!("backup failed: {}", e));
    }

    // Append blocked domains
    let mut hosts_file = fs::OpenOptions::new()
        .append(true)
        .open(HOSTS_FILE)
        .unwrap_or_else(|_| die(&format!("cannot write to {}", HOSTS_FILE)));

    writeln!(hosts_file, "\n# plnk url blocking")
        .unwrap_or_else(|_| die(&format!("failed to write to {}", HOSTS_FILE)));

    for domain in site_to_block {
        writeln!(hosts_file, "{} {}", URL_TO_REDIRECT, domain)
            .unwrap_or_else(|_| die("failed to write to hosts file"));
    }

    println!("urls blocked");
}

fn usage() -> ! {
    let argv0 = env::args().next().unwrap_or_else(|| "program".to_string());
    eprintln!("usage: {} [-u]", argv0);
    eprintln!("  -u  unblock (restore original hosts file)");
    process::exit(1);
}

fn check_root() {
    #[cfg(unix)]
    {
        if let Err(_) = fs::OpenOptions::new().append(true).open(HOSTS_FILE) {
            die("must run as root");
        }
    }
}

fn load_config() -> Config {
    let content = fs::read_to_string(CONFIG_FILE)
        .unwrap_or_else(|_| {
            die(format!("Failed to read {}", CONFIG_FILE).as_str());
        });

    let config: Config = toml::from_str(&content).unwrap_or_else(|err| {
       die(format!("Failed to parse {}: {}", CONFIG_FILE, err).as_str());
    });

    config
}

fn main() {
    let site_to_block = load_config();
    let args: Vec<String> = env::args().collect();
    
    check_root();

    match args.len() {
        2 if args[1] == "-u" => {
            restore_hosts();
        }
        1 => {
            block_sites(&site_to_block.blocked_domains);
        }
        _ => {
            usage();
        }
    }
}
