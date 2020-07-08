/// File Descriptors.
pub mod fd;
/// Flags for `open()`.
pub mod flags;
/// File Permission for `open()`.
pub mod mode;

/// Re-export `RawFd`.
pub use fd::RawFd     as RawFd;
/// Re-export `STDIN`.
pub use fd::STDIN     as STDIN;
/// Re-export `STDOUT`.
pub use fd::STDOUT    as STDOUT;
/// Re-export `STDERR`.
pub use fd::STDERR    as STDERR;
/// Re-export `Flags`.
pub use flags::Flags  as Flags;
/// Re-export `Mode`.
pub use mode::Mode    as Mode;
