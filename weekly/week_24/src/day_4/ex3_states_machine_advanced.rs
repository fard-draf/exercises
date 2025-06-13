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
            let parsed = parse_command(slice).map_err(|_| "Unable to parse");
            println!("PARSED{:?}", parsed);
            if let Ok(slice) = parsed {
                match slice {
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
                }
            }
            println!("ACC {:?}", acc);
            acc
        })
}

// Helper function suggérée (optionnelle)
fn parse_command(line: &str) -> Result<Command, String> {
    // TODO: Parser une ligne en Command enum
    let parsed: String = line.parse().map_err(|_| "Bad request")?;

    match parsed {
        value if value.to_uppercase().starts_with("SET") => {
            let (key, value, _) = value
                .chars()
                .fold((None::<char>, 0, false), |mut acc, char| {
                    if char.is_whitespace() {
                        acc.2 = true;
                    }
                    if !char.is_whitespace() && acc.2 && !char.is_numeric() {
                        acc.0 = Some(char);
                        // println!("is char {:?}", acc.0);
                    } else if char.is_numeric() && acc.2 {
                        acc.1 = acc.1 * 10 + (char as u8 - b'0') as i32;
                    } else {
                        let format = format!("Invalid number {}", value);
                    }

                    println!("DONE {:?}", acc);
                    acc
                });
            if let Some(char) = key {
                let key = char.to_string();
                // println!("ADD: Key {}, value {}", key, value);
                Ok(Command::Set { key, value })
            } else {
                Err("unable to parse".to_string())
            }
        }
        value if value.to_uppercase().starts_with("ADD") => {
            let (key, value, _) = value
                .chars()
                .fold((None::<char>, 0, false), |mut acc, char| {
                    if char.is_whitespace() {
                        acc.2 = true;
                    }
                    if !char.is_whitespace() && acc.2 && !char.is_numeric() {
                        acc.0 = Some(char);
                    }
                    if char.is_numeric() && acc.2 {
                        acc.1 = char.to_string().parse::<i32>().unwrap();
                    }
                    acc
                });
            if let Some(char) = key {
                let key = char.to_string();
                Ok(Command::Add { key, amount: value })
            } else {
                Err("unable to parse".to_string())
            }
        }
        value if value.to_uppercase().starts_with("PRINT") => {
            let (key, _, _) = value
                .chars()
                .fold((None::<char>, 0, false), |mut acc, char| {
                    if char.is_whitespace() {
                        acc.2 = true;
                    }
                    if !char.is_whitespace() && acc.2 && !char.is_numeric() {
                        acc.0 = Some(char);
                    }
                    if char.is_numeric() && acc.2 {
                        {};
                    }
                    acc
                });
            if let Some(char) = key {
                let key = char.to_string();
                Ok(Command::Print { key })
            } else {
                Err("unable to parse".to_string())
            }
        }

        value if value.to_uppercase().starts_with("CLEAR") => Ok(Command::Clear),
        _ => Err("Unvalid command".to_string()),
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
