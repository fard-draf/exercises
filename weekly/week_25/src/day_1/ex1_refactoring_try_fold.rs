// 🎯 Challenge: Réfactoring avec try_fold
// 📊 Niveau: Easy
// ⏱️ Durée: 45min
//
// 📋 MISSION COMPLETE:
// Le code existant utilise fold pour valider une liste de commandes. Si une commande est invalide,
// la fonction continue de parcourir toute la liste inutilement.
// Ta mission est de ré-implémenter cette logique en utilisant try_fold pour que le processus
// s'arrête et retourne une erreur dès la première commande invalide rencontrée.
//
// 📥 ENTRÉES:
// - commands: Un slice de Command (&[Command]).
//   - Command est une struct simple avec un id (u32) et une payload (String).
//
// 📤 SORTIES:
// - Success: Ok(()) si toutes les commandes sont valides.
// - Erreurs: Err(ValidationError) si une commande est invalide.
//   - ValidationError est un enum qui doit contenir InvalidId ou PayloadTooShort(u32).
//
// 📏 RÈGLES MÉTIER:
// 1. Une commande est valide si son id n'est pas 0.
// 2. Une commande est valide si sa payload a une longueur d'au moins 3 caractères.
// 3. La validation doit s'arrêter à la PREMIÈRE erreur.
//
// 🧪 TESTS FOURNIS:
// Les tests vérifient la validité, l'erreur sur l'ID, et l'erreur sur la longueur de la payload,
// ainsi que le comportement de court-circuit.

use std::ops::ControlFlow;

#[derive(Debug, PartialEq)]
pub enum ValidationError {
    InvalidId,
    PayloadTooShort(u32), // Contient l'ID de la commande en erreur
}

#[derive(Debug, Clone)]
pub struct Command {
    id: u32,
    payload: String,
}

// L'objectif est de remplacer cette implémentation par une utilisant try_fold.
pub fn validate_all_commands(commands: &[Command]) -> Result<(), ValidationError> {
    let result = commands.iter().try_fold((), |_, command| {
        if command.id == 0 {
            ControlFlow::Break(ValidationError::InvalidId)
        } else if command.payload.len() < 3 {
            ControlFlow::Break(ValidationError::PayloadTooShort(command.id))
        } else {
            ControlFlow::Continue(())
        }
    });
    match result {
        ControlFlow::Break(ValidationError::InvalidId) => Err(ValidationError::InvalidId),
        ControlFlow::Break(ValidationError::PayloadTooShort(e)) => {
            Err(ValidationError::PayloadTooShort(e))
        }
        ControlFlow::Continue(()) => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_commands_valid() {
        let commands = vec![
            Command {
                id: 1,
                payload: "start".to_string(),
            },
            Command {
                id: 2,
                payload: "process".to_string(),
            },
        ];
        assert_eq!(validate_all_commands(&commands), Ok(()));
    }

    #[test]
    fn test_invalid_id_stops_processing() {
        let commands = vec![
            Command {
                id: 1,
                payload: "start".to_string(),
            },
            Command {
                id: 0,
                payload: "invalid".to_string(),
            }, // Erreur ici
            Command {
                id: 3,
                payload: "never_reaches".to_string(),
            },
        ];
        assert_eq!(
            validate_all_commands(&commands),
            Err(ValidationError::InvalidId)
        );
    }

    #[test]
    fn test_payload_too_short_stops_processing() {
        let commands = vec![
            Command {
                id: 1,
                payload: "start".to_string(),
            },
            Command {
                id: 42,
                payload: "no".to_string(),
            }, // Erreur ici
            Command {
                id: 3,
                payload: "never_reaches".to_string(),
            },
        ];
        assert_eq!(
            validate_all_commands(&commands),
            Err(ValidationError::PayloadTooShort(42))
        );
    }

    #[test]
    fn test_empty_input_is_valid() {
        let commands: Vec<Command> = vec![];
        assert_eq!(validate_all_commands(&commands), Ok(()));
    }
}

// 5 minutes