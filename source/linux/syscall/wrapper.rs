use super::
{
  Failure,
  Pointer,
  Success,
  Syscall,
  super::io::
  {
    Flags,
    Mode,
    RawFd,
  },
};

impl          Syscall
{
  /// Safeish wrapper for `Read`.
  /// Read from `count` bytes from file descriptor `file` into `buffer`.
  ///
  /// # Errors
  /// Fails if result is negativ.
  ///
  /// # Safety
  /// Assuming this `Syscall` exists on this machine,
  ///   is conforming to the standard and
  ///   the `buffer` is large enough,
  ///     this wrapper is safe.
  ///
  /// # Examples
  /// Read 16 bytes from Standard Input to `buffer`:
  /// ```rust
  ///   let buffer = [u8;16];
  ///   let length = Syscall::read(fs::STDIN, &buffer, buffer.len());
  /// ```
  #[inline(always)]
  pub fn        read                    < T >
  (
    file:                               RawFd,
    buffer:                             T,
    count:                              usize,
  )
  ->  Result
      <
        Success,
        Failure,
      >
  where
    T:  Pointer,
  {
    unsafe
    {
      Syscall::Read
      .call3
      (
        file as usize,
        buffer,
        count,
      )
    }
  }

  /// Safeish wrapper for `Write`.
  /// Write `count` bytes from `buffer` to file descriptor `file`.
  ///
  /// # Errors
  /// Fails if result is negativ.
  ///
  /// # Safety
  /// Assuming this `Syscall` exists on this machine,
  ///   is conforming to the standard and
  ///   the `buffer` is large enough,
  ///     this wrapper is safe.
  ///
  /// # Examples
  /// Write »Hello World« to Standard Output:
  /// ```rust
  ///   let buffer = "HelloWorld\n";
  ///   let length = Syscall::write(fs::STDOUT, &buffer, buffer.len());
  /// ```
  #[inline(always)]
  pub fn        write                   < T >
  (
    file:                               RawFd,
    buffer:                             T,
    count:                              usize,
  )
  ->  Result
      <
        Success,
        Failure,
      >
  where
    T:  Pointer,
  {
    unsafe
    {
      Syscall::Write
      .call3
      (
        file as usize,
        buffer,
        count,
      )
    }
  }

  /// Safeish wrapper for `Open`.
  /// Open and possibly create the file `pathName` with given `flags`.
  /// Set permissions to `mode` if creating the file.
  ///
  /// # Errors
  /// Fails if result is negativ.
  ///
  /// # Safety
  /// Assuming this `Syscall` exists on this machine,
  ///   is conforming to the standard and
  ///   the `pathName` is null-terminated,
  ///     this wrapper is safe.
  ///
  /// # Examples
  /// Read 16 bytes from Standard Input to `buffer`:
  /// ```rust
  ///   let buffer = [u8;16];
  ///   let length = Syscall::read(fs::STDIN, &buffer, buffer.len());
  /// ```
  #[inline(always)]
  pub fn        open
  (
    pathName:                           &str,
    flags:                              Flags,
    mode:                               Mode,
  )
  ->  Result
      <
        Success,
        Failure,
      >
  {
    unsafe
    {
      Syscall::Open
      .call3
      (
        pathName,
        flags,
        mode,
      )
    }
  }
}
