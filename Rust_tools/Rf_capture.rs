// 📜 src/rf_capture.rs
use std::fs::File;
use std::io::BufWriter;
use hackrf::{HackRF, IQSample};

pub fn capture(frequency: u64, duration: u64) -> std::io::Result<()> {
    let mut radio = HackRF::open()?;
    radio.set_freq(frequency)?;
    radio.set_sample_rate(2_000_000)?;

    let file = File::create("capture.iq")?;
    let mut writer = BufWriter::new(file);

    radio.start_rx(|samples: &[IQSample]| {
        for sample in samples {
            writer.write_all(&sample.to_bytes())?;
        }
        Ok(())
    }, duration)?;

    println!("🎙️ Señal guardada en capture.iq");
    Ok(())
}
