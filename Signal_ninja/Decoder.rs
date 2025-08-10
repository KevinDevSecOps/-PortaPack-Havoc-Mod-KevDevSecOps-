// 📜 src/decoder.rs
use hound::WavReader;

pub fn extract_message(audio_path: &str, msg_length: usize) -> String {
    let reader = WavReader::open(audio_path).unwrap();
    let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();

    // Extraer bits LSB
    let mut msg_bytes = Vec::new();
    for chunk in samples.chunks_exact(8).take(msg_length) {
        let byte = chunk.iter()
            .enumerate()
            .fold(0, |acc, (i, s)| acc | ((s & 1) << i));
        msg_bytes.push(byte as u8);
    }

    String::from_utf8_lossy(&msg_bytes).into_owned()
}
