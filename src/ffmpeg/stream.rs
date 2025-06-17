use std::io::{self, Error, ErrorKind};
use std::{
    process::{Command, Stdio},
};
use super::{
    Devices,
    DeviceType::*,
    Child,
};

/// A struct to package all the stream information
/// in one place in order to make the launch easier
pub(crate) struct Stream {
    pub(super) video: String,
    pub(super) audio: Option<String>,
    pub(super) output: String,
    pub(super) path: Option<String>,
}

impl Stream {
    /// Creates a new `Stream`
    pub(crate) fn new(video: String, audio: Option<String>, output: String,
                      path: Option<String>)
-> Stream {
        Stream {
            video,
            audio,
            output,
            path,
        }
    }

    pub(crate) fn output_file(&self) -> String {
        let output = if let Some(path) = self.path.clone() {
            path.to_owned()
        } else {
            String::new()
        };
        output + "/" + &self.output
    }

    /// Launch recording for one stream
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use super::get_devices;
    ///
    /// let devices = get_devices();
    /// let stream = Stream::new(String::from("FaceTime"),
    ///     Some(String::from("MacBook Air Microphone")),
    ///     String::from("output.mp4"), None);
    /// let launch_result = stream.record(&Devices);
    /// ```
    pub(crate) fn record(&self, devices: &Devices, preview: bool) -> io::Result<Child>
    {
        let bin = if preview {
            "ffplay"
        } else {
            "ffmpeg"
        };
        let video = devices.get_index(&self.video, Video)
            .map_err(|e| Error::new(ErrorKind::Other, e))?;
        let framerate: u8 = 30;
        let mut ffmpeg_command = Command::new(bin);
        ffmpeg_command.arg("-f").arg("avfoundation");
        //Setting the framerate for displays that aren't the screen
        //TODO!: Fix (abstract) if it breaks
        if !self.video.starts_with("Capture screen") {
            ffmpeg_command.arg("-framerate").arg(framerate.to_string());
        }
        ffmpeg_command
            .arg("-video_device_index")
            .arg(video.to_string());
        if let Some(audio) = &self.audio {
            if ! preview {
                let audio = devices.get_index(audio, Audio)
                    .map_err(|e| Error::new(ErrorKind::Other, e))?;
                ffmpeg_command
                    .arg("-audio_device_index")
                    .arg(audio.to_string());
            }
        }
        ffmpeg_command.arg("-i").arg("\"\"");
        if ! preview {
            ffmpeg_command.arg(self.output_file());
        }
        ffmpeg_command.stderr(Stdio::null())
            .stdin(Stdio::null())
            .stdout(Stdio::null());
        let child = ffmpeg_command.spawn()?;
        Ok(Child::new(child))
    }

    pub(crate) fn set_path(self: &mut Self, path: &str) {
        self.path = Some(path.to_owned());
    }

    pub(crate) fn readable_name(&mut self, devices: &Devices)
    -> Result<(), String>
    {
        if let Ok(idx) = self.video.parse::<usize>() {
            self.video = devices.get_name(idx, Video)?;
        }
        if let Some(ref audio) = self.audio {
            if let Ok(idx) = audio.parse::<usize>() {
                self.audio = Some(devices.get_name(idx, Audio)?);
            }
        }
        Ok(())
    }

    pub(crate) fn display(self: &Self) {
        println!("---Stream---");
        println!("video: {}", self.video);
        println!("audio: {}", self.audio
                 .as_deref().unwrap_or_else(|| {"None"}));
        println!("output: {}", self.output);
        println!("output directory: {}", self.path.as_deref()
                 .unwrap_or_else(|| {"."}));
    }
}
