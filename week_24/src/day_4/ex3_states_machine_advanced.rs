// 🎯 Challenge: Command Parser State Machine
// 📊 Niveau: Medium
// ⏱️ Durée: 1h30

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ExecutionResult {
    pub variables: HashMap<String, i32>,
    pub output: Vec<String>,
    pub errors: Vec<String>,
}

pub fn execute_commands_fold(commands: &str) -> ExecutionResult {
    // TODO: Parser et exécuter les commandes en utilisant fold
    // Accumulateur: ExecutionResult en construction
    // État: mise à jour selon chaque commande
    //
    // Stratégie suggérée :
    // 1. Split sur '\n' et fold sur les lignes
    // 2. Pour chaque ligne : parser la commande
    // 3. Exécuter et mettre à jour l'état
    // 4. Capturer les erreurs sans arrêter
    commands
        .split("\n")
        .fold(ExecutionResult::default(), |mut acc, slice| {
            match parse_command(slice.trim()) {
                Ok(cmd) => match cmd {
                    Command::Set { key, value } => {
                        acc.variables.entry(key).insert_entry(value);
                    }
                    Command::Add { key, amount } => {
                        acc.variables
                            .entry(key)
                            .and_modify(|x| *x += amount)
                            .or_insert(amount);
                        println!("amount {}", amount);
                    }
                    Command::Print { key } => {
                        if let Some(value) = acc.variables.get(&key) {
                            let format = format!("{}={}", key, value);
                            acc.output.push(format);
                        } else {
                            let format = format!("Unknown key: {}", key);
                            acc.errors.push(format);
                        }
                    }
                    Command::Clear => {
                        acc.variables.drain();
                    }
                },
                Err(e) => acc.errors.push(e.to_string()),
            }
            println!("ACCU {:#?}", acc);
            acc
        })
}

// Helper function suggérée (optionnelle)
fn parse_command(line: &str) -> Result<Command, String> {
    // TODO: Parser une ligne en Command enum
    let parts = line.split_whitespace().collect::<Vec<&str>>();
    if parts.is_empty() {
        return Err("Empty command".to_string());
    }

    match parts[0].to_uppercase().as_str() {
        "SET" if parts.len() == 3 => {
            let key = parts[1].to_string();
            let value = parts[2]
                .parse::<i32>()
                .map_err(|_| format!("Invalid number: {}", parts[2]))?;
            Ok(Command::Set { key, value })
        }
        "ADD" if parts.len() == 3 => {
            let key = parts[1].to_string();
            let amount = parts[2]
                .parse::<i32>()
                .map_err(|_| format!("Invalid amount: {}", parts[2]))?;
            Ok(Command::Add { key, amount })
        }
        "PRINT" if parts.len() == 2 => Ok(Command::Print {
            key: parts[1].to_string(),
        }),
        "CLEAR" => Ok(Command::Clear),
        _ => Err("Invalid command".to_string()),
    }
}

#[derive(Debug)]
enum Command {
    Set { key: String, value: i32 },
    Add { key: String, amount: i32 },
    Print { key: String },
    Clear,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_commands() {
        let commands = "SET x 10\nSET y 20\nPRINT x\nPRINT y";
        let result = execute_commands_fold(commands);

        assert_eq!(result.variables.get("x"), Some(&10));
        assert_eq!(result.variables.get("y"), Some(&20));
        assert_eq!(result.output, vec!["x=10", "y=20"]);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_add_command() {
        let commands = "SET x 10\nADD x 5\nADD y 3\nPRINT x\nPRINT y";
        let result = execute_commands_fold(commands);

        assert_eq!(result.variables.get("x"), Some(&15));
        assert_eq!(result.variables.get("y"), Some(&3));
        assert_eq!(result.output, vec!["x=15", "y=3"]);
    }

    #[test]
    fn test_clear_command() {
        let commands = "SET x 10\nSET y 20\nCLEAR\nPRINT x";
        let result = execute_commands_fold(commands);

        assert!(result.variables.is_empty());
        assert!(result.output.is_empty());
        assert_eq!(result.errors, vec!["Unknown key: x"]);
    }

    #[test]
    fn test_error_handling() {
        let commands = "SET x abc\nADD y xyz\nPRINT z\nSET w 42";
        let result = execute_commands_fold(commands);

        assert_eq!(result.variables.get("w"), Some(&42));
        assert_eq!(result.errors.len(), 3);
        assert!(result.errors[0].contains("Invalid number: abc"));
        assert!(result.errors[1].contains("Invalid amount: xyz"));
        assert!(result.errors[2].contains("Unknown key: z"));
    }

    #[test]
    fn test_empty_lines_and_spaces() {
        let commands = "SET  x   10\n\n  \nADD  x   5  \nPRINT   x";
        let result = execute_commands_fold(commands);

        assert_eq!(result.variables.get("x"), Some(&15));
        assert_eq!(result.output, vec!["x=15"]);
    }
}
