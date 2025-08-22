//! 🛡️ Guardián Ético - Previene uso inapropiado

use std::collections::HashSet;
use std::time::{SystemTime, Duration};

#[derive(Debug, Clone)]
pub struct ResearchAuthorization {
    pub authorized: bool,
    pub expiration: SystemTime,
    pub licensee: String,
    pub license_number: String,
}

pub struct EthicalGuard {
    protected_frequencies: HashSet<(f64, f64)>, // (start, end) en Hz
    authorized: bool,
    max_time_limit: Duration,
}

impl EthicalGuard {
    pub fn new() -> Self {
        let mut protected = HashSet::new();
        
        // Bandas protegidas (en Hz)
        // Aviación
        protected.insert((108e6, 137e6));
        // Emergencias
        protected.insert((144e6, 146e6));
        // Servicios públicos
        protected.insert((400e6, 470e6));
        // Comunicaciones críticas
        protected.insert((800e6, 900e6));
        // GPS
        protected.insert((1.1e9, 1.6e9));
        
        Self {
            protected_frequencies: protected,
            authorized: false, // Por defecto no autorizado
            max_time_limit: Duration::from_secs(5),
        }
    }
    
    pub fn is_research_authorized(&self) -> bool {
        // En producción, verificar licencias reales
        self.authorized
    }
    
    pub fn is_frequency_allowed(&self, frequency: f64) -> bool {
        for &(start, end) in &self.protected_frequencies {
            if frequency >= start && frequency <= end {
                return false;
            }
        }
        true
    }
    
    pub fn get_max_time_limit(&self) -> Duration {
        self.max_time_limit
    }
    
    // Solo para pruebas y desarrollo autorizado
    #[cfg(feature = "hardware_testing")]
    pub fn enable_research_mode(&mut self, license: &str) -> Result<(), String> {
        if license != "AUTH-RESEARCH-2024" {
            return Err("Licencia de investigación inválida".into());
        }
        self.authorized = true;
        Ok(())
    }
}