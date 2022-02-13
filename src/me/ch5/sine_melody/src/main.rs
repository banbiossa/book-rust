use hound::WavWriter;
use std::f32::consts::PI;
use std::io::{Seek, Write};

const SAMPLE_RATE: f32 = 44_100.0;
const BPM: f32 = 120.0; // tempo

#[allow(unused_variables)]
fn main() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut fw = WavWriter::create("melody.wav", spec).unwrap();
    let (c4, d4, e4, f4) = (261.63, 293.66, 329.63, 349.23);
    let (g4, a4, b4, c5) = (392.00, 440.00, 493.88, 523.25);

    let l4 = ((60.0 / BPM) * SAMPLE_RATE) as u32;
    let l2 = l4 * 2;
    // melody
    write_tone(&mut fw, c4, l4);
    write_tone(&mut fw, d4, l4);
    write_tone(&mut fw, e4, l2);
    write_tone(&mut fw, c4, l4);
    write_tone(&mut fw, d4, l4);
    write_tone(&mut fw, e4, l2);
    write_tone(&mut fw, g4, l4);
    write_tone(&mut fw, e4, l4);
    write_tone(&mut fw, d4, l4);
    write_tone(&mut fw, c4, l4);
    write_tone(&mut fw, d4, l4);
    write_tone(&mut fw, e4, l4);
    write_tone(&mut fw, d4, l2);
}

fn write_tone<W>(fw: &mut WavWriter<W>, tone: f32, len: u32)
where
    W: Write + Seek,
{
    for t in 0..len {
        let a = t as f32 / SAMPLE_RATE;
        let v = (a * tone * 2.0 * PI).sin();
        fw.write_sample((v * i16::MAX as f32) as i16).unwrap();
    }
}
