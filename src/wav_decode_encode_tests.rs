use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use hound::{WavReader, WavWriter};
use crate::params::{AllocationMethod, Blocks, ByteOrder, ChannelMode, Frequency, Subbands};
use crate::{SBCDecoder, SBCEncoder};

const WAV_FILE: &[u8] = include_bytes!("../audio_testfiles/AdhesiveWombat - Osmium.wav");

#[test]
fn decode_encode() {
    let bitpool = 44;
    let encoded_length = 64 * 1024;
    let mode = ChannelMode::SBC_MODE_STEREO;
    let method = AllocationMethod::SBC_AM_LOUDNESS;
    let subbands = Subbands::SBC_SB_4;
    let input_file = WAV_FILE;
    let output_file = "tmp.wav";

    sbc_encode_decode(bitpool, encoded_length, mode, method, subbands, input_file, output_file);
}

#[test]
fn decode_encode_all_bitpools() {
    let encoded_length = 64 * 1024;
    let mode = ChannelMode::SBC_MODE_STEREO;
    let method = AllocationMethod::SBC_AM_LOUDNESS;
    let subbands = Subbands::SBC_SB_4;
    let input_file = WAV_FILE;

    for bitpool in 1..64 {
        let output_file_bitpool = format!("tmp.{:02}.wav", bitpool);
        sbc_encode_decode(bitpool, encoded_length, mode, method, subbands, input_file, &output_file_bitpool);
    }
}

fn sbc_encode_decode(bitpool: u8, encoded_length: usize, mode: ChannelMode, method: AllocationMethod, subbands: Subbands, input_file: &[u8], output_file: &str) {
    let reader = WavReader::new(input_file).unwrap();
    let spec = reader.spec();
    let mut sbc = SBCEncoder::new(
        match spec.sample_rate {
            48000 => Frequency::SBC_FREQ_48000,
            44100 => Frequency::SBC_FREQ_44100,
            32000 => Frequency::SBC_FREQ_32000,
            16000 => Frequency::SBC_FREQ_16000,
            invalid_rate => panic!("Invalid sample rate: {}", invalid_rate)
        },
        Blocks::SBC_BLK_16,
        mode,
        method,
        subbands,
        ByteOrder::SBC_LE,
        bitpool,
    );

    drop(reader);

    let wav_data = &WAV_FILE[46..];
    let mut sbc_data = vec![0u8; encoded_length];
    let codesize = sbc.get_codesize();
    let mut encoded_written = 1;
    let mut in_start = 0;
    let mut out_start = 0;
    while in_start + codesize < WAV_FILE.len() && encoded_written > 0 {
        let x = &wav_data[in_start..];
        let x1 = &mut sbc_data[out_start..];
        let read = sbc.encode(x, x1, &mut encoded_written);
        // println!("SBC encode    read: {}   written: {}", read, encoded_written);
        if encoded_written > 0 {
            in_start += read as usize;
            out_start += encoded_written as usize;
        }
    }

    drop(sbc);
    let mut sbc = SBCDecoder::new(
        match spec.sample_rate {
            48000 => Frequency::SBC_FREQ_48000,
            44100 => Frequency::SBC_FREQ_44100,
            32000 => Frequency::SBC_FREQ_32000,
            16000 => Frequency::SBC_FREQ_16000,
            invalid_rate => panic!("Invalid sample rate: {}", invalid_rate)
        },
        Blocks::SBC_BLK_16,
        mode,
        method,
        subbands,
        ByteOrder::SBC_LE,
        bitpool,
    );
    let sbc_len = out_start;

    // println!("\nwav len: {}", WAV_FILE.len());
    // println!("sbc len: {}\n", sbc_len);

    let mut decoded = vec![0u8; WAV_FILE.len()];
    let mut decoded_written = 1;
    let mut in_start = 0;
    let mut out_start = 0;

    while in_start < sbc_len && decoded_written > 0 {
        // println!("{in_start} < {sbc_len}");
        let read = sbc.decode(&sbc_data[in_start..], &mut decoded[out_start..], &mut decoded_written);
        // println!("sbc decode    read: {}   written: {}", read, decoded_written);
        if decoded_written > 0 {
            in_start += read as usize;
            out_start += decoded_written;
        }
    }

    let result = File::create(output_file).unwrap();
    let mut writer = WavWriter::new(result, spec).unwrap();
    writer.flush().unwrap();
    drop(writer);

    let mut result = File::options().write(true).open(output_file).unwrap();
    result.seek(SeekFrom::Start(4)).unwrap();
    let length = out_start as u32 + 36;
    // println!("174628 - {} = {}", length, 174628 - length);
    result.write_all(&length.to_le_bytes()).unwrap();
    result.seek(SeekFrom::Start(40)).unwrap();
    let length = out_start as u32;
    result.write_all(&length.to_le_bytes()).unwrap();
    result.write_all(&decoded[0..out_start]).unwrap();
    result.flush().unwrap();
}
