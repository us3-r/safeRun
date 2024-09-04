use crate::utils::{custom_print, custom_println};
use clap::Parser;
use serde_json;
use std::{
    cmp::max,
    fs,
    io::{self, Write},
};
use structs::{Config, PatternVS, Result, Settings};
use walkdir::WalkDir;

mod structs;
mod utils;
mod wsj;

///Program to search for patterns in files
#[derive(Parser, Debug)]
#[command(name = "safeRun")]
#[command(version = "1.2")]
#[command(author = "us3-r")]
#[command(
    about = "Searches for phrases in files to find any sensitive data you might have left in your code"
)]
struct Args {
    /// Sets the path to search
    #[arg(short, long, required = true)]
    path: String,

    /// Path to settings.json file
    #[arg(short = 's', long, required = true)]
    settings: String,

    /// Only checks if patterns are present (not where)
    #[arg(short, long, default_value = "false")]
    fast: bool,

    /// prints the lines where the patterns are found
    #[arg(short = 'l', long, default_value = "false")]
    show_lines: bool,
}

fn main() {
    let config = parse_args();

    let settings_data =
        fs::read_to_string(&config.settings).expect("Error reading settings file >> read");
    let settings: Settings =
        serde_json::from_str(&settings_data).expect("Error parsing settings file to struct");
    let severity = &settings.patterns.severity;
    let mut patterns: Vec<PatternVS> = Vec::new();
    let mut vector_for_report_whit_all = Vec::new();

    // put all pattern blocks into one vector
    patterns.extend(severity.h.iter().map(|p| PatternVS {
        pattern: p.pattern.clone(),
        comment: p.comment.clone(),
        regex: p.regex,
        severity: 1,
    }));
    patterns.extend(severity.m.iter().map(|p| PatternVS {
        pattern: p.pattern.clone(),
        comment: p.comment.clone(),
        regex: p.regex,
        severity: 2,
    }));
    patterns.extend(severity.l.iter().map(|p| PatternVS {
        pattern: p.pattern.clone(),
        comment: p.comment.clone(),
        regex: p.regex,
        severity: 3,
    }));

    let ignore_list = &settings.ignore;
    let project_path = if config.path == "j" {
        &settings.project_path
    } else {
        &config.path
    };
    let color = settings.run_settings.color_output;

    println!("\n+{:-<width$}+", "", width = 26 + project_path.len());
    println!("Searching for patterns in {}", project_path);
    println!("+{:-<width$}+\n", "", width = 26 + project_path.len());
    let mut appearance = 0;
    let max_length;

    if config.show {
        let mut filenames: Vec<String> = Vec::new();
        for entry in WalkDir::new(&project_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                filenames.push(entry.path().display().to_string());
            }
        }
        max_length = filenames.iter().map(|s| s.len()).max().unwrap_or(0);
    } else {
        max_length = 5;
    }

    for entry in WalkDir::new(&project_path) {
        let entry = entry.unwrap();
        let path = entry.path();
        let path_str = path.display().to_string();
        let contains = ignore_list.iter().any(|i| path_str.contains(i.as_str()));
        if contains {
            continue;
        } else {
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

                io::stdout().flush().unwrap();
                let mut result = utils::find_matches(&config, &file, &patterns, color);
                let max_length = patterns.iter().map(|p| p.pattern.len()).max().unwrap_or(0);
                let max_vec = result
                    .matches
                    .iter()
                    .map(|m| m.len())
                    .max()
                    .unwrap_or(0)
                    .to_string()
                    .len();

                let max_above = max(12, max_vec) + max_length;

                // print vector matches
                if result.matches.iter().map(|m| m.len()).sum::<usize>() > 0 {
                    custom_print(
                        color,
                        "0;37;0",
                        format_args!(
                            " |=| {:width$}",
                            path.display(),
                            width = max_length - (max_length * 2 / 3)
                        ),
                    );
                    custom_println(
                        color,
                        "0;0;0",
                        format_args!("\n\t| Patterns found: ... : lines"),
                    );
                    for (i, pattern) in patterns.iter().enumerate() {
                        match result.matches.get(i) {
                            Some(matches) => {
                                if !matches.is_empty() {
                                    vector_for_report_whit_all.push(vec![
                                        path.display().to_string(),
                                        matches[0][0].clone(),
                                        matches[0][1].clone(),
                                        pattern.pattern.clone(),
                                        if result.high {
                                            "high".to_string()
                                        } else if result.mid {
                                            "medium".to_string()
                                        } else {
                                            "low".to_string()
                                        },
                                    ]);
                                    custom_print (
                                        color,
                                        "0;34;1",
                                        format_args! (
                                            "\t| {:width$} : {}",
                                            pattern.pattern,
                                            matches[0].join(", "),
                                            width = max_length
                                        ),
                                    );
                                    appearance += result.matches[i].len();
                                }
                            }
                            None => {}
                        }
                    }
                    if result.high | result.mid {
                        custom_println(color, "5;41;1", format_args!("\n[!!!] RISKS FOUND:"));
                    }
                    if result.high {
                        custom_println(color, "0;31;1", format_args!("[ HIGH ] It seems like a highly privileged information has been left in the code base\n\t Change the way you implemented such data !"));
                    }
                    if result.mid {
                        custom_println(color, "0;33;1", format_args!("[ MID ] Some data has been left in the code that mby should not be in it?\n\tConsider an alternative way to display such data or remove it"));
                    }
                    result.clear();
                    if color {
                        custom_println(
                            color,
                            "0;30;1",
                            format_args!("\n[ {:=<width$} ]", "", width = max_above),
                        );
                    } else {
                        custom_println(
                            color,
                            "0",
                            format_args!("\n[ {:=<width$} ]", "", width = max_above),
                        );
                    }
                } else {
                    if settings.run_settings.display_ok_files {
                        print!(
                            " |=| {:width$}\x1b[0m",
                            path.display(),
                            width = max_length - (max_length * 2 / 3)
                        );
                        custom_print(
                            color,
                            "0;32;1",
                            format_args!("{:s$}OK\n", "", s = max_above),
                        );
                    }
                }
            }
        }
    }
    custom_println(
        color,
        "0;35;1",
        format_args!("\n+{:-<width$}+", "", width = 26 + project_path.len()),
    );
    custom_println(
        color,
        "0;35;1",
        format_args!(
            "Found {} matches ... {}",
            appearance,
            utils::att(&appearance)
        ),
    );
    custom_println(
        color,
        "0;35;1",
        format_args!("+{:-<width$}+", "", width = 26 + project_path.len()),
    );

    // println!("{:?}", vector_for_report_whit_all);
    wsj::init_report(color, settings.report_settings, vector_for_report_whit_all);

}

/// Parse command line arguments
fn parse_args() -> Config {
    let args = Args::parse();
    let path = args.path;
    let settings = args.settings;
    let fast = args.fast;
    let show = args.show_lines;
    // return Config (do not use ';' after the expression you want to return)

    Config {
        path: path.to_string(),
        settings: settings.to_string(),
        fast,
        show,
    }
}
