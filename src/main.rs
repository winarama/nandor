#![allow(unused_imports)]
#![allow(dead_code)]

use std::env;
use std::fs;
use std::fs::File;
use colored::Colorize;
use std::process::Command;
use std::io::{BufRead, BufReader, LineWriter};

static CYAN_BOLD_BRIGHT: &str = "\x1b33[1;96m";
static GREEN_BOLD_BRIGHT: &str = "\x1b33[1;92m";
static RESET: &str = "\x1b33[0m";

fn main() {
    println!("{} {}", "Nandor:".cyan(), "I am Nandor the Relentless! I must protect the borders of Al Qolnidar!".green());    
    let args: Vec<String> = env::args().collect();

    generate_log_output(&args[1]);
    let mut script_kiddies: Vec<String> = Vec::new();

    let file = File::open("log.out").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let res = parse_line(&line.unwrap());
        if res.is_some() {
            let vec = res.unwrap();
            if is_script_kiddie(&vec[1]) {
                let ip = String::from(&vec[0]).replace("IP>>", "");
                if !script_kiddies.contains(&ip) {
                    script_kiddies.push(ip);
                }
            }
        }
    }

    for ip in script_kiddies.iter() {
        block_script_kiddie(&ip);
        println!("{} {} {}", "Nandor:".cyan(), "I have blocked >>".green(), ip.red());
    }

    clean_up();
}

fn generate_log_output(service: &str) {
    Command::new("journalctl")
        .arg("-n")
        .arg("1000")
        .arg("-u")
        .arg(service)
        .arg(">")
        .arg("log.out")
        .spawn()
        .expect("Nandor: Oh no, I releneted.");
}

fn parse_line(line: &str) -> Option<Vec<String>> {
    let start_index: Option<usize> = line.find("IP");  //  Find the position of keyword
    if start_index.is_some() {
        let section: &str = &line[start_index.unwrap()..line.len()];
        let vec: Vec<&str> = section.split("|").collect();
        let mut res: Vec<String> = Vec::new();
        res.push(String::from(vec[0]).replace(" ", ""));
        res.push(String::from(vec[1]).replace(" ", ""));
        return Some(res)
    }
    return None
}

fn is_script_kiddie(log_entry: &str) -> bool {
    let mut res = false;
    let file = File::open("script_kiddie_lingo").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if log_entry.contains(&line.unwrap()) {
            res = true;
            break;
        }
    }
    return res
}

fn block_script_kiddie(ip: &str) {
    Command::new("iptables")
        .arg("-A")
        .arg("INPUT")
        .arg("-s")
        .arg(ip)
        .arg("-j")
        .arg("DROP")
        .spawn()
        .expect("Nandor: Oh no, I releneted.");

    Command::new("iptables-save")
        .arg("&>/dev/null")
        .spawn()
        .expect("Nandor: Oh no, I releneted.");
}

fn clean_up() {
    fs::remove_file("log.out").unwrap();
}
