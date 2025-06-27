use chrono::TimeDelta;

use crate::{
    analyzer, domain::{
        AnalysisReport, LogLevel, ParsedLog, TimeStamp
    }, error::*, parser::log_parser
};

use std::collections::HashMap;


pub fn analyzer_report(input: &str) -> Result<AnalysisReport> {
    if input.is_empty() {
        return Err(AppError::UnvalidInput { input: input.to_string() });
    }

    let init_analysis = AnalysisReport::default();

   let mut res = input.lines().enumerate().fold(init_analysis, |mut acc, (line_index, line)| {
           acc.proceed_lines += 1;
   
           if let Ok(parsed_line) = log_parser(line, line_index) {
               acc.parsed_lines += 1;
               *acc.log_level_counter.entry(parsed_line.level.clone()).and_modify(|e| *e += 1).or_insert(1);
   
               match &parsed_line.level {
                   LogLevel::Error => {
                       acc.log_error_timestamp.push(parsed_line.time_stamp.clone());

   
                       if let Some(key) = acc.log_level_counter.get(&LogLevel::Error){
                            acc.log_error_timeline.push((parsed_line.time_stamp, *key));
                       }
 
                   },
                   _ => {},
               }
           } else {
               acc.parse_error_counter += 1;
           }
   
           acc
       });
       
        if !res.log_error_timeline.is_empty() {

            let mut adapted_view = res.log_error_timeline[..res.log_error_timeline.len() -1].windows(2);
            let burst= adapted_view.try_fold(Vec::<(TimeStamp, u32)>::new(), |mut acc, view| {
                if res.log_error_timeline.len() < 2 {                
                    return Err(AppError::Generique);
                }
                
                let (timestamp0, count0) = &view[0];
                let (timestamp1, count1) = &view[1];   
                
                let delta = timestamp1.clone() - timestamp0.clone();
                let sum = *count0 + *count1;    
                
                if delta > TimeDelta::seconds(5) && sum < 3 {
                    return Ok(acc)
                }
                
                if delta <= TimeDelta::seconds(5) && sum <= 3 {
                    return Ok(acc)
                }  
                
                // if (delta + delta_prime.clone()) <= TimeDelta::seconds(5) && sum + count >= 3{
                    //     println!("PUSH");
                    //     acc.push((view0.0.clone(), (sum + count)));
                    //     return Ok((acc,(TimeDelta::seconds(0), 0)));
                    // }   
                    
                    if delta <= TimeDelta::seconds(5) && sum >= 3 {
                        acc.push((timestamp0.clone(), *count1));
                        return Ok(acc)
                    }   ;
                    
                    
                    
                    
                    Ok(acc)
                })?;
                
                
                res.burst_error = burst;
            }

        println!("BURST {:#?}", res.burst_error);
        println!("BURST LEN {}", res.burst_error.len());
        



     

    Ok(res)
        
}


// pub fn windows_on_logerror(analysis: &AnalysisReport) -> Result<(TimeStamp, u32)> {

//     let log_err_data = analysis.log_error_timeline.clone();
//     let first_err_timestamp = analysis.burst_error.clone();

//     let res = log_err_data.windows(2).scan((false, 0), |(time_in, count), view| {
//         let view0 = &view[0];
//         let view1 = &view[1];

//         // let t0 = view0.0.clone();
//         // let t1 = view1.0.clone();

//         let delta = view1.0.clone() - view0.0.clone();

//         // let c0 = view0.1;
//         // let c1 = view1.1;

//         let sum = view0.1 + view1.1;

//         if delta < TimeDelta::seconds(5) && sum>= 3 {
//             return N((view0.0, sum));
//         }

//         if delta < TimeDelta::seconds(5) && sum < 3 {
//             return Some(())
//         }



//         Some((false, 2))
//     })

//     // if let Some(timestamp) = first_err_timestamp {
//     //     let mut delta = Vec::<(TimeDelta,u32)>::new();
        
//     //     log_err_data.iter().for_each(|(time, e)| {
//     //         let value = time.clone() - timestamp.clone();
//     //         delta.push((value, *e));
//     //     });

//     //     delta.iter().map(|e| {
//     //     })
//     // };


    
    



//     Ok(String::new())
// }


#[cfg(test)]

mod test {
    use super::*;

        #[test]
        fn test_analyzer_without_logerror() {
            let input = "2025-06-27T10:00:01Z [INFO] Application starting up.
2025_06-27T10:00:05Z [INFO] Database connection established.
2025-06-27T10:01:10Z [WARNING] Configuration value 'timeout' is deprecated.";
            
            let mut log_level_hash = HashMap::new();
            *log_level_hash.entry(LogLevel::Info).or_insert(0) += 1;
            *log_level_hash.entry(LogLevel::Warning).or_insert(0) += 1;

            let analyzer = AnalysisReport {
                proceed_lines: 3,
                parsed_lines: 2,
                parse_error_counter: 1,
                log_level_counter: log_level_hash,
                burst_error: vec![],
                log_error_timestamp: Vec::<TimeStamp>::new(),
                log_error_timeline: Vec::<(TimeStamp, u32)>::new(),
            };

            let result = analyzer_report(input).unwrap();
            
            assert_eq!(result, analyzer);


        } 

        #[test]
        fn test_analyzer_with_logerror() {
            let input = "2025-06-27T10:00:00Z [ERROR] Application starting up.
2025-06-27T10:00:05Z [INFO] Database connection established.
2025-06-27T10:01:10Z [WARNING] Configuration value 'timeout' is deprecated.
2025-06-27T10:02:02Z [ERROR] Circuit breaker opened for upstream service.
2025-06-27T10:02:03Z [ERROR] Circuit breaker opened for upstream service.
2025-06-27T10:02:04Z [ERROR] Circuit breaker opened for upstream service.
2025-06-27T10:02:07Z [ERROR] Circuit breaker opened for upstream service.
2025-06-27T10:02:08Z [ERROR] Critical error, system will shutdown in 5 seconds..";
            
            let mut log_level_hash = HashMap::new();
            *log_level_hash.entry(LogLevel::Error).or_insert(0) += 3;
            *log_level_hash.entry(LogLevel::Info).or_insert(0) += 1;
            *log_level_hash.entry(LogLevel::Warning).or_insert(0) += 1;

            let time_stamp1 = TimeStamp::from_str("2025-06-27T10:00:00Z").unwrap();
            let time_stamp2 = TimeStamp::from_str("2025-06-27T10:02:02Z").unwrap();
            let time_stamp3 = TimeStamp::from_str("2025-06-27T10:02:03Z").unwrap();
            let time_stamp4 = TimeStamp::from_str("2025-06-27T10:02:04Z").unwrap();
            let time_stamp5 = TimeStamp::from_str("2025-06-27T10:02:07Z").unwrap();
            let time_stamp6 = TimeStamp::from_str("2025-06-27T10:02:08Z").unwrap();


            let mut vec_timestamp = vec!(time_stamp1.clone());
            vec_timestamp.push(time_stamp2.clone());
            vec_timestamp.push(time_stamp3.clone());
            vec_timestamp.push(time_stamp4.clone());
            vec_timestamp.push(time_stamp5.clone());
            vec_timestamp.push(time_stamp6.clone());

            let mut vec_timeline = vec!((time_stamp1.clone(), 1));
            vec_timeline.push((time_stamp2, 2));
            vec_timeline.push((time_stamp3, 3));
            vec_timeline.push((time_stamp4, 4));
            vec_timeline.push((time_stamp5, 5));
            vec_timeline.push((time_stamp6, 6));


            let analyzer = AnalysisReport {
                proceed_lines: 8,
                parsed_lines: 8,
                parse_error_counter: 0,
                log_level_counter: log_level_hash,
                burst_error: vec![],
                log_error_timestamp: vec_timestamp,
                log_error_timeline: vec_timeline,
            };

            let result = analyzer_report(input).unwrap();
            
            assert_eq!(result, analyzer);


        }
}