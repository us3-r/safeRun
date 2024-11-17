use std::fmt::Arguments;
use clap::builder::Str;
use regex::Regex;
use crate::{structs::{FoundMatchesResult, Settings}, Config, PatternVS};
use serde_json;
use std::{
    cmp::max,
    fs,
    io::{self, Write},
};


/// Fun function
pub fn att(n: &usize) -> &'static str {
    let att: &str;
    if *n == 0 {
        att = "~ WOOOOOOOOOOO ~";
    } else if *n < 5 {
        att = ":)";
    } else if *n < 10 {
        att = ":|";
    } else if *n > 10 && *n < 20 {
        att = ":(";
    } else {
        att = " WAT IN THE GODS NAME HAVE YOU DONE ";
    }
    att
}


/// Custom function to print with/without color (replace print!())
/// ### Arguments
/// #### `color` - bool to print with color or not
/// #### `color_code` - color code to print with
/// #### `format` - stuff to be printed 
/// ```
/// [example: println!("{} {}", a, b) -> format_args!("{} {}", a, b)]
/// ```
/// 
pub fn custom_print(color: bool, color_code: &str, format: Arguments){
    if color {
        print!("\x1b[{}m{}\x1b[0m", color_code, format);
    } else {
        print!("{}", format);
    }
}

/// Custom function to print with/without color (replace println!())
/// ### Arguments
/// #### `color` - bool to print with color or not
/// #### `color_code` - color code to print with
/// #### `format` - stuff to be printed 
/// ```
/// [example: println!("{} {}", a, b) -> format_args!("{} {}", a, b)]
/// ```
/// 
pub fn custom_println(color: bool, color_code: &str, format: Arguments){
    if color {
        println!("\x1b[{}m{}\x1b[0m", color_code, format);
    } else {
        println!("{}", format);
    }
}

/// Function to check if the file ends with a blank line
/// ### Arguments
/// #### `file` - file to check
/// 
/// ### Returns
/// #### `bool` - true if the file ends with a blank line, false otherwise
/// 
pub fn ends_with_blank_line(file: &str) -> bool {
    // println!("{:?}", file.lines().last().unwrap().trim());
    if let Some(last_line) = file.lines().last() {
        return last_line.trim().is_empty();
    }
    false
}

/// Function to load settings from a file
/// ### Arguments
/// #### `settings_path` - path to the settings file
/// 
/// ### Returns
/// #### `Settings` - settings loaded from the file
/// 
pub fn load_settings(settings_path: &str) -> Settings {
    let settings_data = fs::read_to_string(settings_path).expect("Error reading settings file");
    serde_json::from_str(&settings_data).expect("Error parsing settings file")
}

/// Function to load patterns from settings
/// ### Arguments
/// #### `settings` - settings to load patterns from
/// 
/// ### Returns
/// #### `Vec<PatternVS>` - vector of patterns
/// 
pub fn load_patterns(settings: &Settings) -> Vec<PatternVS> {
    let mut patterns: Vec<PatternVS> = Vec::new();

    // Combine high severity patterns
    for pattern in &settings.patterns.severity.h {
        patterns.push(PatternVS {
            pattern: pattern.pattern.clone(),
            comment: pattern.comment.clone(),
            regex: pattern.regex,
            severity: 1,  // High severity
        });
    }

    // Combine medium severity patterns
    for pattern in &settings.patterns.severity.m {
        patterns.push(PatternVS {
            pattern: pattern.pattern.clone(),
            comment: pattern.comment.clone(),
            regex: pattern.regex,
            severity: 2,  // Medium severity
        });
    }

    // Combine low severity patterns (if any)
    for pattern in &settings.patterns.severity.l {
        patterns.push(PatternVS {
            pattern: pattern.pattern.clone(),
            comment: pattern.comment.clone(),
            regex: pattern.regex,
            severity: 3,  // Low severity
        });
    }

    patterns
}


/// Function to find matches in a file
/// ### Arguments
/// #### `line_` - line to search
/// #### `patterns` - list of patterns to search
/// 
/// ### Returns
/// #### `Option<FoundMatchesResult>` - struct of lists of matches
/// 
pub fn find_matches(line_: &str, patterns: &Vec<PatternVS>) -> Option<FoundMatchesResult> {

    for current_pattern in patterns {
        let pattern = &current_pattern.pattern;
        let is_regex = current_pattern.regex;

        if is_regex {
            let re = Regex::new(pattern).unwrap();

            if re.is_match(line_) {
                return Some(FoundMatchesResult {
                    // line_: line_.to_string(),
                    pattern_: pattern.to_string(),
                    severity_: current_pattern.severity,
                });
            }
        } else {
            if line_.contains(pattern) {
                return Some(FoundMatchesResult {
                    // line_: line_.to_string(),
                    pattern_: pattern.to_string(),
                    severity_: current_pattern.severity,
                });
            }
        }
    }   

    None

}