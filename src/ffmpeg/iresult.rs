use std::fmt;

/// A Result for operations that can fail mid-way
#[must_use = "This `IResult` can be `Incomplete`, meaning resources should be freed
or `Err`, meaning an error should be handled"]
pub(crate) enum IResult<T,E> {
    Ok(T),
    Incomplete(T,E),
    Err(E),
}

#[allow(dead_code)]
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
    /// If the contained value is [`Self::Incomplete`] or [`Self::Error`]
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

    /// Returns the contained [`Self::Ok`] value, consuming the `self` value
    /// takes a closure for the eventual cleanup of the `T` value
    ///
    /// # Panics
    ///
    /// If the contained value is [`Self::Incomplete`] or [`Self::Error`]
    pub(crate) fn unwrap_with<F>(self, cleanup: F) -> T
    where
        F: FnOnce(T),
        E: fmt::Debug,
    {
        match self {
            Self::Ok(t) => t,
            Self::Incomplete(t, e) => {
                cleanup(t);
                panic!("Call to unwrap on IResult::Incomplete: {e:?}");
            },
            Self::Err(e) =>
                panic!("Call to unwrap on IResult::Error: {e:?}"),
        }
    }
}
