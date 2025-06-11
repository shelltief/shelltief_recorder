//! # Record
//!
//! Module that controls the recording
use super::{DeviceType::*, Devices, Stream, IResult, Children, Child};
use std::io::{self, Error, ErrorKind};
use std::process::{Command, Stdio};



/// Launch all streams at the same time and catch launch errors
///
/// # Examples
/// ```ignore
/// use super::get_devices;
///
/// let devices = get_devices();
/// let stream = Stream::new("FaceTime", Some("MacBook Air Microphone"), "output.mp4");
/// let stream1 = Stream::new("Capture screen 0", None, "screen.mp4");
/// let streams: Vec<Stream> = Vec::from([stream, stream1]);
/// let launch_result = launch(streams, devices);
/// ```
///
/// # Errors
///
/// If one of the `Child` doesn't launch properly

pub(crate) fn launch(streams: Vec<Stream>, devices: Devices)
-> IResult<Children, io::Error>
{
    if streams.is_empty() {
        return IResult::Err(Error::new(ErrorKind::Other, "No stream to launch"));
    }
    for i in 0..(streams.len() - 1) {
        for j in (i+1)..streams.len() {
            if streams[i].output == streams[j].output {
                return IResult::Err(Error::new(ErrorKind::Other,
                format!("Two output files with name '{}' found", streams[i].output)));
            }
        }
    }
    let mut children: Children = Children::new();
    for stream in streams {
        let child = stream.record(&devices);
        if child.is_err() {
            return IResult::Incomplete(children, child.unwrap_err());
        }
        children.push(child.unwrap())
    }
    IResult::Ok(children)
}

impl<'a> Stream<'a> {
    /// Creates a new `Stream`
    pub(crate) fn new(video: &'a str, audio: Option<&'a str>, output: &'a str)
-> Stream<'a> {
        Stream {
            video,
            audio,
            output,
        }
    }

    /// Launch recording for one stream
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use super::get_devices;
    ///
    /// let devices = get_devices();
    /// let stream = Stream::new("FaceTime", Some("MacBook Air Microphone"), "output.mp4");
    /// let launch_result = stream.record(&Devices);
    /// ```
    pub(crate) fn record(&self, devices: &Devices) -> io::Result<Child>
    {
        let video = devices.get_index(self.video, Video);
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
        if !self.video.starts_with("Capture screen") {
            ffmpeg_command.arg("-framerate").arg(framerate.to_string());
        }
        ffmpeg_command
            .arg("-video_device_index")
            .arg(video.to_string());
        if let Some(audio) = self.audio {
            let audio = devices.get_index(audio, Audio);
            if audio.is_err() {
                return Err(Error::new(ErrorKind::Other, audio.unwrap_err()));
            }
            println!("{:#?}", audio);
            let audio = audio.unwrap();
            ffmpeg_command
                .arg("-audio_device_index")
                .arg(audio.to_string());
        }
        ffmpeg_command.arg("-i").arg("\"\"")
            .arg(self.output);
        ffmpeg_command.stderr(Stdio::null())
            .stdin(Stdio::null())
            .stdout(Stdio::null());
        let child = ffmpeg_command.spawn()?;
        Ok(Child::new(child))
    }
}
