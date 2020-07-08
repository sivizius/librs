use
{
  super::super::
  {
    types::CUInt,
    syscall::Argument,
  },
  bitflags::bitflags,
};

bitflags!
{
  /// File Permissions.
  #[allow(non_camel_case_types)]
  pub struct  Mode:                     CUInt
  {
    /// No bit is set.
    const None                          =   0b000_000_000_000;
    /// File can be executed by others.
    const OtherExecutable               =   0b000_000_000_001;
    /// File can be written to by others.
    const OtherWrite                    =   0b000_000_000_010;
    /// File can be read from by others.
    const OtherRead                     =   0b000_000_000_100;
    /// File can be executed by group.
    const GroupExecutable               =   0b000_000_001_000;
    /// File can be written to by group.
    const GroupWrite                    =   0b000_000_010_000;
    /// File can be read from by group.
    const GroupRead                     =   0b000_000_100_000;
    /// File can be executed by user.
    const UserExecutable                =   0b000_001_000_000;
    /// File can be written to by user.
    const UserWrite                     =   0b000_010_000_000;
    /// File can be read from by user.
    const UserRead                      =   0b000_100_000_000;
    /// Sticky Bit.
    const StickyBit                     =   0b001_000_000_000;
    /// Executable could gain the same privileges as the group of this file.
    const SetGID                        =   0b010_000_000_000;
    /// Executable could gain the same privileges as the user of this file.
    const SetUID                        =   0b100_000_000_000;
  }
}

impl          Argument                  for Mode
{
  fn  into  ( self  ) ->  usize { self.bits as  usize }
}
