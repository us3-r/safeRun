use std::collections::HashSet;
use std::fs;
use regex::Regex;
use crate::{Config, Patterns};

enum ProcessedResult {
    VecResult(Vec<String>),
    HashSetResult(HashSet<String>),
}

fn handle_lines(file_content: &str, return_hash_set: bool) -> ProcessedResult {

    let content_lines = file_content.lines();
    if return_hash_set {
        let lines: HashSet<String> = content_lines
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with('#')) // Ignore empty lines and comments
            .map(|line| {
                if line.contains(" # "){
                    line.split(" # ").collect::<Vec<&str>>()[0]
                }
                else {
                    line
                }
            })
            .filter(|line| !line.is_empty())
            .map(String::from)
            .collect();
        ProcessedResult::HashSetResult(lines)
    }
    else {
        let lines = content_lines
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with('#')) // Ignore empty lines and comments
            .map(|line| {
                if line.contains(" # ") {
                    line.split(" # ").collect::<Vec<&str>>()[0]
                } else {
                    line
                }
            })
            .filter(|line| !line.is_empty())
            .map(String::from)
            .collect();
        ProcessedResult::VecResult(lines)
    }
}


/// Function to read the ignore file and return a list of paths to ignore
/// # Arguments
/// * `ignore_file` - path to the ignore file
/// # Returns
/// * `HashSet` - set of paths to ignore
pub fn get_ignored_paths(ignore_file: &str) -> HashSet<String> {
    let content = match fs::read_to_string(ignore_file){
        Ok(content) => content,
        Err(e) => {
            panic!("Error reading ignore file: {}", e)
        }
    };
    let ignore_files = handle_lines(&content, true);
    match ignore_files {
        ProcessedResult::HashSetResult(ignore_files) => ignore_files,
        _ => panic!("Error processing ignore file")
    }
}

/// Function to read the pattern file and return a list of patterns
/// # Arguments
/// * `file_path` - path to the pattern file
/// # Returns
/// * `Patterns` - struct of list of patterns
pub fn make_pattern_list(file_path: &str) -> Patterns {
    let pattern_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            panic!("Error reading pattern file: {}", e)
            //     mby change and add some internal patterns which will be run in case of error
        }
    };

    let patterns = handle_lines(&pattern_content, false);
    match patterns {
        ProcessedResult::VecResult(patterns) => {
            println!("Patterns: {:?}", &patterns);
            Patterns {
                patterns,
            }
        },
        _ => panic!("Error processing pattern file")
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
pub fn find_matches(config: &Config, file: &str, patterns: &Vec<String>) -> crate::Result {
    // initialize a vector for all the matches
    let mut matches = Vec::new();
    let mut h = false;
    let mut m = false;
    let know_patterns:Vec<&str> = vec![
        r"((\[(A.*|a.*)\]\{\d+\}){1}|.*PRIVATE KEY|^ey)",
        r".*\[0.*\].*\{\d+\}"
    ];
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
                                let mut severity = -1;
                                for (i, p) in know_patterns.iter().enumerate() {
                                    let re = Regex::new(p).unwrap();
                                    if re.is_match(&pattern[1..]) {
                                        severity = i as i32;
                                        break;
                                    }
                                }
                                let line = format!("\t{}", line.trim());
                                match severity {

                                    0 => {
                                        h = true;
                                        println!("\x1b[0;31;1m\t   line: {}|{}\x1b[0m",line_number, line)
                                    },
                                    1 => {
                                        m = true;
                                        println!("\x1b[0;33;1m\t   line: {}|{}\x1b[0m",line_number, line)
                                    },
                                    _ => println!("\x1b[0;34;1m\t   line: {}|{}\x1b[0m",line_number, line),
                                }
                                // println!("{}: {}", line_number, line);
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
                                // Check for severity of the pattern (with regex)
                                let mut severity = -1;
                                for (i, p) in know_patterns.iter().enumerate() {
                                    let re = Regex::new(p).unwrap();
                                    if re.is_match(&pattern[1..]) {
                                        severity = i as i32;
                                        break;
                                    }
                                }
                                let line = format!("\t{}", line.trim());
                                match severity {

                                    0 => {
                                        h = true;
                                        println!("\x1b[0;31;1m\t   line: {}|{}\x1b[0m",line_number, line)
                                    },
                                    1 => {
                                        m = true;
                                        println!("\x1b[0;33;1m\t   line: {}|{}\x1b[0m",line_number, line)
                                    },
                                    _ => println!("\x1b[0;34;1m\t   line: {}|{}\x1b[0m",line_number, line),
                                }
                                // println!("{}: {}", line_number, line);
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
    crate::Result {
        matches,
        high: h,
        mid: m,
    }
}

pub fn att(n:&usize) -> &'static str {
    let mut att:&str=":(";
    if n == &0 {
        att = "~ WOOOOOOOOOOO ~";
    }
    else if n < &5 {
        att = ":)";
    }
    else if n < &10 {
        att = ":|";
    }
    else if n > &10 && n < &20{
        att = ":(";
    }
    else {
        att = " WAT IN THE GODS NAME HAVE YOU DONE ";
    }
    att
}