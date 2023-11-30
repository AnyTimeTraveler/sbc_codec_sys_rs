// #![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ffi::{c_void, CStr};
use core::ptr;
// use crate::bindings::{sbc_decode, sbc_encode, sbc_finish, sbc_get_codesize, sbc_get_frame_duration, sbc_get_frame_length, sbc_get_implementation_info, sbc_init, sbc_t};
use crate::params::{AllocationMethod, Blocks, ByteOrder, ChannelMode, Frequency, Subbands};

mod params;
#[cfg(test)]
mod test;
// mod bindings;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub struct SBCEncoder {
    sbc: sbc_t,
}

/// Even though the same struct can be used in sbc_decode and sbc_encode,
/// it produces garbage output for some reason that I don't have time to investigate atm.
pub struct SBCDecoder {
    sbc: sbc_t,
}

impl SBCEncoder {
    pub fn new(
        frequency: Frequency,
        blocks: Blocks,
        channel_mode: ChannelMode,
        allocation_method: AllocationMethod,
        subbands: Subbands,
        byte_order: ByteOrder,
        bitpool_size: u8,
    ) -> SBCEncoder {
        let sbc = init_sbc(frequency, blocks, channel_mode, allocation_method, subbands, byte_order, bitpool_size);

        SBCEncoder {
            sbc,
        }
    }

    pub fn encode(
        &mut self,
        input: &[u8],
        output: &mut [u8],
        written: &mut isize,
    ) -> isize {
        unsafe {
            sbc_encode(
                ptr::addr_of_mut!(self.sbc),
                input.as_ptr().cast(),
                input.len(),
                output.as_mut_ptr() as *mut c_void,
                output.len(),
                written,
            )
        }
    }
}

impl SBCDecoder {
    pub fn new(
        frequency: Frequency,
        blocks: Blocks,
        channel_mode: ChannelMode,
        allocation_method: AllocationMethod,
        subbands: Subbands,
        byte_order: ByteOrder,
        bitpool_size: u8,
    ) -> SBCDecoder {
        let sbc = init_sbc(frequency, blocks, channel_mode, allocation_method, subbands, byte_order, bitpool_size);

        SBCDecoder {
            sbc,
        }
    }

    pub fn decode(
        &mut self,
        input: &[u8],
        output: &mut [u8],
        written: &mut usize,
    ) -> isize {
        unsafe {
            sbc_decode(
                ptr::addr_of_mut!(self.sbc),
                input.as_ptr().cast(),
                input.len(),
                output.as_mut_ptr() as *mut c_void,
                output.len(),
                written,
            )
        }
    }
}

fn init_sbc(frequency: Frequency, blocks: Blocks, channel_mode: ChannelMode, allocation_method: AllocationMethod, subbands: Subbands, byte_order: ByteOrder, bitpool_size: u8) -> sbc_struct {
    let mut sbc = sbc_t {
        flags: 0,
        frequency: 0,
        blocks: 0,
        subbands: 0,
        mode: 0,
        allocation: 0,
        bitpool: 0,
        endian: 0,
        priv_: ptr::null_mut(),
        priv_alloc_base: ptr::null_mut(),
    };
    unsafe {
        sbc_init(ptr::addr_of_mut!(sbc), 0);
    }

    sbc.frequency = frequency as u8;
    sbc.blocks = blocks as u8;
    sbc.subbands = subbands as u8;
    sbc.mode = channel_mode as u8;
    sbc.allocation = allocation_method as u8;
    sbc.bitpool = bitpool_size;
    sbc.endian = byte_order as u8;
    sbc
}

impl SBCDecoder {
    pub fn get_frame_length(&mut self) -> usize {
        unsafe {
            sbc_get_frame_length(ptr::addr_of_mut!(self.sbc))
        }
    }

    pub fn get_frame_duration(&mut self) -> u32 {
        unsafe {
            sbc_get_frame_duration(ptr::addr_of_mut!(self.sbc))
        }
    }

    pub fn get_codesize(&mut self) -> usize {
        unsafe {
            sbc_get_codesize(ptr::addr_of_mut!(self.sbc))
        }
    }

    pub fn get_implementation_info(&mut self) -> &'static str {
        unsafe {
            let info = sbc_get_implementation_info(ptr::addr_of_mut!(self.sbc));
            let c_str = CStr::from_ptr(info);
            c_str.to_str().unwrap()
        }
    }
}

impl SBCEncoder {
    pub fn get_frame_length(&mut self) -> usize {
        unsafe {
            sbc_get_frame_length(ptr::addr_of_mut!(self.sbc))
        }
    }

    pub fn get_frame_duration(&mut self) -> u32 {
        unsafe {
            sbc_get_frame_duration(ptr::addr_of_mut!(self.sbc))
        }
    }

    pub fn get_codesize(&mut self) -> usize {
        unsafe {
            sbc_get_codesize(ptr::addr_of_mut!(self.sbc))
        }
    }

    pub fn get_implementation_info(&mut self) -> &'static str {
        unsafe {
            let info = sbc_get_implementation_info(ptr::addr_of_mut!(self.sbc));
            let c_str = CStr::from_ptr(info);
            c_str.to_str().unwrap()
        }
    }
}

impl Drop for SBCDecoder {
    fn drop(&mut self) {
        unsafe {
            sbc_finish(ptr::addr_of_mut!(self.sbc));
        }
    }
}

impl Drop for SBCEncoder {
    fn drop(&mut self) {
        unsafe {
            sbc_finish(ptr::addr_of_mut!(self.sbc));
        }
    }
}
