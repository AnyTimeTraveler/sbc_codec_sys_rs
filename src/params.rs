/* sampling frequency */
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Frequency {
    SBC_FREQ_16000 = 0x00,
    SBC_FREQ_32000 = 0x01,
    SBC_FREQ_44100 = 0x02,
    SBC_FREQ_48000 = 0x03,
}

/* blocks */
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Blocks {
    SBC_BLK_4 = 0x00,
    SBC_BLK_8 = 0x01,
    SBC_BLK_12 = 0x02,
    SBC_BLK_16 = 0x03,
}

/* channel mode */
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum ChannelMode {
    SBC_MODE_MONO = 0x00,
    SBC_MODE_DUAL_CHANNEL = 0x01,
    SBC_MODE_STEREO = 0x02,
    SBC_MODE_JOINT_STEREO = 0x03,
}

/* allocation method */
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum AllocationMethod {
    SBC_AM_LOUDNESS = 0x00,
    SBC_AM_SNR = 0x01,
}

/* subbands */
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Subbands {
    SBC_SB_4 = 0x00,
    SBC_SB_8 = 0x01,
}

/* Data endianess */
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum ByteOrder {
    SBC_LE = 0x00,
    SBC_BE = 0x01,
}
