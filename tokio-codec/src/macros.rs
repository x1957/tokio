/// A macro to reduce some of the boilerplate for projecting from
/// `Pin<&mut T>` to `Pin<&mut T.field>`
macro_rules! pin {
    ($e:expr) => {
        std::pin::Pin::new(&mut $e)
    };
}

// This is a l facade that wraps the `tracing` APIs we use, so that when the
// `tracing` dependency is disabled, `tracing`'s macros expand to no-ops.
//
// This means we don't have to put a `#[cfg(feature = "tracing")]` on every
// individual use of a `tracing` macro.
#[cfg(not(feature = "tracing"))]
#[derive(Clone, Debug)]
pub(crate) struct Span {}

#[cfg(feature = "tracing")]
macro_rules! trace {
    ($($arg:tt)+) => {
        tracing::trace!($($arg)+)
    };
}

#[cfg(not(feature = "tracing"))]
macro_rules! trace {
    ($($arg:tt)+) => {};
}

#[cfg(feature = "tracing")]
macro_rules! debug {
    ($($arg:tt)+) => {
        tracing::debug!($($arg)+)
    };
}

#[cfg(feature = "tracing")]
macro_rules! trace_span {
    ($($arg:tt)+) => {
        tracing::trace_span!($($arg)+)
    };
}

#[cfg(not(feature = "tracing"))]
macro_rules! trace_span {
    ($($arg:tt)+) => {
        crate::macros::Span::new()
    };
}

#[cfg(not(feature = "tracing"))]
impl Span {
    pub(crate) fn new() -> Self {
        Span {}
    }

    pub(crate) fn enter(&self) -> Span {
        Span {}
    }
}
