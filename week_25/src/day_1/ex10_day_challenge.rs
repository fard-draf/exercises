// 🎯 [Traits, Error Handling] - Niveau [AVANCÉ] - [90-180min]
//
// ⚓ MISSION :
// Vous devez implémenter un processeur pour un flux de paquets de données brutes.
// Ce processeur doit valider chaque paquet et agréger des informations statistiques
// en une seule passe. Le traitement doit s'arrêter immédiatement si un paquet est
// invalide ou si une condition globale d'agrégation est violée.
//
// 📋 ENTRÉES/SORTIES :
// - Input: Une collection de "paquets", où chaque paquet est une tranche d'octets.
// - Output: Un `Resultat` contenant soit un rapport de synthèse complet, soit une
//   erreur décrivant la nature précise de la première défaillance rencontrée.
//
// 📐 CONTRAINTES :
// 1. Un paquet est valide si sa longueur est d'au moins 2 octets ET si son dernier
//    octet (le checksum) correspond à la somme des octets de la charge utile
//    (tous les octets sauf le dernier), le tout modulo 256.
// 2. La "valeur" d'un paquet est la somme de sa charge utile (sans le checksum).
//    Le traitement doit échouer si la somme cumulative de toutes les valeurs
//    des paquets valides dépasse la capacité d'un `u32`.
// 3. L'analyse complète du flux de paquets doit être effectuée en une seule
//    itération sur la collection d'entrée. Aucune passe multiple n'est autorisée.
//
// 🧪 VALIDATION :
// Vos tests passent = exercice réussi.

// === STRUCTURES DE DONNÉES (NON MODIFIABLES) ===
#[derive(Debug, PartialEq, Eq, Default)]
pub struct AnalysisReport {
    pub packets_processed: u32,
    pub total_payload_value: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProcessorError {
    PacketTooShort,
    InvalidChecksum,
    CumulativeOverflow,
}

// === CODE MINIMAL ===
// TODO: Remplacez par votre implémentation
pub fn process_packet_stream(packets: &[Vec<u8>]) -> Result<AnalysisReport, ProcessorError> {
    let result = packets
        .iter()
        .try_fold(AnalysisReport::default(), |mut acc, packet| {
            if packet.len() < 2 {
                return Err(ProcessorError::PacketTooShort);
            }

            let checksum = packet.last().ok_or(ProcessorError::InvalidChecksum)?;
            let len = packet.len() - 1;
            let raw_sum: u32 = packet.iter().take(len).map(|e| *e as u32).sum();

            if raw_sum % 256 != *checksum as u32 {
                return Err(ProcessorError::InvalidChecksum);
            }

            acc.packets_processed += 1;

            match acc.total_payload_value.checked_add(*checksum as u32) {
                Some(new_total) => acc.total_payload_value = new_total,
                None => return Err(ProcessorError::CumulativeOverflow),
            }

            println!("{}", acc.total_payload_value);
            Ok(acc)
        });

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_cas_nominal() {
        // Paquet 1: charge [10, 20], checksum [30]. Valeur = 30
        // Paquet 2: charge [5, 5], checksum [10]. Valeur = 10
        // Total: 2 paquets, valeur totale 40
        let packets = vec![vec![10, 20, 30], vec![5, 5, 10]];
        let report = process_packet_stream(&packets).unwrap();
        assert_eq!(
            report,
            AnalysisReport {
                packets_processed: 2,
                total_payload_value: 40,
            }
        );
    }

    #[test]
    fn validation_arret_immediat_checksum() {
        // Le second paquet a un checksum incorrect (5+6 != 10)
        let packets = vec![vec![10, 10, 20], vec![5, 6, 10]];
        let error = process_packet_stream(&packets).unwrap_err();
        assert_eq!(error, ProcessorError::InvalidChecksum);
    }

    #[test]
    fn validation_arret_immediat_paquet_court() {
        // Le premier paquet est valide, le second est trop court
        let packets = vec![vec![100, 200, 44], vec![50]];
        let error = process_packet_stream(&packets).unwrap_err();
        assert_eq!(error, ProcessorError::PacketTooShort);
    }

    #[test]
    fn validation_flux_vide() {
        let packets = vec![];
        let report = process_packet_stream(&packets).unwrap();
        assert_eq!(
            report,
            AnalysisReport {
                packets_processed: 0,
                total_payload_value: 0,
            }
        );
    }
}
