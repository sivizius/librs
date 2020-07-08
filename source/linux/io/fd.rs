use super::super::types::CInt;

/// A Raw File Descriptor.
pub type      RawFd                     =   CInt;

/// Standard Input.
pub const   STDIN:  RawFd               =   0;

/// Standard Output.
pub const   STDOUT: RawFd               =   1;

/// Standard Error Output.
pub const   STDERR: RawFd               =   2;
