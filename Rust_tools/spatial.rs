use ndarray::{Array2, Array1, Axis};
use rustfft::{FftPlanner, num_complex::Complex, num_traits::Zero};
use std::f32::consts::PI;

pub struct SpatialAnalyzer {
    fft_planner: FftPlanner<f32>,
    sample_rate: u32,
    num_antennas: usize,
}

impl SpatialAnalyzer {
    pub fn new(sample_rate: u32, num_antennas: usize) -> Self {
        Self {
            fft_planner: FftPlanner::new(),
            sample_rate,
            num_antennas,
        }
    }

    /// Beamforming digital para localizar fuentes de señal
    pub fn do_beamforming(&mut self, iq_data: &[Vec<Complex<f32>>]) -> Vec<f32> {
        let mut spectrum = vec![0.0; 360]; // 0-359 grados
        
        for angle in 0..360 {
            let phase_shift = (angle as f32).to_radians();
            let mut beamformed = Vec::with_capacity(iq_data[0].len());
            
            for sample_idx in 0..iq_data[0].len() {
                let mut sum = Complex::zero();
                for ant_idx in 0..self.num_antennas {
                    let delay = (ant_idx as f32) * phase_shift.sin();
                    let phase_correction = Complex::new(0.0, -2.0 * PI * delay).exp();
                    sum += iq_data[ant_idx][sample_idx] * phase_correction;
                }
                beamformed.push(sum);
            }
            
            // Calcular potencia para este ángulo
            spectrum[angle] = beamformed.iter()
                .map(|c| c.norm_sqr())
                .sum::<f32>()
                .log10();
        }
        
        spectrum
    }

    /// Detectar direcciones de llegada (DoA)
    pub fn find_signal_sources(&mut self, iq_data: &[Vec<Complex<f32>>], threshold: f32) -> Vec<u32> {
        let spectrum = self.do_beamforming(iq_data);
        let max_value = spectrum.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        
        spectrum.iter()
            .enumerate()
            .filter(|(_, &power)| power > max_value - threshold)
            .map(|(angle, _)| angle as u32)
            .collect()
    }

    /// Seguimiento de fuente en movimiento
    pub fn track_moving_source(&mut self, iq_frames: &[Vec<Vec<Complex<f32>>>]) -> Vec<u32> {
        iq_frames.iter()
            .map(|frame| {
                let sources = self.find_signal_sources(frame, 3.0);
                sources.first().cloned().unwrap_or(360) // 360 = no detectado
            })
            .collect()
    }
}