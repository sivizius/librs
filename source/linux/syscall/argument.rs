use super::Pointer;

/// Something that could be used as an Argument of a `Syscall`.
/// If something is some pointerish thing, implement `Pointer` instead.
/// `Pointer` automatically implements `Argument.`
pub trait     Argument
{
  /// Converts the `Argument` into something,
  ///   that could be moved into a register to call the `Syscall`.
  ///
  /// # Examples
  /// …
  /// ```rust
  /// …
  /// ```
  fn            into
  (
    self,
  )
  ->  usize;
}

impl  < T >   Argument                  for T
where
  T:                                    Pointer,
{
  fn  into  ( self  ) ->  usize { self.into ( ) }
}

impl          Argument                  for bool
{
  fn  into  ( self  ) ->  usize { self                as  usize }
}

impl          Argument                  for char
{
  fn  into  ( self  ) ->  usize { self                as  usize }
}

impl          Argument                  for f32
{
  fn  into  ( self  ) ->  usize { self.to_bits  ( )   as  usize }
}

impl          Argument                  for f64
{
  fn  into  ( self  ) ->  usize { self.to_bits  ( )   as  usize }
}

impl          Argument                  for isize
{
  fn  into  ( self  ) ->  usize { self                as  usize }
}

impl          Argument                  for !
{
  fn  into  ( self  ) ->  usize { 0                             }
}

impl          Argument                  for ( )
{
  fn  into  ( self  ) ->  usize { 0                             }
}

impl          Argument                  for usize
{
  fn  into  ( self  ) ->  usize { self                          }
}
