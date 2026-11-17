pub struct AudioFormat {
    pub sample_rate: u32,
    pub channels: u8,
    pub bits_per_sample: u8,
}

pub struct AudioDeviceState {
    pub connected: bool,
    pub temperature: Option<f32>,
    pub format: Option<AudioFormat>,
    pub output_muted: bool,
}
