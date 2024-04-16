mod utils;

use clap::{Parser};
use walkdir::WalkDir;
use std::{
    cmp::max,
    env,
    fs,
    io::{self, Write},
};

struct Config {
    path: String,
    pattern: String,
    ignore: String,
    fast: bool,
    show: bool,
}

struct Patterns {
    patterns: Vec<String>,
}

struct Result {
    matches: Vec<Vec<String>>,
    high: bool,
    mid: bool,
}
impl Result {
    fn clear(&mut self) {
        self.matches.clear();
    }
}

///Program to search for patterns in files

#[derive(Parser, Debug)]
#[command(name = "safeRun")]
#[command(version = "1.1")]
#[command(author = "us3-r")]
#[command(about = "Searches for phrases in files to find any sensitive data you might have left in your code")]
struct Args {
    /// Sets the path to search
    #[arg(short, long, required = true)]
    path: String,

    /// Sets the pattern to search (text file)
    #[arg(short='r', long, default_value = "None")]
    pattern: String,

    /// sets what files and folders to ignore
    #[arg(short, long, default_value = "None")]
    ignore: String,

    /// Only checks if patterns are present (not where)
    #[arg(short, long, default_value = "false")]
    fast: bool,

    /// prints the lines where the patterns are found
    #[arg(short, long, default_value = "false")]
    show_lines: bool,
}

fn main(){
    let config = parse_args();
    let getter = utils::make_pattern_list(&config.pattern.as_str());
    println!("\n+{:-<width$}+", "", width = 26+config.path.len());
    println!("Searching for patterns in {}", config.path);
    println!("+{:-<width$}+\n", "", width = 26+config.path.len());
    let patterns = &getter.patterns;
    let ignore_list = utils::get_ignored_paths(&config.ignore);
    let mut apperance = 0;
    let mut max_length = 0;

    if config.show{
        let mut filenames: Vec<String> = Vec::new();
        for entry in WalkDir::new(&config.path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                filenames.push(entry.path().display().to_string());
            }
        }
        max_length = filenames.iter().map(|s| s.len()).max().unwrap_or(0);
    }
    else {
        max_length = 5;
    }

    for entry in WalkDir::new(&config.path){
        let entry = entry.unwrap();
        let path = entry.path();
        let path_str = path.display().to_string();
        let contains = ignore_list.iter().any(|i| path_str.contains(i.as_str()));
        if contains {
            continue;
        }else {
            if path.is_file() {
                let file = match fs::read(path) {
                    Ok(content) => {
                        let val = String::from_utf8_lossy(&content).to_string();
                        val
                    }
                    Err(e) => {
                        panic!("Error reading file: {}", e)
                    }
                };
                print!(" |=| {:width$}\x1b[0m", path.display(), width=max_length-(max_length*2/3));
                io::stdout().flush().unwrap();
                let mut result = utils::find_matches(&config, &file, &patterns);
                let max_length = patterns.iter().map(|p| p.len())
                    .max().unwrap_or(0);
                let max_vec = result.matches.iter().map(|m| m.len())
                    .max().unwrap_or(0).to_string().len();

                let max_above = max(12, max_vec) + max_length;

                // print vector matches
                if result.matches.iter().map(|m| m.len()).sum::<usize>() > 0{
                    for (i, pattern) in patterns.iter().enumerate() {
                        match result.matches.get(i) {
                            Some(matches) => {
                                if !matches.is_empty() {
                                    print!("\n\t\x1b[0;30;1m| {:width$} : {}", pattern, matches.join(", "), width = max_length);
                                    apperance += result.matches[i].len();
                                }
                            },
                            None => {},
                        }
                    }
                    if result.high | result.mid {
                        println!("\n\x1b[0;42;37;1m\n[!!!] FOUND SEVERITY RISKS: \x1b[0m");
                    }
                    if result.high {
                        println!("\n\x1b[0;31;1m[ HIGH ] It seems like a highly privileged information has been left in the code base\n\t Change the way you implemented such data !\x1b[0m");
                    }
                    if result.mid {
                        println!("\n\x1b[0;33;1m[ MID ] Some data has been left in the code that mby should not be in it?\n\tConsider an alternative way to display such data or remove it\x1b[0m");
                    }
                    result.clear();
                    println!("\n\x1b[0;30;1m[ {:=<width$} ]\x1b[0m", "", width = max_above);
                }else{
                    print!("\x1b[0;32;1m {:s$}OK\n\x1b[0m", "", s = 10);
                    // println!("\x1b[0m");
                }
            }
        }
    }
    println!("\n\x1b[0;35;1m+{:-<width$}+", "", width = 26+config.path.len());
    println!("Done , found {} matches ... {} ", apperance, utils::att(&apperance));
    println!("+{:-<width$}+", "", width = 26+config.path.len());
    println!("\x1b[0m");
}

fn parse_args() -> Config {
    let args = Args::parse();
    let path = args.path;
    let mut pattern = args.pattern;
    let mut ignore = args.ignore;
    let fast = args.fast;
    let show = args.show_lines;
    // return Config ( do not use ';' after the expression you want to return )
    if &pattern == "None" {
        pattern = env::current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("patterns.txt").to_str().unwrap().to_string();
    }
    if &ignore == "None" {
        ignore = env::current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().join("ignore.txt").to_str().unwrap().to_string();
    }
    Config {
        path: path.to_string(),
        pattern: pattern.to_string(),
        ignore: ignore.to_string(),
        fast,
        show,
    }
}
