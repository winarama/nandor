#![allow(unused_imports)]
#![allow(dead_code)]

use std::env;
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
            if update_firewall_rules(&block) {
                println!("{} {} {}", "Nandor:".cyan(), "I have blocked >>".green(), block.red());
            }
        }
        if reload_firewall() {
            println!("{} {} {}", "Nandor:".cyan(), "I have blocked all of >>".green(), country.to_uppercase().red());
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
    let status = Command::new("ufw")
    .arg("deny")
    .arg("from")
    .arg(cidr)
    .status()
    .expect("Nandor: Oh no, I releneted.");
    return status.success()
}

fn reload_firewall() -> bool {
    let status = Command::new("ufw")
    .arg("reload")
    .status()
    .expect("Nandor: Oh no, I releneted.");
    return status.success()
}

fn fetch_ip_block(code: &str) -> Result<String, Box<dyn Error>> {
    let body = reqwest::blocking::get(IPDENY_URL.to_owned() + code + ".zone")?.text()?;
    return Ok(body)
}