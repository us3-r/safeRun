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
    let mut first_match = true;
    let mut written_lines = Vec::new();

    for pattern in patterns {
        let mut found = Vec::new();

        // checks if the pattern is a form of regex or a simple string
        match pattern.chars().next() {
            Some('$') => {  // REGEX
                let mut line_number = 1;
                let re = Regex::new(&pattern[2..]).unwrap();

                // if fast is not set, go through all the lines else just finds if it contains the pattern
                if !config.fast {
                    for line in file.lines() {
                        if re.is_match(line) {
                            if first_match {
                                print!("\n");
                                first_match = false;
                            }
                            if config.show && !written_lines.contains(&line_number) {
                                let severity = pattern.chars().nth(1).unwrap().to_digit(10).unwrap() as i32;

                                let line = format!("\t{}", line.trim());
                                match severity {

                                    1 => {
                                        h = true;
                                        println!("\x1b[0;31;1m\t   line: {}|{}\x1b[0m",line_number, line)
                                    },
                                    2 => {
                                        m = true;
                                        println!("\x1b[0;33;1m\t   line: {}|{}\x1b[0m",line_number, line)
                                    },
                                    _ => println!("\x1b[0;34;1m\t   line: {}|{}\x1b[0m",line_number, line),
                                }
                                // println!("{}: {}", line_number, line);
                            }
                            found.push(line_number.to_string());
                            written_lines.push(line_number);
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
                        if line.contains(&pattern[2..]) {
                            if config.show && !written_lines.contains(&line_number){
                                // Check for severity of the pattern (with regex)
                                let severity = pattern.chars().nth(1).unwrap().to_digit(10).unwrap() as i32;

                                let line = format!("\t{}", line.trim());
                                match severity {

                                    1 => {
                                        h = true;
                                        println!("\x1b[0;31;1m\t   line: {}|{}\x1b[0m",line_number, line)
                                    },
                                    2 => {
                                        m = true;
                                        println!("\x1b[0;33;1m\t   line: {}|{}\x1b[0m",line_number, line)
                                    },
                                    _ => println!("\x1b[0;34;1m\t   line: {}|{}\x1b[0m",line_number, line),
                                }
                                // println!("{}: {}", line_number, line);
                            }
                            found.push(line_number.to_string());
                            written_lines.push(line_number);
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
        // print values of the found vector


    };
    //




    crate::Result {
        matches,
        high: h,
        mid: m,
    }
}

pub fn att(n:&usize) -> &'static str {
    let  att:&str;
    if *n == 0 {
        att = "~ WOOOOOOOOOOO ~";
    }
    else if *n < 5 {
        att = ":)";
    }
    else if *n < 10 {
        att = ":|";
    }
    else if *n > 10 && *n < 20{
        att = ":(";
    }
    else {
        att = " WAT IN THE GODS NAME HAVE YOU DONE ";
    }
    att
}

// TODO: Might be faster than the current implementation but smthing wired is happening (chineese symbols)
// pub fn find_matches_test(config: &Config, file: &str, patterns: &Vec<String>) -> crate::Result {
//     let mut matches = vec![Vec::new(); patterns.len()];
//     let mut high = false;
//     let mut mid = false;
//
//     // Pre-compile all regex patterns (including those for severity checking)
//     let regex_patterns: Vec<_> = patterns.iter().map(|p| Regex::new(&p[1..]).unwrap()).collect();
//     let known_patterns = vec![
//         Regex::new(r"((\[(A.*|a.*)]\{\d+})|.*PRIVATE KEY|^ey)").unwrap(),
//         Regex::new(r".*\[0.*].*\{\d+}").unwrap(),
//     ];
//
//     // Process each line of the file once
//     file.lines().enumerate().for_each(|(line_number, line)| {
//         regex_patterns.iter().enumerate().for_each(|(i, regex)| {
//             if regex.is_match(line) {
//                 if config.fast {
//                     matches[i].push(line_number.to_string());
//                     return;
//                 } else {
//                     // Determine severity and log accordingly
//                     let severity = known_patterns.iter().enumerate().find_map(|(idx, pat)| {
//                         if pat.is_match(line) { Some(idx) } else { None }
//                     });
//
//                     if config.show {
//                         let line_display = format!("\t{}", line.trim());
//                         match severity {
//                             Some(0) => {  // High severity
//                                 high = true;
//                                 println!("\x1b\n[0;31;1m\t   line: {}|{}\x1b[0m", line_number, line_display);
//                             },
//                             Some(1) => {  // Medium severity
//                                 mid = true;
//                                 println!("\x1b\n[0;33;1m\t   line: {}|{}\x1b[0m", line_number, line_display);
//                             },
//                             _ => println!("\x1b\n[0;34;1m\t   line: {}|{}\x1b[0m", line_number, line_display),
//                         }
//                     }
//                     matches[i].push(line_number.to_string());
//                 }
//             }
//         });
//     });
//
//     // Construct the result with the gathered matches and severity information
//     crate::Result {
//         matches,
//         high,
//         mid,
//     }
// }
