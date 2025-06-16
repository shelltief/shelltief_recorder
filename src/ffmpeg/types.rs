
#[derive(Debug, PartialEq)]
pub(super) struct AVFoundationDevice {
    pub(super) idx: u8,
    pub(super) name: String,
    pub(super) dtype: DeviceType,
}

#[derive(Debug)]
pub(crate) struct Devices {
    pub(super) audio: Vec<AVFoundationDevice>,
    pub(super) video: Vec<AVFoundationDevice>,
}

#[derive(Debug, PartialEq)]
pub(super) enum DeviceType {
    Video,
    Audio,
}

impl Devices {
    pub(crate) fn list(&self) {
        println!("---Video Devices---");
        for device in &self.video {
            println!("Name: {} -- Index: {}", &device.name, &device.idx);
        }
        println!("---Audio Devices---");
        for device in &self.audio {
            println!("Name: {} -- Index: {}", &device.name, &device.idx);
        }
    }
}
