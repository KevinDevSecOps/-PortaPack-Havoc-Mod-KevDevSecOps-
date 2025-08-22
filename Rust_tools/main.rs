
## 3. src/main.rs

```rust
//! 🚨 HERRAMIENTA DE INVESTIGACIÓN ÉTICA PARA PORTAPACK
//! 
//! ⚠️  ADVERTENCIA: USO SOLO CON AUTORIZACIÓN EXPLÍCITA
//! 📜 Cumplir todas las regulaciones locales e internacionales

#[macro_use]
extern crate log;

use clap::Parser;
use std::process;
use std::time::Duration;

mod ethical_guard;
mod spectrum_analyzer;
mod jammer;

use ethical_guard::{EthicalGuard, ResearchAuthorization};
use spectrum_analyzer::SpectrumAnalyzer;
use jammer::JammerController;

#[derive(Parser, Debug)]
#[command(
    name = "PortaPack Research Tool",
    author = "KevDevSecOps",
    version = "0.1.0",
    about = "Herramienta ética de investigación de espectro RF",
    long_about = "⚠️  USO SOLO PARA INVESTIGACIÓN AUTORIZADA\n\n🚨 INTERFERIR CON COMUNICACIONES ES DELITO GRAVE"
)]
struct Cli {
    /// Frecuencia central en MHz
    #[arg(short, long, default_value_t = 433.92)]
    frequency: f64,

    /// Ancho de banda en kHz
    #[arg(short, long, default_value_t = 1000.0)]
    bandwidth: f64,

    /// Modo de análisis (solo recepción)
    #[arg(short, long)]
    analyze: bool,

    /// Modo investigación (requiere autorización)
    #[arg(short, long, hide = true)] // Oculto por seguridad
    research: bool,

    /// Tiempo límite en segundos
    #[arg(short, long, default_value_t = 5)]
    time_limit: u64,

    /// Nivel de verbosidad
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inicializar logger
    env_logger::init();
    
    let args = Cli::parse();
    
    // Mostrar advertencias iniciales
    show_ethical_warnings();
    
    // Inicializar guardián ético
    let ethical_guard = EthicalGuard::new();
    
    // Convertir frecuencia a Hz
    let frequency_hz = args.frequency * 1e6;
    let bandwidth_hz = args.bandwidth * 1e3;
    
    if args.analyze {
        // Modo análisis (siempre permitido)
        info!("Iniciando análisis espectral en {:.2} MHz", args.frequency);
        let analyzer = SpectrumAnalyzer::new();
        let results = analyzer.analyze_spectrum(frequency_hz, bandwidth_hz)?;
        
        println!("📊 Resultados del análisis:");
        for (freq, power) in results {
            println!("  - {:.3} MHz: {:.1} dBm", freq / 1e6, power);
        }
        
    } else if args.research {
        // 🚨 MODO INVESTIGACIÓN - REQUIERE AUTORIZACIÓN
        warn!("INTENTANDO ACCEDER A MODO INVESTIGACIÓN");
        
        // Verificar autorizaciones
        if !ethical_guard.is_research_authorized() {
            error!("❌ NO AUTORIZADO PARA INVESTIGACIÓN");
            error!("Consulte README.md para requisitos");
            process::exit(1);
        }
        
        // Verificar frecuencia permitida
        if !ethical_guard.is_frequency_allowed(frequency_hz) {
            error!("❌ Frecuencia {} MHz en banda protegida", args.frequency);
            process::exit(1);
        }
        
        // Iniciar con límite de tiempo estricto
        info!("🔬 Iniciando investigación autorizada");
        let mut jammer = JammerController::new(frequency_hz, bandwidth_hz);
        jammer.set_time_limit(Duration::from_secs(args.time_limit));
        
        if let Err(e) = jammer.start_research_mode() {
            error!("Error en investigación: {}", e);
            process::exit(1);
        }
        
    } else {
        // Modo por defecto: solo análisis
        println!("🔍 Modo análisis pasivo activado");
        println!("Use --analyze para análisis espectral");
        println!("Use --help para más opciones");
    }
    
    Ok(())
}

fn show_ethical_warnings() {
    println!("{}", include_str!("../ethical_warning.txt"));
    
    // Pausa para asegurar lectura de advertencias
    std::thread::sleep(Duration::from_secs(2));
}
mod spatial_analyzer;
mod ml_detection;
mod signal_processing;
mod serial_interface;

use spatial_analyzer::SpatialAnalyzer;
use ml_detection::ModulationDetector;
use signal_processing::AdvancedSignalProcessor;
use serial_interface::PortaPackInterface;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 PortaPack AI Analyser iniciado");
    
    // Conectar con PortaPack
    let mut portapack = PortaPackInterface::new("/dev/ttyACM0")?;
    
    // Inicializar componentes
    let mut spatial_analyzer = SpatialAnalyzer::new(48000, 2);
    let mut modulation_detector = ModulationDetector::new();
    let mut signal_processor = AdvancedSignalProcessor::new();
    
    // Bucle principal de procesamiento
    loop {
        // Capturar datos del PortaPack
        let iq_data = portapack.capture_iq(1024)?;
        
        // Análisis en paralelo
        rayon::scope(|s| {
            // Análisis espacial
            s.spawn(|_| {
                let sources = spatial_analyzer.find_signal_sources(&iq_data, 3.0);
                println!("📡 Fuentes detectadas en ángulos: {:?}", sources);
            });
            
            // Detección de modulación
            s.spawn(|_| {
                for (ant_idx, antenna_data) in iq_data.iter().enumerate() {
                    let features = modulation_detector.extract_features(antenna_data);
                    let modulation = modulation_detector.classify_modulation(&features);
                    println!("🎛️  Antena {}: Modulación detectada: {}", ant_idx, modulation);
                }
            });
            
            // Detección de anomalías
            s.spawn(|_| {
                let anomalies = signal_processor.real_time_anomaly_detection(&iq_data[0]);
                if !anomalies.is_empty() {
                    println!("⚠️  Anomalías detectadas en muestras: {:?}", anomalies);
                }
            });
        });
        
        // Esperar para siguiente captura
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}