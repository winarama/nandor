#![allow(dead_code)]

use std::io;
use std::env;
use std::io::Write;
use std::error::Error;
use colored::Colorize;
use std::process::{Command, Stdio};
use reqwest::blocking::{Client, Response};

//  example url >> https://www.ipdeny.com/ipblocks/data/countries/af.zone
static IPDENY_URL: &str=  "https://www.ipdeny.com/ipblocks/data/countries/";

pub fn run(country_codes: Vec<String>) {
    for country in country_codes.iter().skip(2) {
        let res = fetch_ip_block(country).unwrap();
        let blocks = res.split("\n");
        for block in blocks {
            if block.len() > 0 {
                print!("{} {} {} {}", "Nandor:".cyan(), "Attempting to block".green(), block.red(), ">>".cyan());
                io::stdout().flush().unwrap();
                if update_firewall_rules(&block) {
                    print!(" {}\n", "Success!".green());
                    io::stdout().flush().unwrap();
                } else {
                    print!(" {}\n", "Failure!".red());
                    io::stdout().flush().unwrap();
                }
            }
        }
        if reload_firewall() {
            println!("{} {} {} {}", "Nandor:".cyan(), "Firewall successfully reloaded. All of ".green(), country.to_uppercase().red(), "has been blocked.".green());
        } else {
            println!("{} {} {} {}", "Nandor:".cyan(), "Firewall not reloaded. All of ".red(), country.to_uppercase().red(), "has not been blocked.".red());
        }
    }
}

/*
 *  firewall-cmd --version
 *  firewall-cmd --permanent --add-rich-rule="rule family='ipv4' source address='115.239.228.12/24' reject"
 *  firewall-cmd --reload
 * 
 *  ufw --version
 *  ufw deny from 115.239.228.12/24
 *  ufw reload   
 */
fn update_firewall_rules(cidr: &str) -> bool {
    let output = Command::new("ufw")
    .arg("deny")
    .arg("from")
    .arg(cidr)
    .output()
    .expect("Nandor: Oh no, I releneted.");
    return output.status.success()
}

fn reload_firewall() -> bool {
    let output = Command::new("ufw")
    .arg("reload")
    .output()
    .expect("Nandor: Oh no, I releneted.");
    return output.status.success()
}

fn fetch_ip_block(code: &str) -> Result<String, Box<dyn Error>> {
    let body = reqwest::blocking::get(IPDENY_URL.to_owned() + code + ".zone")?.text()?;
    return Ok(body)
}