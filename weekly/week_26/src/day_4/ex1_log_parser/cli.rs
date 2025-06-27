use clap::Parser;
use std::path::PathBuf;
use crate::{domain::{AnalysisReport, LogLevel}, error::*};


#[derive(Parser, Debug)]
#[command(version = "1.0", about = "Log Parser", long_about = None)]
pub struct LogParserCli {
    path: PathBuf,
}


pub fn initialize_cli(data: AnalysisReport) -> Result<()> {
    let cli = LogParserCli::parse();

    println!("==================== Log Analysis Report ====================");
    println!("File: {:?}", cli);

    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    println!("--- Summary ---");

    println!("Total logs processed: {}", data.proceed_lines);
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


    println!("--- Error Analysis --- \n Cumulative error count over time:");
    for element in data.log_error_timeline {
        println!(" - {:?}: {}", element.0, element.1);
    };

    println!("Detected error bursts (>= 3 errors in a 5s window):");
    for element in data.burst_error {
        println!(" - Burst started at {:?} with {} errors", element.0, element.1);
    }

    println!("================================================================");   
    Ok(())
}