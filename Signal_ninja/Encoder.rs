// 📜 src/encoder.rs
use hound::{WavReader, WavWriter};
use std::fs::File;

pub fn hide_message(audio_path: &str, message: &str, output_path: &str) {
    let mut reader = WavReader::open(audio_path).unwrap();
    let spec = reader.spec();
    let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();

    // Convertir el mensaje a bits
    let msg_bits: Vec<u8> = message.bytes()
        .flat_map(|b| (0..8).map(move |i| (b >> i) & 1))
        .collect();

    // Ocultar bits en los LSBs de las muestras
    let mut encoded_samples = Vec::new();
    for (i, sample) in samples.iter().enumerate() {
        let new_sample = if i < msg_bits.len() {
            (sample & 0xFFFE) | (msg_bits[i] as i16)
        } else {
            *sample
        };
        encoded_samples.push(new_sample);
    }

    // Guardar el archivo con el mensaje oculto
    let mut writer = WavWriter::create(output_path, spec).unwrap();
    for sample in encoded_samples {
        writer.write_sample(sample).unwrap();
    }
    println!("🖤 Mensaje oculto en: {}", output_path);
}
