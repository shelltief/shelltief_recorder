use std::io::{self, Error, ErrorKind};
use std::process::{Command, Stdio};
use super::{
    Devices,
    DeviceType::*,
    Child,
};

/// A struct to package all the stream information
/// in one place in order to make the launch easier
pub(crate) struct Stream<'a> {
    pub(super) video: &'a str,
    pub(super) audio: Option<&'a str>,
    pub(super) output: &'a str,
    pub(super) path: Option<&'a str>,
}

impl<'a> Stream<'a> {
    /// Creates a new `Stream`
    pub(crate) fn new(video: &'a str, audio: Option<&'a str>, output: &'a str,
                      path: Option<&'a str>)
-> Stream<'a> {
        Stream {
            video,
            audio,
            output,
            path,
        }
    }

    pub(crate) fn output_file(&self) -> String {
        let output = if let Some(path) = self.path {
            path.to_owned()
        } else {
            String::new()
        };
        output + "/" + self.output
    }

    /// Launch recording for one stream
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use super::get_devices;
    ///
    /// let devices = get_devices();
    /// let stream = Stream::new("FaceTime", Some("MacBook Air Microphone"), "output.mp4", None);
    /// let launch_result = stream.record(&Devices);
    /// ```
    pub(crate) fn record(&self, devices: &Devices) -> io::Result<Child>
    {
        let video = devices.get_index(self.video, Video)
            .map_err(|e| Error::new(ErrorKind::Other, e))?;
        #[cfg(debug_assertions)]
        println!("video device index: {:#?}", video);
        let framerate: u8 = 30;
        let mut ffmpeg_command = Command::new("ffmpeg");
        ffmpeg_command.arg("-f").arg("avfoundation");
        //Setting the framerate for displays that aren't the screen
        //TODO!: Fix (abstract) if it breaks
        if !self.video.starts_with("Capture screen") {
            ffmpeg_command.arg("-framerate").arg(framerate.to_string());
        }
        ffmpeg_command
            .arg("-video_device_index")
            .arg(video.to_string());
        if let Some(audio) = self.audio {
            let audio = devices.get_index(audio, Audio)
                .map_err(|e| Error::new(ErrorKind::Other, e))?;
            #[cfg(debug_assertions)]
            println!("{:#?}", audio);
            ffmpeg_command
                .arg("-audio_device_index")
                .arg(audio.to_string());
        }
        ffmpeg_command.arg("-i").arg("\"\"")
            .arg(self.output_file());
        ffmpeg_command.stderr(Stdio::null())
            .stdin(Stdio::null())
            .stdout(Stdio::null());
        let child = ffmpeg_command.spawn()?;
        Ok(Child::new(child))
    }
}
