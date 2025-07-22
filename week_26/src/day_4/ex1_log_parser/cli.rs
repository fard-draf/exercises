use crate::{
    domain::{AnalysisReport, LogLevel},
    error::*,
};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version = "1.0", about = "Log Parser", long_about = None)]
pub struct LogParserCli {
    path: PathBuf,
}

pub fn initialize_cli(data: AnalysisReport) -> Result<()> {
    println!("==================== Log Analysis Report ====================");
    println!("File: thereisnofile.txt");
    println!("\n");
    println!("--- Summary ---");
    println!("Total logs processed: {}", data.proceed_lines);
    println!("Total parsed error: {}", data.parse_error_counter);
    println!(
        "Total parsed lines: {} -> {:.2}% of succeed",
        data.parsed_lines,
        (data.parsed_lines as f64 / data.proceed_lines as f64) * 100.0
    );

    println!("");
    println!("Log level distribution:");

    if let Some(value) = data.log_level_counter.get(&LogLevel::Info) {
        println!("\t - INFO: {:?}", value);
    } else {
        println!("\t - INFO: 0");
    }
    if let Some(value) = data.log_level_counter.get(&LogLevel::Warning) {
        println!("\t - Warning: {:?}", value);
    } else {
        println!("\t - Warning: 0");
    }
    if let Some(value) = data.log_level_counter.get(&LogLevel::Error) {
        println!("\t - Error: {:?}", value);
    } else {
        println!("\t - INFO: 0");
    }

    println!("");
    println!("--- Error Analysis --- \n Cumulative error count over time:");
    for element in data.log_error_timeline {
        println!(" - {:?}: {}", element.0, element.1);
    }
    println!("");
    println!("Detected error bursts (>= 3 errors in a 5s window):");
    for element in data.burst_error {
        println!(
            " - Burst started at {:?} with {} errors",
            element.0, element.1
        );
    }

    println!("");
    println!("================================================================");
    Ok(())
}
