use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct  Config {
    pub path: String,
    pub settings: String,
    pub fast: bool,
    pub show: bool,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct  Pattern {
    pub pattern: String,
    pub comment: String,
    pub regex: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PatternVS {
    pub pattern: String,
    pub comment: String,
    pub regex: bool,
    pub severity: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct  Severity {
    pub h: Vec<Pattern>,
    pub m: Vec<Pattern>,
    pub l: Vec<Pattern>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct  Patterns {
    pub severity: Severity,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReportParams {
    pub type_: String,
    pub exclude_: String,
    pub caps_: bool,
    pub filename_as_head: bool,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct  RunSettings {
    pub color_output: bool,
    pub display_ok_files: bool,
    pub show_patterns: bool,
    pub use_custom_severity_and_exp: bool,
    pub check_code: bool,
    pub write_report: bool
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CodeOptionsChck {
    pub comments: bool,
    pub ends_with_blank_line: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReportSettings {
    pub report_path: String,
    pub report_title: String,
    pub report_params: ReportParams,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    pub run_settings: RunSettings,
    pub check_code_options: CodeOptionsChck,
    pub report_settings: ReportSettings,
    pub project_path: String,
    pub patterns: Patterns,
    pub ignore: Vec<String>,
}


pub struct Result {
    pub matches: Vec<Vec<Vec<String>>>,
    pub high: bool,
    pub mid: bool,
    pub ends_with_blank_line: bool,
}

impl Result {
    pub fn clear(&mut self) {
        self.matches.clear();
    }
}

#[derive(Debug, Clone)]
pub struct FoundMatchesResult {
    // pub line_: String,
    pub pattern_: String,
    pub severity_: u32,
}