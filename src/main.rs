
use reqwest;
use clap::{Parser, CommandFactory};
use colored::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// A single domain to scan
    #[arg(short, long)]
    domain: Option<String>,

    /// File with list of domains to scan
    #[arg(short, long)]
    file: Option<String>,
}

fn scan_domain(domain: &str) {
    let schemes = ["http://", "https://"];

    for scheme in &schemes {
        let base_url = format!("{}{}", scheme, domain);
        let git_url = format!("{}/.git/", &base_url);
        
        println!("Trying {}", git_url.bright_blue()); // Colored output

        match reqwest::blocking::get(&git_url) {
            Ok(response) => {
                println!("Response status: {}", response.status().to_string().green()); // Colored output
                
                if response.status().is_success() || response.status().as_u16() == 403 {
                    println!("Found .git/ folder at {}", git_url.bright_green());

                    let config_url = format!("{}/.git/config", &base_url);
                    println!("Trying {}", config_url.bright_blue()); // Colored output

                    match reqwest::blocking::get(&config_url) {
                        Ok(resp) => {
                            println!("Response status for config: {}", resp.status().to_string().green()); // Colored output

                            if resp.status().is_success() {
                                match resp.text() {
                                    Ok(config_content) => {
                                        println!("Contents of .git/config at {}:\n{}", base_url, config_content.bright_yellow());
                                    },
                                    Err(e) => println!("Failed to read .git/config: {}", e.to_string().red()),
                                }
                            } else {
                                println!("No .git/config found or access denied at {}", base_url.red());
                            }
                        },
                        Err(e) => println!("Error accessing .git/config at {}: {}", base_url, e.to_string().red()),
                    }

                    break;
                }
            },
            Err(e) => {
                println!("Error accessing {}: {}", git_url, e.to_string().red()); // Colored output
                continue;
            }
        }
    }
}

fn read_domains_from_file<P: AsRef<Path>>(file_path: P) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let buf_reader = io::BufReader::new(file);
    buf_reader.lines().collect()
}

fn main() {
    let args = Args::parse();

    match (&args.domain, &args.file) {
        (Some(domain), None) => scan_domain(domain),
        (None, Some(file_path)) => {
            match read_domains_from_file(file_path) {
                Ok(domains) => {
                    for domain in domains {
                        scan_domain(&domain);
                    }
                },
                Err(e) => eprintln!("Failed to read file: {}", e.to_string().red()),
            }
        },
        _ => {
            Args::command().print_help().expect("Failed to print help");
            std::process::exit(1);
        }
    }
}