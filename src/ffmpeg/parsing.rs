//! # Parsing
//!
//! Module to parse ffmpeg output and retrieve usable informations about
//! the devices
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
    fmt,
};

impl AVFoundationDevice {
    /// Parses a string into an AVFoundationDevice
    /// The format string has to be of the form:
    /// `[idx] device name`
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let test_string: &str = "[1] Test Device";
    /// let device = parse_device(test_string, DeviceType::Video)
    ///     .expect("test string is properly formatted");
    /// assert_eq!(device.name, "Test Device");
    /// ```
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
///
/// # Panics
///
/// - If ffmpeg output isn't valid `UTF-8`
/// - If ffmpeg output isn't properly formatted
pub(crate) fn get_devices() -> Devices {
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

impl Devices {

    /// Returns the index in the `Ok` variant if only one corresponding
    /// device is found. Returns `Err` otherwise.
    pub(super) fn get_index(&self, name: &str, dtype: DeviceType)
    -> Result<u8, String>
    {
        let mut result: Option<u8> = None;
        let to_search: &Vec<AVFoundationDevice> = if dtype == DeviceType::Audio {
            &self.audio
        } else {
            &self.video
        };
        for candidate in to_search {
            if ! candidate.name.starts_with(name) {
                continue;
            }
            if result.is_some() {
                return Err(
                    format!("Multiple {} devices found starting with name: '{}'",
                            dtype, name)
                );
            }
            result = Some(candidate.idx);
        }
        result.ok_or(format!("No {} device found starting with name: '{}'",
                             dtype, name))
    }
    pub(super) fn get_name(&self, index: usize, dtype: DeviceType)
    -> Result<String, String>
    {
        let to_search: &Vec<AVFoundationDevice> = if dtype == DeviceType::Audio {
            &self.audio
        } else {
            &self.video
        };
        if index >= to_search.len() {
            return Err(format!("Index: '{}' is greather than array \
of {} devices of length '{}'", index, dtype, to_search.len()));
        }
        Ok(to_search[index].name.clone())
    }
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match *self {
            DeviceType::Video => "video",
            DeviceType::Audio => "audio",
        };
        write!(f, "{s}")
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
