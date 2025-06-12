use std::fmt;

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


/// A struct to package all the stream information
/// in one place in order to make the launch easier
pub(crate) struct Stream<'a> {
    pub(super) video: &'a str,
    pub(super) audio: Option<&'a str>,
    pub(super) output: &'a str,
}

/// A Result for operations that can fail mid-way
#[must_use = "This `IResult` can be `Incomplete`, meaning resources should be freed
or `Err`, meaning an error should be handled"]
pub(crate) enum IResult<T,E> {
    Ok(T),
    Incomplete(T,E),
    Err(E),
}

impl<T,E> IResult<T,E> {
    pub(crate) fn is_err(&self) -> bool {
        match self {
            Self::Ok(_) => false,
            Self::Incomplete(_, _) => true,
            Self::Err(_) => true,
        }
    }

    /// Returns the contained [`Self::Ok`] value, consuming the `self` value
    /// Usable like a [`Result`] `unwrap` method.
    /// # Panics
    ///
    /// If the contained value is `Self::Incomplete` or `Self::Error`
    pub(crate) fn unwrap(self) -> T
    where
        E: fmt::Debug,
    {
        match self {
            Self::Ok(t) => t,
            Self::Incomplete(_,e) =>
                panic!("Call to unwrap on IResult::Incomplete: {e:?}"),
            Self::Err(e) =>
                panic!("Call to unwrap on IResult::Error: {e:?}"),
        }
    }
}
