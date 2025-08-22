use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub struct SignalProcessor {
    sample_rate: u32,
}

impl SignalProcessor {
    pub fn new(sample_rate: u32) -> Self {
        SignalProcessor { sample_rate }
    }
    
    pub fn process_iq_file(&self, filename: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let mut iq_data = Vec::new();
        
        for line in reader.lines() {
            let line = line?;
            if let Ok(value) = line.trim().parse::<f32>() {
                iq_data.push(value);
            }
        }
        
        Ok(self.process_iq_data(&iq_data))
    }
    
    pub fn process_iq_data(&self, data: &[f32]) -> Vec<f32> {
        data.chunks(2)
            .filter_map(|chunk| {
                if chunk.len() == 2 {
                    // Procesamiento IQ básico
                    Some((chunk[0].powi(2) + chunk[1].powi(2)).sqrt())
                } else {
                    None
                }
            })
            .collect()
    }
    
    pub fn demodulate_fm(&self, data: &[f32]) -> Vec<f32> {
        let mut result = Vec::with_capacity(data.len().saturating_sub(1));
        for i in 1..data.len() {
            let phase_diff = data[i].atan2(data[i-1]);
            result.push(phase_diff);
        }
        result
    }
    
    pub fn find_peaks(&self, data: &[f32], threshold: f32) -> Vec<usize> {
        let mut peaks = Vec::new();
        for i in 1..data.len()-1 {
            if data[i] > data[i-1] && data[i] > data[i+1] && data[i] > threshold {
                peaks.push(i);
            }
        }
        peaks
    }
}

pub fn analyze_signal(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let processor = SignalProcessor::new(48000);
    let processed_data = processor.process_iq_file(filename)?;
    
    let peaks = processor.find_peaks(&processed_data, 0.5);
    println!("Encontrados {} picos en la señal", peaks.len());
    
    // Análisis espectral básico
    if let Some(max_peak) = processed_data.iter().max_by(|a, b| a.partial_cmp(b).unwrap()) {
        println!("Pico máximo de señal: {:.6}", max_peak);
    }
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <iq-file>", args[0]);
        std::process::exit(1);
    }
    
    analyze_signal(&args[1])?;
    
    Ok(())
}