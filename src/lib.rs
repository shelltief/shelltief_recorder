use std::{
    process::Command,
    str::{
        self,
        FromStr
    }
};

pub fn run(){
 
    println!("Hello World!");
}

#[derive(Debug)]
struct AVFoundationDevice {
    id: u8,
    name: String,
}


impl FromStr for AVFoundationDevice {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, name) = s
            .strip_prefix('[')
            .and_then(|s| s.split_once(' '))
            .ok_or(String::from("wrong formatting"))?;


        let id = id
            .strip_suffix(']')
            .expect("id should still have a right bracket attached")
            .parse::<u8>().map_err(|_| String::from("id is not a digit"))?;
        let name = String::from(name);

        Ok(AVFoundationDevice{ id, name })
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
        let device = info.parse::<AVFoundationDevice>()
            .expect("at this stage, the line should be \
                    properly formatted");
        if video {
            video_devices.push(device);
        } else {
            audio_devices.push(device);
        }
    }
    println!("{:#?}", video_devices);
    println!("{:#?}", audio_devices);
}
