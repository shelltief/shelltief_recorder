use std::process::{
    Command,
    Child,
};
use std::io::{
    self,
    Error,
    ErrorKind,
};
use super::{
    AVFoundationDevice,
    DeviceType::*,
    Devices,
};

pub(crate) fn record(video_device: &str,
           audio_device: Option<&str>,
           devices: Devices,
           output_name: &str)
-> io::Result<Child>
{
    let video = devices.get_index(video_device, Video);
    if video.is_err() {
        return Err(Error::new(ErrorKind::Other, video.unwrap_err()));
    }
    println!("{:#?}", video);
    let video = video.unwrap();
    let framerate: u8 = 30;
    let mut ffmpeg_command = Command::new("ffmpeg");
    ffmpeg_command.arg("-f").arg("avfoundation");
    //Setting the framerate for displays that aren't the screen
    //TODO!: Fix (abstract) if it breaks
    if ! video_device.starts_with("Capture screen") {
        ffmpeg_command.arg("-framerate").arg(framerate.to_string());
    }
    ffmpeg_command.arg("-video_device_index").arg(video.to_string());
    if let Some(audio) = audio_device {
        let audio = devices.get_index(audio, Audio);
        if audio.is_err(){
            return Err(Error::new(ErrorKind::Other, audio.unwrap_err()));
        }
        println!("{:#?}", audio);
        let audio = audio.unwrap();
        ffmpeg_command.arg("-audio_device_index")
            .arg(audio.to_string());
    }
    ffmpeg_command.arg("-i \"\"").arg(output_name);
    println!("{:#?}", ffmpeg_command);
    //ffmpeg_command.spawn()
    Command::new("echo").arg("bonjour").spawn()
}
