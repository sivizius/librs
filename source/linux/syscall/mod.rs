/// Something that could be used as an Argument of a `Syscall`.
pub mod argument;
/// Different Ways a Syscall could fail.
pub mod failure;
/// Some Pointer that could be used as an Argument of a `Syscall`.
pub mod pointer;
/// Return Value, if Syscall was successful.
pub mod success;
/// Safish wrapper for the `Syscall`s.
pub mod wrapper;

pub use argument::Argument  as  Argument;
pub use failure::Failure    as  Failure;
pub use pointer::Pointer    as  Pointer;
pub use success::Success    as  Success;

/// Implementation of `Syscall` for amd64.
pub mod amd64;
/// Architecture independend alias.
pub use amd64::Syscall      as  Syscall;

impl          Syscall
{
  /// Wrap the `result` of a `Syscall` into a `Result<Success,Failure>`.
  ///
  /// # Errors
  /// Fails if result is negativ.
  ///
  /// # Examples
  /// …
  /// ```rust
  /// …
  /// ```
  #[inline(always)]
  fn            wrap
  (
    result:                             isize,
  )
  ->  Result
      <
        Success,
        Failure,
      >
  {
    if result < 0
    {
      Err ( Failure::wrap     ( result            ) )
    }
    else
    {
      Ok  ( success::Success  ( result  as  usize ) )
    }
  }
}

/// Variadic and Unsafe call of `Syscall`s.
#[macro_export]
macro_rules!  unsafe_call
{
  (
    $Number:expr    $(  , )?
  )
  =>  {
        unsafe
        {
          $Number.call0 ( )
        }
      };
  (
    $Number:expr,
    $Argument0:expr $(  , )?
  )
  =>  {
        unsafe
        {
          $Number.call1
          (
            $Argument0,
          )
        }
      };
  (
    $Number:expr,
    $Argument0:expr,
    $Argument1:expr $(  , )?
  )
  =>  {
        unsafe
        {
          $Number.call2
          (
            $Argument0,
            $Argument1,
          )
        }
      };
  (
    $Number:expr,
    $Argument0:expr,
    $Argument1:expr,
    $Argument2:expr $(  , )?
  )
  =>  {
        unsafe
        {
          $Number.call3
          (
            $Argument0,
            $Argument1,
            $Argument2,
          )
        }
      };
  (
    $Number:expr,
    $Argument0:expr,
    $Argument1:expr,
    $Argument2:expr,
    $Argument3:expr $(  , )?
  )
  =>  {
        unsafe
        {
          $Number.call4
          (
            $Argument0,
            $Argument1,
            $Argument2,
            $Argument3,
          )
        }
      };
  (
    $Number:expr,
    $Argument0:expr,
    $Argument1:expr,
    $Argument2:expr,
    $Argument3:expr,
    $Argument4:expr $(  , )?
  )
  =>  {
        unsafe
        {
          $Number.call5
          (
            $Argument0,
            $Argument1,
            $Argument2,
            $Argument3,
            $Argument4,
          )
        }
      };
  (
    $Number:expr,
    $Argument0:expr,
    $Argument1:expr,
    $Argument2:expr,
    $Argument3:expr,
    $Argument4:expr,
    $Argument5:expr $(  , )?
  )
  =>  {
        unsafe
        {
          $Number.call6
          (
            $Argument0,
            $Argument1,
            $Argument2,
            $Argument3,
            $Argument4,
            $Argument5,
          )
        }
      };
}
