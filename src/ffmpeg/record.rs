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

