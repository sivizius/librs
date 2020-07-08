/// Some Pointer that could be used as an Argument of a `Syscall`.
/// `Pointer` automatically implements `Argument.`
pub trait     Pointer
{
  /// Converts the `Pointer` into something,
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

macro_rules!  implArray
{
  ( $(  $Amount: expr,  )+  )
  =>  {
        $(
          impl  < T >   Pointer         for [ T;  $Amount ]
          {
            fn  into  ( self  ) ->  usize { self.as_ptr ( ) as  usize }
          }
        )+
      };
}

implArray!
(
  0,  1,  2,  3,  4,  5,  6,  7,  8,  9,  10, 11, 12, 13, 14, 15,
  16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
  32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
  48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
);

macro_rules!  implFunctionArgument
{
    (
      $FnTy: ty,
      $(
        $Arg: ident
      ),*
    )
    =>  {
          impl<Ret, $($Arg),*> Pointer  for $FnTy
          {
            fn  into  ( self  ) -> usize  { self as usize }
          }
        };
}

macro_rules!  implFunction
{
    (
      $(
        $Arg: ident
      ),+
    )
    =>  {
          implFunctionArgument! { extern        "Rust"  fn  ( $(  $Arg  ),+       ) ->  Ret,  $(  $Arg  ),+ }
          implFunctionArgument! { extern        "C"     fn  ( $(  $Arg  ),+       ) ->  Ret,  $(  $Arg  ),+ }
          implFunctionArgument! { extern        "C"     fn  ( $(  $Arg  ),+ , ... ) ->  Ret,  $(  $Arg  ),+ }
          implFunctionArgument! { unsafe extern "Rust"  fn  ( $(  $Arg  ),+       ) ->  Ret,  $(  $Arg  ),+ }
          implFunctionArgument! { unsafe extern "C"     fn  ( $(  $Arg  ),+       ) ->  Ret,  $(  $Arg  ),+ }
          implFunctionArgument! { unsafe extern "C"     fn  ( $(  $Arg  ),+ , ... ) ->  Ret,  $(  $Arg  ),+ }
        };
    ()
    =>  {
          // No variadic functions with 0 parameters
          implFunctionArgument! { extern        "Rust"  fn  ( ) ->  Ret,  }
          implFunctionArgument! { extern        "C"     fn  ( ) ->  Ret,  }
          implFunctionArgument! { unsafe extern "Rust"  fn  ( ) ->  Ret,  }
          implFunctionArgument! { unsafe extern "C"     fn  ( ) ->  Ret,  }
        };
}

implFunction! { }
implFunction! { A }
implFunction! { A, B }
implFunction! { A, B, C }
implFunction! { A, B, C, D }
implFunction! { A, B, C, D, E }
implFunction! { A, B, C, D, E, F }
implFunction! { A, B, C, D, E, F, G }
implFunction! { A, B, C, D, E, F, G, H }
implFunction! { A, B, C, D, E, F, G, H, I }
implFunction! { A, B, C, D, E, F, G, H, I, J }
implFunction! { A, B, C, D, E, F, G, H, I, J, K }
implFunction! { A, B, C, D, E, F, G, H, I, J, K, L }
implFunction! { A, B, C, D, E, F, G, H, I, J, K, L, M }
implFunction! { A, B, C, D, E, F, G, H, I, J, K, L, M, N }
implFunction! { A, B, C, D, E, F, G, H, I, J, K, L, M, N, O }
implFunction! { A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P }
implFunction! { A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q }
implFunction! { A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R }
implFunction! { A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S }
implFunction! { A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T }
implFunction! { A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U }
implFunction! { A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V }
implFunction! { A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W }
implFunction! { A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X }
implFunction! { A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y }
implFunction! { A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z }

impl  < T >   Pointer                   for *const  T
{
  fn  into  ( self  ) ->  usize { self                as  usize }
}

impl  < T >   Pointer                   for *mut    T
{
  fn  into  ( self  ) ->  usize { self                as  usize }
}

impl  < T >   Pointer                   for &T
{
  fn  into  ( self  ) ->  usize { self  as  *const  T as usize }
}

impl  < T >   Pointer                   for &mut T
{
  fn  into  ( self  ) ->  usize { self  as  *mut    T as usize }
}

impl  < T >   Pointer                   for &[  T ]
{
  fn  into  ( self  ) ->  usize { self.as_ptr ( )     as  usize }
}

impl  < T >   Pointer                   for &mut  [ T ]
{
  fn  into  ( self  ) ->  usize { self.as_ptr ( )     as  usize }
}

impl          Pointer                   for &str
{
  fn  into  ( self  ) ->  usize { self.as_ptr ( )     as  usize }
}
