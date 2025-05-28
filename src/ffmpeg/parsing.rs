use super::{
    AVFoundationDevice,
    DeviceType,
    Devices,
};
use std::{
    process::Command,
    str::{
        self,
    },
};

/// Parses a string into an AVFoundationDevice
/// The format string has to be of the form:
/// `[idx] device name`
impl AVFoundationDevice {
    fn parse_device(s: &str, dtype: DeviceType) -> Result<Self, String> {
        let (idx, name) = s
            .strip_prefix('[')
            .and_then(|s| s.split_once(' '))
            .ok_or(String::from("wrong formatting"))?;


        let idx = idx
            .strip_suffix(']')
            .ok_or(String::from("idx should still have a right bracket attached"))?
            .parse::<u8>().map_err(|_| String::from("idx is not an integer"))?;
        let name = String::from(name);

        Ok(AVFoundationDevice{ idx, name, dtype })
    }
}

/// Retrieves the avfoundation devices in a custom structure that
/// is made of two vectors of devices.
/// Each device is retrieved by parsing the output of the command:
/// `ffmeg -f avfoundation -list_devices true -i ""`
/// The function then iterates on all the devices and parses it into
/// a `AVFoundationDevice` and later tags it as a `audio` or `video`
/// device.
pub fn get_devices() -> Devices {
    let mut list_devices = Command::new("ffmpeg");
    list_devices.args(["-f", "avfoundation",
            "-list_devices", "true",
            "-i", "\"\""]);
    let output = list_devices.output()
        .expect("ffmpeg should be found");
    let raw_output: &str = str::from_utf8(&output.stderr).unwrap();
    let text: Vec<&str> = raw_output.split('\n').collect();
    let mut infos: Vec<&str> = vec![];
    for line in text {
        if line.starts_with("[AVFoundation") {
            if let Some(chunk) = line.find("] ") {
                infos.push(&line[chunk+2..]);
            } else {
                unreachable!("Ending bracket should be found, ffmpeg line is \
corrupted: '{}'", &line);
            }
        }
    }
    let mut video_devices: Vec<AVFoundationDevice> = vec![];
    let mut audio_devices: Vec<AVFoundationDevice> = vec![];
    let mut video: bool = false;
    for info in infos {
        if info == "AVFoundation video devices:" {
            video = true;
            continue;
        }
        if info == "AVFoundation audio devices:" {
            video = false;
            continue;
        }
        if video {
        let device = AVFoundationDevice::parse_device(&info, DeviceType::Video)
            .expect("at this stage, the line should be properly formatted");
            video_devices.push(device);
        } else {
        let device = AVFoundationDevice::parse_device(&info, DeviceType::Audio)
            .expect("at this stage, the line should be properly formatted");
            audio_devices.push(device);
        }
    }
    Devices {
        audio: audio_devices,
        video: video_devices,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn device_parsing() {
        let test: String = String::from("[10] Camera");
        let device = AVFoundationDevice::parse_device(&test, DeviceType::Unassigned)
            .unwrap();
        assert_eq!(device.idx, 10);
        assert_eq!(device.name, "Camera");
        assert_eq!(device.dtype, DeviceType::Unassigned);
    }

    #[test]
    #[should_panic(expected="idx is not an integer")]
    fn wrong_idx() {
        let test: String = String::from("[a] Camera");
        let _device = AVFoundationDevice::parse_device(&test, DeviceType::Unassigned)
            .unwrap();
    }

    #[test]
    #[should_panic(expected="wrong formatting")]
    fn no_space() {
        let test: String = String::from("camera");
        let _device = AVFoundationDevice::parse_device(&test, DeviceType::Unassigned)
            .unwrap();
    }

    #[test]
    #[should_panic(expected="idx should still have a right bracket attached")]
    fn no_bracket() {
        let test: String = String::from("[10 Test String");
        let _device = AVFoundationDevice::parse_device(&test, DeviceType::Unassigned)
            .unwrap();
    }
}
