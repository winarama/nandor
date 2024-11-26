#![allow(unused_imports)]
#![allow(dead_code)]

use std::env;
use std::fs;
use std::fs::File;
use colored::Colorize;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, LineWriter};

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
    let log_out = File::create("log.out").unwrap();
    let mut child = Command::new("journalctl").arg("-n").arg("1000").arg("-u") .arg(service).stdout(log_out).spawn().expect("Nandor: Oh no, I releneted.");
    let _res = child.wait().unwrap();
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
    let mut child = Command::new("iptables").arg("-A").arg("INPUT").arg("-s").arg(ip).arg("-j").arg("DROP").spawn().expect("Nandor: Oh no, I releneted.");
    let _res = child.wait().unwrap();
}

fn clean_up() {
    Command::new("iptables-save").stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()).spawn().expect("Nandor: Oh no, I releneted.");
    fs::remove_file("log.out").unwrap();
}
