use rustfft::{FftPlanner, num_complex::Complex};
use ndarray::{Array1, Array2};
use rayon::prelude::*;

pub struct AdvancedSignalProcessor {
    fft_planner: FftPlanner<f32>,
}

impl AdvancedSignalProcessor {
    pub fn new() -> Self {
        Self {
            fft_planner: FftPlanner::new(),
        }
    }

    /// Análisis espectral con ventaneado
    pub fn spectral_analysis(&mut self, iq_data: &[Complex<f32>], window_size: usize) -> Array1<f32> {
        let fft = self.fft_planner.plan_fft_forward(window_size);
        let mut output = vec![Complex::new(0.0, 0.0); window_size];
        
        iq_data.chunks(window_size)
            .map(|chunk| {
                let mut buffer = chunk.to_vec();
                buffer.resize(window_size, Complex::new(0.0, 0.0));
                
                fft.process(&mut buffer);
                
                buffer.iter()
                    .map(|c| c.norm_sqr())
                    .collect::<Vec<f32>>()
            })
            .fold(Array1::zeros(window_size), |acc, spectrum| {
                &acc + &Array1::from_vec(spectrum)
            })
    }

    /// Filtrado adaptativo LMS
    pub fn adaptive_lms_filter(&self, signal: &[f32], reference: &[f32], step_size: f32) -> Vec<f32> {
        let filter_order = 32;
        let mut weights = vec![0.0; filter_order];
        let mut output = Vec::with_capacity(signal.len());

        for i in 0..signal.len() {
            let mut estimation = 0.0;
            for j in 0..filter_order {
                if i >= j {
                    estimation += weights[j] * signal[i - j];
                }
            }

            if i < reference.len() {
                let error = reference[i] - estimation;
                
                // Actualizar pesos
                for j in 0..filter_order {
                    if i >= j {
                        weights[j] += step_size * error * signal[i - j];
                    }
                }
            }

            output.push(estimation);
        }

        output
    }

    /// Detección de anomalías en tiempo real
    pub fn real_time_anomaly_detection(&self, iq_stream: &[Complex<f32>]) -> Vec<usize> {
        let window_size = 1024;
        let threshold = 3.0; // 3 sigma
        
        iq_stream.par_chunks(window_size)
            .enumerate()
            .filter_map(|(chunk_idx, chunk)| {
                let magnitudes: Vec<f32> = chunk.iter().map(|c| c.norm()).collect();
                let mean = magnitudes.iter().sum::<f32>() / magnitudes.len() as f32;
                let std_dev = (magnitudes.iter()
                    .map(|&x| (x - mean).powi(2))
                    .sum::<f32>() / magnitudes.len() as f32).sqrt();

                let anomalies = magnitudes.iter()
                    .enumerate()
                    .filter(|(_, &x)| (x - mean).abs() > threshold * std_dev)
                    .map(|(pos, _)| chunk_idx * window_size + pos)
                    .collect::<Vec<usize>>();

                if anomalies.is_empty() {
                    None
                } else {
                    Some(anomalies)
                }
            })
            .flatten()
            .collect()
    }
}