use std::{
    process::Command,
    str::{
        self,
        FromStr
    }
};

#[derive(Debug)]
struct AVFoundationDevice {
    idx: u8,
    name: String,
    dtype: DeviceType,
}

#[derive(Debug)]
struct Devices {
    audio: Vec<AVFoundationDevice>,
    video: Vec<AVFoundationDevice>,
}

#[derive(Debug)]
enum DeviceType {
    Video,
    Audio,
    Unassigned,
}


impl FromStr for AVFoundationDevice {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (idx, name) = s
            .strip_prefix('[')
            .and_then(|s| s.split_once(' '))
            .ok_or(String::from("wrong formatting"))?;


        let idx = idx
            .strip_suffix(']')
            .expect("idx should still have a right bracket attached")
            .parse::<u8>().map_err(|_| String::from("idx is not a digit"))?;
        let name = String::from(name);

        Ok(AVFoundationDevice{ idx, name, dtype: DeviceType::Unassigned })
    }
}

pub fn get_devices() {
    let mut list_devices = Command::new("ffmpeg");
    list_devices.args(["-f", "avfoundation",
            "-list_devices", "true",
            "-i", "\"\""]);
    let output = list_devices.output()
        .expect("ffmpeg should be found");
    let text: &str = str::from_utf8(&output.stderr).unwrap();
    let text: Vec<&str> = text.split('\n').collect();
    let mut infos: Vec<&str> = vec![];
    for line in text {
        if line.starts_with("[AVFoundation") {
            if let Some(chunk) = line.find("] ") {
                infos.push(&line[chunk+2..]);
            } else {
                panic!("ending bracket should be found");
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
        let mut device = info.parse::<AVFoundationDevice>()
            .expect("at this stage, the line should be \
                    properly formatted");
        if video {
            device.dtype = DeviceType::Video;
            video_devices.push(device);
        } else {
            device.dtype = DeviceType::Audio;
            audio_devices.push(device);
        }
    }
    let devices: Devices = Devices {
        audio: audio_devices,
        video: video_devices,
    };
    println!("{:#?}", devices);
}
