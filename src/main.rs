#![allow(unused_imports)]
#![allow(dead_code)]

mod geoblock;
mod logscan;

use std::env;
use colored::Colorize;

fn main() {
    println!("\n{} {}", "Nandor:".cyan(), "I am Nandor the Relentless! I must protect the borders of Al Qolnidar!".green());     
    let args: Vec<String> = env::args().collect();

    if &args[1] == "logscan" {
        logscan::run(&args[2]);
    } else if &args[1] == "geoblock" {
        geoblock::run(args);
    } else {
        println!("{} {}", "Nandor:".cyan(), "What are you on about Guillermo?".green());
    }
}
