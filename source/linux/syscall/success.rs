use core::fmt;

/// Return Value, if Syscall was successful.
#[derive(Debug)]
pub struct    Success                   ( pub ( super ) usize );

impl          fmt::Display              for Success
{
  fn            fmt
  (
    &self,
    formatter:                          &mut fmt::Formatter,
  )
  ->  fmt::Result
  {
    write!
    (
      formatter,
      "{:?}",
      self,
    )
  }
}
