use super::
{
  Syscall,
  super::
  {
    Argument,
    Failure,
    Success,
  },
};

impl          Syscall
{
  /// Calls the `Syscall` without arguments.
  ///
  /// # Errors
  /// Fails if result is negativ.
  ///
  /// # Safety
  /// # Examples
  /// …
  /// ```rust
  /// …
  /// ```
  #[inline(always)]
  pub unsafe fn call0
  (
    self,
  )
  ->  Result
      <
        Success,
        Failure,
      >
  {
    let result: isize;
    asm!
    (
      "syscall",
      in      ( "rax" ) 1,
      out     ( "rcx" ) _,
      out     ( "r11" ) _,
      lateout ( "rax" ) result,
    );
    Self::wrap  ( result  )
  }

  /// Calls the `Syscall` with one argument.
  ///
  /// # Errors
  /// Fails if result is negativ.
  ///
  /// # Safety
  /// # Examples
  /// …
  /// ```rust
  /// …
  /// ```
  #[inline(always)]
  pub unsafe fn call1
  (
    self,
    argument0:                          impl  Argument,
  )
  ->  Result
      <
        Success,
        Failure,
      >
  {
    let result: isize;
    asm!
    (
      "syscall",
      in      ( "rax" ) 1,
      in      ( "rdi" ) argument0.into  ( ),
      out     ( "rcx" ) _,
      out     ( "r11" ) _,
      lateout ( "rax" ) result,
    );
    Self::wrap  ( result  )
  }

  /// Calls the `Syscall` with two arguments.
  ///
  /// # Errors
  /// Fails if result is negativ.
  ///
  /// # Safety
  /// # Examples
  /// …
  /// ```rust
  /// …
  /// ```
  #[inline(always)]
  pub unsafe fn call2
  (
    self,
    argument0:                          impl  Argument,
    argument1:                          impl  Argument,
  )
  ->  Result
      <
        Success,
        Failure,
      >
  {
    let result: isize;
    asm!
    (
      "syscall",
      in      ( "rax" ) 1,
      in      ( "rdi" ) argument0.into  ( ),
      in      ( "rsi" ) argument1.into  ( ),
      out     ( "rcx" ) _,
      out     ( "r11" ) _,
      lateout ( "rax" ) result,
    );
    Self::wrap  ( result  )
  }

  /// Calls the `Syscall` with three arguments.
  ///
  /// # Errors
  /// Fails if result is negativ.
  ///
  /// # Safety
  /// # Examples
  /// …
  /// ```rust
  /// …
  /// ```
  #[inline(always)]
  pub unsafe fn call3
  (
    self,
    argument0:                          impl  Argument,
    argument1:                          impl  Argument,
    argument2:                          impl  Argument,
  )
  ->  Result
      <
        Success,
        Failure,
      >
  {
    let result: isize;
    asm!
    (
      "syscall",
      in      ( "rax" ) 1,
      in      ( "rdi" ) argument0.into  ( ),
      in      ( "rsi" ) argument1.into  ( ),
      in      ( "rdx" ) argument2.into  ( ),
      out     ( "rcx" ) _,
      out     ( "r11" ) _,
      lateout ( "rax" ) result,
    );
    Self::wrap  ( result  )
  }

  /// Calls the `Syscall` with four arguments.
  ///
  /// # Errors
  /// Fails if result is negativ.
  ///
  /// # Safety
  /// # Examples
  /// …
  /// ```rust
  /// …
  /// ```
  #[inline(always)]
  pub unsafe fn call4
  (
    self,
    argument0:                          impl  Argument,
    argument1:                          impl  Argument,
    argument2:                          impl  Argument,
    argument3:                          impl  Argument,
  )
  ->  Result
      <
        Success,
        Failure,
      >
  {
    let result: isize;
    asm!
    (
      "syscall",
      in      ( "rax" ) 1,
      in      ( "rdi" ) argument0.into  ( ),
      in      ( "rsi" ) argument1.into  ( ),
      in      ( "rdx" ) argument2.into  ( ),
      in      ( "r10" ) argument3.into  ( ),
      out     ( "rcx" ) _,
      out     ( "r11" ) _,
      lateout ( "rax" ) result,
    );
    Self::wrap  ( result  )
  }

  /// Calls the `Syscall` with five arguments.
  ///
  /// # Errors
  /// Fails if result is negativ.
  ///
  /// # Safety
  /// # Examples
  /// …
  /// ```rust
  /// …
  /// ```
  #[inline(always)]
  pub unsafe fn call5
  (
    self,
    argument0:                          impl  Argument,
    argument1:                          impl  Argument,
    argument2:                          impl  Argument,
    argument3:                          impl  Argument,
    argument4:                          impl  Argument,
  )
  ->  Result
      <
        Success,
        Failure,
      >
  {
    let result: isize;
    asm!
    (
      "syscall",
      in      ( "rax" ) 1,
      in      ( "rdi" ) argument0.into  ( ),
      in      ( "rsi" ) argument1.into  ( ),
      in      ( "rdx" ) argument2.into  ( ),
      in      ( "r10" ) argument3.into  ( ),
      in      ( "r8"  ) argument4.into  ( ),
      out     ( "rcx" ) _,
      out     ( "r11" ) _,
      lateout ( "rax" ) result,
    );
    Self::wrap  ( result  )
  }

  /// Calls the `Syscall` with six arguments.
  ///
  /// # Errors
  /// Fails if result is negativ.
  ///
  /// # Safety
  /// # Examples
  /// …
  /// ```rust
  /// …
  /// ```
  #[inline(always)]
  pub unsafe fn call6
  (
    self,
    argument0:                          impl  Argument,
    argument1:                          impl  Argument,
    argument2:                          impl  Argument,
    argument3:                          impl  Argument,
    argument4:                          impl  Argument,
    argument5:                          impl  Argument,
  )
  ->  Result
      <
        Success,
        Failure,
      >
  {
    let result: isize;
    asm!
    (
      "syscall",
      in      ( "rax" ) 1,
      in      ( "rdi" ) argument0.into  ( ),
      in      ( "rsi" ) argument1.into  ( ),
      in      ( "rdx" ) argument2.into  ( ),
      in      ( "r10" ) argument3.into  ( ),
      in      ( "r8"  ) argument4.into  ( ),
      in      ( "r9"  ) argument5.into  ( ),
      out     ( "rcx" ) _,
      out     ( "r11" ) _,
      lateout ( "rax" ) result,
    );
    Self::wrap  ( result  )
  }
}
