use clap::{Parser};
use walkdir::WalkDir;
use std::fs;
use regex::Regex;
use std::collections::HashSet;
use std::cmp::max;

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
}
impl Result {
    fn clear(&mut self) {
        self.matches.clear();
    }
}

///Program to search for patterns in files

#[derive(Parser, Debug)]
#[command(name = "safeRun")]
#[command(version = "0")]
#[command(author = "us3-r")]
#[command(about = "Searches for phrases in files to find any sensitive data you might have left in your code")]
struct Args {
    /// Sets the path to search
    #[arg(short, long, default_value = ".")]
    path: String,

    /// Sets the pattern to search (text file)
    #[arg(short='r', long, default_value = "patterns.txt")]
    pattern: String,

    /// sets what files and folders to ignore
    #[arg(short, long, default_value = "ignore.txt")]
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
    let getter = make_pattern_list(&config.pattern.as_str());
    println!("\n+{:-<width$}+", "", width = 26+config.path.len());
    println!("Searching for patterns in {}", config.path);
    println!("+{:-<width$}+\n", "", width = 26+config.path.len());
    let patterns = &getter.patterns;
    let ignore_list = get_ignored_paths(&config.ignore);
    let mut apperance = 0;
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
                let mut result = find_matches(&config, &file, &patterns);
                let max_length = patterns.iter().map(|p| p.len())
                    .max().unwrap_or(0);
                let max_vec = result.matches.iter().map(|m| m.len())
                    .max().unwrap_or(0).to_string().len();

                let max_above = max(12, max_vec) + max_length;

                // print vector matches
                if result.matches.iter().map(|m| m.len()).sum::<usize>() > 0{
                    println!("\n|=| {}", path.display());
                    for (i, pattern) in patterns.iter().enumerate() {
                        match result.matches.get(i) {
                            Some(matches) => {
                                if !matches.is_empty() {
                                    println!("\t| {:width$} : {}", pattern, matches.join(", "), width = max_length);
                                    apperance += result.matches[i].len();
                                }
                            },
                            None => {},
                        }
                    }
                    result.clear();
                    println!("\n[ {:=<width$} ]", "", width = max_above);
                }else{
                    continue;
                }
            }
        }
        println!();
    }
    println!("Done , found {} matches", apperance);
}

fn parse_args() -> Config {
    let args = Args::parse();
    let path = args.path;
    let pattern = args.pattern;
    let ignore = args.ignore;
    let fast = args.fast;
    let show = args.show_lines;
    // return Config ( do not use ';' after the expression you want to return )
    Config {
        path: path.to_string(),
        pattern: pattern.to_string(),
        ignore: ignore.to_string(),
        fast,
        show,
    }
}

/// Function to read the ignore file and return a list of paths to ignore
/// # Arguments
/// * `ignore_file` - path to the ignore file
/// # Returns
/// * `HashSet` - set of paths to ignore
fn get_ignored_paths(ignore_file: &str) -> HashSet<String> {
    let content = match fs::read_to_string(ignore_file){
        Ok(content) => content,
        Err(e) => {
            panic!("Error reading ignore file: {}", e)
        }
    };
    let lines = content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#')) // Ignore empty lines and comments
        .map(String::from)
        .collect();
    lines
}

/// Function to read the pattern file and return a list of patterns
/// # Arguments
/// * `file_path` - path to the pattern file
/// # Returns
/// * `Patterns` - struct of list of patterns
fn make_pattern_list(file_path: &str) -> Patterns {
    let pattern_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            panic!("Error reading pattern file: {}", e)
        //     mby change and add some internal patterns which will be run in case of error
        }
    };

    let patterns = pattern_content.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(String::from)
        .collect();

    Patterns {
        patterns,
    }
}

/// Function to find matches in a file
/// # Arguments
/// * `config` - Configuration for the search
/// * `file` - path to the file to search
/// * `patterns` - list of patterns to search for
/// # Returns
/// * `Result` - struct of lists of matches
/// # Example
/// ```
/// Result {
///    matches: vec![vec![("pattern1","line"),("pattern1","line")],vec![("pattern2","line"),("pattern2","line")]],...]
/// }
/// ```
fn find_matches(config: &Config, file: &str, patterns: &Vec<String>) -> Result {
    // initialize a vector for all the matches
    let mut matches = Vec::new();
    for pattern in patterns {
        let mut found = Vec::new();

        // checks if the pattern is a form of regex or a simple string
        match pattern.chars().next() {
            Some('$') => {  // REGEX
                let mut line_number = 1;
                let re = Regex::new(&pattern[1..]).unwrap();

                // if fast is not set, go through all the lines else just finds if it contains the pattern
                if !config.fast {
                    for line in file.lines() {
                        if re.is_match(line) {
                            if config.show {
                                println!("{}: {}", line_number, line);
                            }
                            found.push(line_number.to_string());
                        }
                        line_number += 1;
                    }
                }else {
                    for line in file.lines() {
                        if re.is_match(line) {
                            found.push(line_number.to_string());
                            break;
                        }
                    }
                }
                matches.push(found);
            }

            Some('"') => { // STRING
                let mut line_number = 1;
                // if fast is not set, go through all the lines else just finds if it contains the pattern
                if !config.fast {
                    for line in file.lines() {
                        if line.contains(&pattern[1..]) {
                            if config.show {
                                println!("{}: {}", line_number, line);
                            }
                            found.push(line_number.to_string());
                        }
                        line_number += 1;
                    }
                }else {
                    for line in file.lines() {
                        if line.contains(&pattern[1..]) {
                            found.push(line_number.to_string());
                            break;
                        }
                    }
                }
                matches.push(found);

            }
            _ => {}
        }
    };
    Result {
        matches,
    }
}