// 🎯 Exercice 3: Static Lifetimes - Config Manager
// 📊 Niveau: Medium
// ⏱️ Durée: 15min
//
// 📋 MISSION COMPLETE:
// Créer un système de configuration qui gère à la fois:
// - Des valeurs statiques (constantes du programme)
// - Des valeurs dynamiques (chargées à runtime)
// Comprendre quand utiliser 'static vs lifetimes normaux
//
// 📥 ENTRÉES:
// - ConfigValue enum: Static(&'static str) ou Dynamic(String)
// - ConfigManager qui stocke des configs avec différentes lifetimes
// - get_value() -> référence avec lifetime approprié
//
// 📤 SORTIES:
// - ConfigValue::Static pour données constantes (compile-time)
// - ConfigValue::Dynamic pour données runtime (heap-allocated)
// - get_as_str() -> &str avec lifetime correct selon le variant
//
// 📏 RÈGLES MÉTIER:
// 1. Static variant: référence &'static str (vit toute l'app)
// 2. Dynamic variant: String owned (convertible en &str temporaire)
// 3. get_as_str() retourne &str avec lifetime approprié
// 4. Static configs peuvent être utilisées partout
// 5. Dynamic configs lifetime liée à l'owner
//
// 🧪 EXEMPLES:
// let static_config = ConfigValue::Static("localhost");
// let dynamic_config = ConfigValue::Dynamic("127.0.0.1".to_string());
// assert_eq!(static_config.get_as_str(), "localhost");
// assert_eq!(dynamic_config.get_as_str(), "127.0.0.1");

// TODO: Définir ConfigValue enum avec variants Static et Dynamic
enum ConfigValue {
    Static(&'static str), // Compile-time constant
    Dynamic(String),      // Runtime allocated
}

impl ConfigValue {
    // TODO: Méthode qui retourne &str avec lifetime approprié
    // HINT: Static -> 'static, Dynamic -> lifetime de &self
    fn get_as_str(&self) -> &str {
        match self {
            Self::Static(value) => value,
            Self::Dynamic(value) => value,
        }
    }

    // TODO: Vérifie si la config est statique
    fn is_static(&self) -> bool {
        matches!(self, Self::Static(_))
    }

    // TODO: Vérifie si la config est dynamique
    fn is_dynamic(&self) -> bool {
        matches!(self, Self::Dynamic(_))
    }
}

// TODO: Manager qui peut stocker plusieurs configs
struct ConfigManager {
    host: ConfigValue,
    port: ConfigValue,
}

impl ConfigManager {
    // TODO: Constructeur avec host/port configs
    fn new(host: ConfigValue, port: ConfigValue) -> Self {
        Self { host, port }
    }

    // TODO: Getter pour host config
    fn get_host(&self) -> &str {
        self.host.get_as_str()
    }

    // TODO: Getter pour port config
    fn get_port(&self) -> &str {
        self.port.get_as_str()
    }

    // TODO: Retourne true si toutes les configs sont statiques
    fn all_static(&self) -> bool {
        self.host.is_static() && self.port.is_static()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_config() {
        let config = ConfigValue::Static("localhost");

        assert_eq!(config.get_as_str(), "localhost");
        assert_eq!(config.is_static(), true);
        assert_eq!(config.is_dynamic(), false);
    }

    #[test]
    fn test_dynamic_config() {
        let config = ConfigValue::Dynamic("127.0.0.1".to_string());

        assert_eq!(config.get_as_str(), "127.0.0.1");
        assert_eq!(config.is_static(), false);
        assert_eq!(config.is_dynamic(), true);
    }

    #[test]
    fn test_config_manager_mixed() {
        let host = ConfigValue::Static("localhost"); // Constant
        let port = ConfigValue::Dynamic("8080".to_string()); // Runtime

        let manager = ConfigManager::new(host, port);

        assert_eq!(manager.get_host(), "localhost");
        assert_eq!(manager.get_port(), "8080");
        assert_eq!(manager.all_static(), false); // Mixed static/dynamic
    }

    #[test]
    fn test_config_manager_all_static() {
        let host = ConfigValue::Static("localhost");
        let port = ConfigValue::Static("3000");

        let manager = ConfigManager::new(host, port);

        assert_eq!(manager.all_static(), true);
    }

    #[test]
    fn test_config_manager_all_dynamic() {
        let host = ConfigValue::Dynamic("127.0.0.1".to_string());
        let port = ConfigValue::Dynamic("8080".to_string());

        let manager = ConfigManager::new(host, port);

        assert_eq!(manager.get_host(), "127.0.0.1");
        assert_eq!(manager.get_port(), "8080");
        assert_eq!(manager.all_static(), false);
    }
}
