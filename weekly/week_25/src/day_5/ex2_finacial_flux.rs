use std::collections::HashMap;

#[derive(Debug, PartialEq, Default)]
pub struct FinancialReport {
    total_deposits: f64,
    total_withdrawals: f64,
    final_balances: HashMap<String, f64>,
    first_transfer_user: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum AnalysisError {
    InvalidLine(String),
    InvalidTransactionType(String),
    InvalidAmount(String),
    MalformedTransaction(String),
}

pub fn analyze_transactions(data: &str) -> Result<FinancialReport, AnalysisError> {
    // Le coeur de l'exercice est ici.
    // Pense à l'accumulateur de ton try_fold. Il doit être une structure
    // temporaire qui ressemble beaucoup à FinancialReport.
    if data.is_empty() {
        return Ok(FinancialReport::default());
    }
    
    let mut balances = HashMap::<String, f64>::new();
    let mut first_transfer: Option<String> = None;
    let mut total_deposits = 0.0;
    let mut total_withdrawals = 0.0;
    
    data.lines().into_iter().try_fold(FinancialReport::default(), |mut operation, lines| {
        
        if !lines.contains(',') {
            return Err(AnalysisError::MalformedTransaction(lines.to_string()))
        }
        
        let mut splitted_line= lines.split(',');
        let part_1 = splitted_line.next().ok_or(AnalysisError::MalformedTransaction(lines.to_string()))?;
        let part_2 = splitted_line.next().ok_or(AnalysisError::MalformedTransaction(lines.to_string()))?.to_string();
        let part_3 = splitted_line.next().ok_or(AnalysisError::MalformedTransaction(lines.to_string()))?.to_string();
        let part_4 = { if part_1 == "TRANSFER" { 
            Some(splitted_line.next().ok_or(AnalysisError::MalformedTransaction(lines.to_string()))?.to_string())
        } else {
            None
        }
    };
        if let Some(_part) = splitted_line.next() {
            return Err(AnalysisError::MalformedTransaction(lines.to_string()))
        }





    

    match part_1{
            "DEPOSIT" => {
                let deposit_value = part_3.parse::<f64>().map_err(|_| AnalysisError::InvalidAmount(part_3.to_string()))?;
                *balances.entry(part_2).or_insert(0.0) += deposit_value;
                total_deposits += deposit_value;
                
            },
            "WITHDRAW" => {
                let withdraw_value = part_3.parse::<f64>().map_err(|_| AnalysisError::InvalidAmount(part_3.to_string()))?;

                *balances.entry(part_2).or_insert(0.0) -= withdraw_value ;
                total_withdrawals += withdraw_value;
                
            },
            "TRANSFER" => {
                let transfert_value = if let Some(value) = &part_4 {
                    value.parse::<f64>().map_err(|_| AnalysisError::InvalidAmount(part_4.unwrap().to_string()))?                       
                } else {
                    0.0
                };

                *balances.entry(part_2.to_string()).or_insert(0.0) -= transfert_value;    


                *balances.entry(part_3.to_string()).or_insert(0.0) += transfert_value;

                first_transfer = Some(part_2);
                println!("TRANSFER {:#?}", operation.final_balances);
                





            },
            _ => return Err(AnalysisError::InvalidTransactionType(part_1.to_string()))
        };
    
                println!("FINAL {:#?}", balances);

        operation.final_balances = balances.clone();
        operation.first_transfer_user = first_transfer.clone();
        operation.total_deposits = total_deposits;
        operation.total_withdrawals = total_withdrawals;


        Ok(operation)
        


    })
    
    
    

    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_successful_analysis() {
        let data = "DEPOSIT,USER_A,100.0\n\
                    DEPOSIT,USER_B,200.0\n\
                    WITHDRAW,USER_A,20.0\n\
                    TRANSFER,USER_B,USER_A,50.0";
        let report = analyze_transactions(data).unwrap();
        assert_eq!(report.total_deposits, 300.0);
        assert_eq!(report.total_withdrawals, 20.0);
        assert_eq!(report.final_balances.get("USER_A"), Some(&130.0)); // 100 - 20 + 50
        assert_eq!(report.final_balances.get("USER_B"), Some(&150.0)); // 200 - 50
        assert_eq!(report.first_transfer_user, Some("USER_B".to_string()));
    }

    #[test]
    fn test_empty_input() {
        let data = "";
        let report = analyze_transactions(data).unwrap();
        assert_eq!(report.total_deposits, 0.0);
        assert_eq!(report.total_withdrawals, 0.0);
        assert!(report.final_balances.is_empty());
        assert_eq!(report.first_transfer_user, None);
    }

    #[test]
    fn test_error_on_invalid_line() {
        let data = "DEPOSIT,USER_A,100.0\n\
                    INVALID_LINE\n\
                    WITHDRAW,USER_A,20.0";
        let result = analyze_transactions(data);
        assert!(matches!(result, Err(AnalysisError::MalformedTransaction(_))));
    }
    
    #[test]
    fn test_error_on_invalid_amount() {
        let data = "DEPOSIT,USER_A,NOT_A_NUMBER";
        let result = analyze_transactions(data);
        assert!(matches!(result, Err(AnalysisError::InvalidAmount(_))));
    }

    #[test]
    fn test_error_on_unknown_transaction() {
        let data = "PAYMENT,USER_A,100.0";
        let result = analyze_transactions(data);
        assert!(matches!(result, Err(AnalysisError::InvalidTransactionType(_))));
    }
}