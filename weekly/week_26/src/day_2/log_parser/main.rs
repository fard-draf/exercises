use std::collections::HashMap;
use std::{
    default, env,
    fs::{self, File},
    io::{BufRead, BufReader},
};

use chrono::TimeDelta;

use crate::domain::{LogLevel, Operation};
use crate::parser::parse_log;
use crate::{
    domain::{AnalysisReport},
    error::AppError,
};

mod analyzer;
mod domain;
mod error;
mod parser;

fn main() -> Result<(), AppError> {
    // let args: Vec<String> = env::args().collect();

    // if args.len() != 2 {
    //     eprint!("Usage: {} <nom_du_fichier>", &args[0]);
    //     return Err(AppError::CommandLine(error::CommandLine::MissingArgs));
    // }

    // let filepath = &args[1];
    // println!("Reading file: {}", filepath);
    // let file = File::open(filepath)?;
    // let reader = BufReader::new(file);

    // let mut initial_report = AnalysisReport::default();

    // let result = reader
    //     .lines()
    //     .fold(initial_report, |mut report, line| {
    //         report.total_lines += 1;

    //         match line {

    //             Ok(line_str) => {
    //                 match parse_log(&line_str) {
    //                     Ok(log_entry) => {

                            
    //                         let log_duration = report.last_log.get_key_value(&log_entry.user_id).and_then(|(user, (op, timestamp))| {
                                
                                
    //                             report.last_log.get(&log_entry.user_id)
    //                             .and_then(|(next_op, next_time_stamp)| {
    //                                 if *op == Operation::Login && *next_op == Operation::Logout {
    //                                     Some((log_entry.user_id,next_time_stamp.time - timestamp.time))
    //                                 } else {
    //                                     None
    //                                 }
    //                             })
    //                         }).unwrap_or_else(|| (log_entry.user_id, TimeDelta::zero()));
                            
    //                         report.last_log
    //                             .entry(log_entry.user_id)
    //                             .and_modify(|e| *e = (log_entry.operation, log_entry.time_stamp))
    //                             .or_insert((log_entry.operation, log_entry.time_stamp));
                            
    //                         report.log_duration.entry(log_duration.0).and_modify(|e| *e = log_duration.1).or_insert(TimeDelta::zero());
                            
    //                         *report.level_distribution.entry(log_entry.level).or_insert(0) += 1;
    //                         report.log_count += 1;
    //                         *report.user_activity.entry(log_entry.user_id).or_insert(0) += 1;


    //                     },

    //                     Err(e) => {
    //                         report.corrupted_lines +=1;
    //                     },
    //                 }
                    
    //             },

    //             Err(_) => {report.corrupted_lines += 1;},
                
    //         }




    //         report
            
    //     });

    Ok(())
}
