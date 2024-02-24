use std::collections::HashSet;
use std::fs;
use regex::Regex;
use crate::{Config, Patterns};

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
pub fn make_pattern_list(file_path: &str) -> Patterns {
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
pub fn find_matches(config: &Config, file: &str, patterns: &Vec<String>) -> crate::Result {
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
    crate::Result {
        matches,
    }
}