use librs::
{
  unsafe_call,
  linux::
  {
    io,
    syscall::Syscall,
  },
};

/// Test Compilation
#[test]
fn            test_build  ( )
{
  let     message                       =   "Hello World\n";
  println!
  (
    "\nmessage: *{:?}",
    message as *const _,
  );
  println!("\n1:");
  let     result1
  =   unsafe
      {
        Syscall::Write
        .call3
        (
          1usize,
          message,
          message.len ( ),
        )
      };
  println!("\n2:");
  let     result2
  =   Syscall::write
      (
        io::STDOUT,
        message,
        message.len(),
      );
  println!("\n3:");
  let     result3
  =   unsafe
      {
        Syscall::Write
        .call3
        (
          3usize,
          message,
          message.len ( ),
        )
      };
  println!("\ndone\n");
  panic!
  (
    "1: {:?}, 2: {:?}, 3: {:?}",
    result1.map_err ( | error | format! ( "{}", error ) ),
    result2.map_err ( | error | format! ( "{}", error ) ),
    result3.map_err ( | error | format! ( "{}", error ) ),
  );
}
