use std::fs::File;
use std::io::Write;
use hound::WavReader;
use crate::params::{AllocationMethod, Blocks, ByteOrder, ChannelMode, Frequency, Subbands};
use crate::SBC;

const WAV_FILE: &[u8] = include_bytes!("../audio_testfiles/AdhesiveWombat - Osmium.wav");

#[test]
fn decode_encode() {
    let reader = WavReader::new(WAV_FILE).unwrap();
    let spec = reader.spec();
    let mut sbc = SBC::new(
        match spec.sample_rate {
            48000 => Frequency::SBC_FREQ_48000,
            44100 => Frequency::SBC_FREQ_44100,
            32000 => Frequency::SBC_FREQ_32000,
            16000 => Frequency::SBC_FREQ_16000,
            invalid_rate => panic!("Invalid smaple rate: {}", invalid_rate)
        },
        Blocks::SBC_BLK_16,
        ChannelMode::SBC_MODE_DUAL_CHANNEL,
        AllocationMethod::SBC_AM_SNR,
        Subbands::SBC_SB_8,
        ByteOrder::SBC_LE,
        64,
    );

    drop(reader);

    let wav_data = &WAV_FILE[46..];
    let mut sbc_data = vec![0u8; WAV_FILE.len()];
    let codesize = sbc.get_codesize();
    let mut encoded_written = 1;
    let mut in_start = 0;
    let mut out_start = 0;
    while in_start + codesize< WAV_FILE.len() && encoded_written > 0 {
        let read = sbc.encode(&wav_data[in_start..in_start + codesize], &mut sbc_data[out_start..], &mut encoded_written);
        // println!("{} < {}  {}", in_start, WAV_FILE.len(), read);
        // println!("dec codesize: {codesize}   read: {read}   written: {encoded_written}");
        if encoded_written > 0 {
            in_start += read as usize;
            out_start += encoded_written as usize;
        }
    }

    let sbc_len = out_start;

    println!("\nwav len: {}", WAV_FILE.len());
    println!("sbc len: {}\n", sbc_len);

    let step_size = 8 * 1024 * 4;
    let mut decoded = vec![0u8; WAV_FILE.len()];
    let mut decoded_written = 1;
    let mut in_start = 0;
    let mut out_start = 0;

    while in_start < sbc_len && decoded_written > 0 {
        let read = sbc.decode(&sbc_data[in_start..in_start + step_size], &mut decoded[out_start..], &mut decoded_written);
        // println!("enc codesize: {codesize}   read: {read}   written: {decoded_written}");
        if decoded_written > 0 {
            in_start += read as usize;
            out_start += decoded_written;
        }
    }

    let mut result = File::create("tmp.wav").unwrap();
    result.write(&WAV_FILE[0..46]).unwrap();
    result.write_all(&decoded[0..out_start]).unwrap();
    result.flush().unwrap();
}