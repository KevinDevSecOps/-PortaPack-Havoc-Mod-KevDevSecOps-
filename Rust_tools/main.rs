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