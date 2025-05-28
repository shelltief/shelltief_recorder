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


pub(crate) struct Stream<'a> {
    pub(super) video: &'a str,
    pub(super) audio: Option<&'a str>,
    pub(super) output: &'a str,
}
