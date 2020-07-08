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
  /// Flags for opening a file.
  #[allow(non_camel_case_types)]
  pub struct  Flags:                    CUInt
  {
    /// Open file in read-only mode.
    const RDONLY                        =   0;
    /// Open file in write-only mode.
    const WRONLY                        =   1;
    /// Open file in readable and writeable mode.
    const RDWR                          =   2;
    /// Mask for Access Mode Bits.
    const ACCMODE                       =   3;
    /// If `pathName` does not exist, create it as a regular file.
    const CREAT                         =   0b000_000_000_000_000_001_000_000;
    /// Ensure that this call creates the file:
    ///   if this flag is specified in conjunction with `Flags::CREAT`,
    ///   and `pathName` already exists,
    ///     then `open()` fails with the error `Failure::EEXIST`.
    const EXCL                          =   0b000_000_000_000_000_010_000_000;
    /// If `pathName` refers to a terminal device,
    ///   it will not become the process’ controlling terminal,
    ///     even if the process does not have one.
    const NOCTTY                        =   0b000_000_000_000_000_100_000_000;
    /// If the file already exists,
    ///   is a regular file and
    /// the access mode allows writing,
    ///   it will be truncated to length 0.
    /// If the file is a FIFO or terminal device file,
    ///   this flag is ignored.
    /// Otherwise,
    ///   the effect of this flag is unspecified.
    const TRUNC                         =   0b000_000_000_000_001_000_000_000;
    /// The file is opened in append mode.
    /// Before each `write()`,
    ///   the file offset is positioned at the end of the file,
    /// as if with `lseek()`.
    /// The modification of the file offset and
    ///   the write operation are performed as a single atomic step.
    const APPEND                        =   0b000_000_000_000_010_000_000_000;
    /// When possible, the file is opened in nonblocking mode.
    /// Neither the `open()`
    ///   nor any subsequent I/O operations on the file descriptor
    ///     which is returned
    /// will cause the calling process to wait.
    const NONBLOCK                      =   0b000_000_000_000_100_000_000_000;
    /// The same as `Flags::NONBLOCK`.
    const NONDELAY                      =   Self::NONBLOCK.bits;
    /// Write operations on the file
    ///   will complete according to the requirements of synchronized I/O data integrity completion.
    /// By the time `write()` and similar return,
    ///   the output data has been transferred to the underlying hardware,
    ///     along with any file metadata
    ///       that would be required to retrieve that data
    ///       (i.e., as though each `write()` was followed by a call to `fdatasync()`).
    const DSYNC                         =   0b000_000_000_001_000_000_000_000;
    /// Enable signal-driven I/O:
    ///   Generate a signal
    ///     (SIGIO by default, but this can be changed via `fcntl()`)
    ///   when input or output becomes possible on this file descriptor.
    /// This feature is available only for
    /// * terminals,
    /// * pseudoterminals,
    /// * sockets,
    /// * pipes (since Linux 2.6) and
    /// * FIFOs (since Linux 2.6).
    const ASYNC                         =   0b000_000_000_010_000_000_000_000;
    /// Try to minimize cache effects of the I/O to and from this file.
    /// In general this will degrade performance,
    ///   but it is useful in special situations,
    ///     such as when applications do their own caching.
    /// File I/O is done directly to/from user-space buffers.
    /// This flag on its own makes an effort to transfer data synchronously,
    ///   but does not give the guarantees of the `Flags::SYNC` flag that data and
    ///     necessary metadata are transferred.
    /// To guarantee synchronous I/O,
    ///   the `Flags::SYNC` flag must be used in addition to this flag.
    const DIRECT                        =   0b000_000_000_100_000_000_000_000;
    /// Allow files whose sizes cannot be represented in an `u32`,
    ///   but can be represented in an `u64` to be opened.
    const LARGEFILE                     =   0b000_000_001_000_000_000_000_000;
    /// If `pathName` is not a directory,
    ///   cause the `open()`` to fail.
    const DIRECTORY                     =   0b000_000_010_000_000_000_000_000;
    /// If the trailing component (i.e., basename) of pathname is a symbolic link,
    ///   then the `open()` fails with the error `Failure::ELOOP`.
    /// Symbolic links in earlier components of the pathname will still be followed.
    /// Note that the ELOOP error,
    ///   that can occur in this case,
    /// is indistinguishable from the case,
    ///   where an open fails because there are too many symbolic links found
    ///     while resolving components in the prefix part of the pathname.
    const NOFOLLOW                      =   0b000_000_100_000_000_000_000_000;
    /// Do not update the file last access time when the file is `read()`.
    /// This flag can be employed only if one of the following conditions is true:
    /// * The effective UID of the process matches the owner UID of the file.
    /// *  The calling process has the `CAP_FOWNER` capability in its user namespace and
    ///     the owner UID of the file has a mapping in the namespace.
    /// This flag is intended for use by indexing or backup programs,
    ///   where its use can significantly reduce the amount of disk activity.
    /// This flag may not be effective on all filesystems.
    /// One example is NFS, where the server maintains the access time.
    const NOATIME                       =   0b000_001_000_000_000_000_000_000;
    /// Enable the close-on-exec flag for the new file descriptor.
    /// Specifying this flag permits a program
    ///   to avoid additional `fcntl(F_SETFD)` operations to set the `FD_CLOEXEC` flag.
    /// Note that the use of this flag is essential in some multithreaded programs,
    ///   because using a separate `fcntl()` `F_SETFD` operation
    ///     to set the `FD_CLOEXEC` flag does not suffice to avoid race conditions
    ///       where one thread opens a file descriptor and
    ///       attempts to set its close-on-exec flag using `fcntl()` at the same time
    ///         as another thread does a `fork()` plus `execve()`.
    /// Depending on the order of execution,
    ///   the race may lead to the file descriptor returned
    ///     by `open()` being unintentionally leaked to the program executed
    ///     by the child process created by `fork()`.
    /// This kind of race is in principle possible for any system call
    ///   that creates a file descriptor whose close-on-exec flag should be set, and
    ///     various other Linux system calls provide an equivalent
    ///       of this flag to deal with this problem.
    const CLOEXEC                       =   0b000_010_000_000_000_000_000_000;
    /// Write operations on the file
    ///   will complete according to the requirements of synchronized I/O file integrity completion.
    /// By contrast with the synchronized I/O data integrity completion provided by `Flags::O_DSYNC`.
    /// By the time `write()` or similar returns,
    ///   the output data and associated file metadata have been transferred
    ///   to the underlying hardware
    ///     (i.e., as though each `write()` was followed by a call to `fsync()`).
    const SYNC                          =   0b000_100_000_001_000_000_000_000;
    /// Obtain a file descriptor that can be used for two purposes:
    ///   To indicate a location in the filesystem tree and
    ///   to perform operations that act purely at the file descriptor level.
    /// The file itself is not opened and other file operations
    ///   (e.g. `read()`, `write()`, `fchmod()`, `fchown()`, `fgetxattr()`, `ioctl()`, `mmap()`)
    /// fail with the error `Failure::EBADF`.
    const PATH                          =   0b001_000_000_001_000_000_000_000;
    /// Create an unnamed temporary regular file.
    /// The `pathName` argument specifies a directory;
    ///   an unnamed inode will be created in that directorys filesystem.
    /// Anything written to the resulting file will be lost
    ///   when the last file descriptor is closed,
    ///     unless the file is given a name.
    const TMPFILE                       =   0b010_000_000_001_000_000_000_000;
  }
}

impl          Argument                  for Flags
{
  fn  into  ( self  ) ->  usize { self.bits as  usize }
}
