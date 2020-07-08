use
{
  const_type::Const,
  core::fmt,
};

Const!
{
  /// Different Ways a Syscall could fail.
  pub           Failure:                usize
  {
    /// Operation not permitted
    EPERM                               =   1,
    /// No such file or directory
    ENOENT                              =   2,
    /// No such process
    ESRCH                               =   3,
    /// Interrupted system call
    EINTR                               =   4,
    /// I/O error
    EIO                                 =   5,
    /// No such device or address
    ENXIO                               =   6,
    /// Argument list too long
    E2BIG                               =   7,
    /// Exec format error
    ENOEXEC                             =   8,
    /// Bad file number
    EBADF                               =   9,
    /// No child processes
    ECHILD                              =   10,
    /// Try again
    EAGAIN                              =   11,
    /// Out of memory
    ENOMEM                              =   12,
    /// Permission denied
    EACCES                              =   13,
    /// Bad address
    EFAULT                              =   14,
    /// Block device required
    ENOTBLK                             =   15,
    /// Device or resource busy
    EBUSY                               =   16,
    /// File exists
    EEXIST                              =   17,
    /// Cross-device link
    EXDEV                               =   18,
    /// No such device
    ENODEV                              =   19,
    /// Not a directory
    ENOTDIR                             =   20,
    /// Is a directory
    EISDIR                              =   21,
    /// Invalid argument
    EINVAL                              =   22,
    /// File table overflow
    ENFILE                              =   23,
    /// Too many open files
    EMFILE                              =   24,
    /// Not a typewriter
    ENOTTY                              =   25,
    /// Text file busy
    ETXTBSY                             =   26,
    /// File too large
    EFBIG                               =   27,
    /// No space left on device
    ENOSPC                              =   28,
    /// Illegal seek
    ESPIPE                              =   29,
    /// Read-only file system
    EROFS                               =   30,
    /// Too many links
    EMLINK                              =   31,
    /// Broken pipe
    EPIPE                               =   32,
    /// Math argument out of domain of func
    EDOM                                =   33,
    /// Math result not representable
    ERANGE                              =   34,
    /// Resource deadlock would occur
    EDEADLK                             =   35,
    /// File name too long
    ENAMETOOLONG                        =   36,
    /// No record locks available
    ENOLCK                              =   37,
    /// Function not implemented
    ENOSYS                              =   38,
    /// Directory not empty
    ENOTEMPTY                           =   39,
    /// Too many symbolic links encountered
    ELOOP                               =   40,
    /// Operation would block
    EWOULDBLOCK                         =   Self::EAGAIN.0,
    /// No message of desired type
    ENOMSG                              =   42,
    /// Identifier removed
    EIDRM                               =   43,
    /// Channel number out of range
    ECHRNG                              =   44,
    /// Level 2 not synchronized
    EL2NSYNC                            =   45,
    /// Level 3 halted
    EL3HLT                              =   46,
    /// Level 3 reset
    EL3RST                              =   47,
    /// Link number out of range
    ELNRNG                              =   48,
    /// Protocol driver not attached
    EUNATCH                             =   49,
    /// No CSI structure available
    ENOCSI                              =   50,
    /// Level 2 halted
    EL2HLT                              =   51,
    /// Invalid exchange
    EBADE                               =   52,
    /// Invalid request descriptor
    EBADR                               =   53,
    /// Exchange full
    EXFULL                              =   54,
    /// No anode
    ENOANO                              =   55,
    /// Invalid request code
    EBADRQC                             =   56,
    /// Invalid slot
    EBADSLT                             =   57,
    /// Resource deadlock would occur
    EDEADLOCK                           =   Self::EDEADLK.0,
    /// Bad font file format
    EBFONT                              =   59,
    /// Device not a stream
    ENOSTR                              =   60,
    /// No data available
    ENODATA                             =   61,
    /// Timer expired
    ETIME                               =   62,
    /// Out of streams resources
    ENOSR                               =   63,
    /// Machine is not on the network
    ENONET                              =   64,
    /// Package not installed
    ENOPKG                              =   65,
    /// Object is remote
    EREMOTE                             =   66,
    /// Link has been severed
    ENOLINK                             =   67,
    /// Advertise error
    EADV                                =   68,
    /// Srmount error
    ESRMNT                              =   69,
    /// Communication error on send
    ECOMM                               =   70,
    /// Protocol error
    EPROTO                              =   71,
    /// Multihop attempted
    EMULTIHOP                           =   72,
    /// RFS specific error
    EDOTDOT                             =   73,
    /// Not a data message
    EBADMSG                             =   74,
    /// Value too large for defined data type
    EOVERFLOW                           =   75,
    /// Name not unique on network
    ENOTUNIQ                            =   76,
    /// File descriptor in bad state
    EBADFD                              =   77,
    /// Remote address changed
    EREMCHG                             =   78,
    /// Can not access a needed shared library
    ELIBACC                             =   79,
    /// Accessing a corrupted shared library
    ELIBBAD                             =   80,
    /// .lib section in a.out corrupted
    ELIBSCN                             =   81,
    /// Attempting to link in too many shared libraries
    ELIBMAX                             =   82,
    /// Cannot exec a shared library directly
    ELIBEXEC                            =   83,
    /// Illegal byte sequence
    EILSEQ                              =   84,
    /// Interrupted system call should be restarted
    ERESTART                            =   85,
    /// Streams pipe error
    ESTRPIPE                            =   86,
    /// Too many users
    EUSERS                              =   87,
    /// Socket operation on non-socket
    ENOTSOCK                            =   88,
    /// Destination address required
    EDESTADDRREQ                        =   89,
    /// Message too long
    EMSGSIZE                            =   90,
    /// Protocol wrong type for socket
    EPROTOTYPE                          =   91,
    /// Protocol not available
    ENOPROTOOPT                         =   92,
    /// Protocol not supported
    EPROTONOSUPPORT                     =   93,
    /// Socket type not supported
    ESOCKTNOSUPPORT                     =   94,
    /// Operation not supported on transport endpoint
    EOPNOTSUPP                          =   95,
    /// Protocol family not supported
    EPFNOSUPPORT                        =   96,
    /// Address family not supported by protocol
    EAFNOSUPPORT                        =   97,
    /// Address already in use
    EADDRINUSE                          =   98,
    /// Cannot assign requested address
    EADDRNOTAVAIL                       =   99,
    /// Network is down
    ENETDOWN                            =   100,
    /// Network is unreachable
    ENETUNREACH                         =   101,
    /// Network dropped connection because of reset
    ENETRESET                           =   102,
    /// Software caused connection abort
    ECONNABORTED                        =   103,
    /// Connection reset by peer
    ECONNRESET                          =   104,
    /// No buffer space available
    ENOBUFS                             =   105,
    /// Transport endpoint is already connected
    EISCONN                             =   106,
    /// Transport endpoint is not connected
    ENOTCONN                            =   107,
    /// Cannot send after transport endpoint shutdown
    ESHUTDOWN                           =   108,
    /// Too many references: cannot splice
    ETOOMANYREFS                        =   109,
    /// Connection timed out
    ETIMEDOUT                           =   110,
    /// Connection refused
    ECONNREFUSED                        =   111,
    /// Host is down
    EHOSTDOWN                           =   112,
    /// No route to host
    EHOSTUNREACH                        =   113,
    /// Operation already in progress
    EALREADY                            =   114,
    /// Operation now in progress
    EINPROGRESS                         =   115,
    /// Stale NFS file handle
    ESTALE                              =   116,
    /// Structure needs cleaning
    EUCLEAN                             =   117,
    /// Not a XENIX named type file
    ENOTNAM                             =   118,
    /// No XENIX semaphores available
    ENAVAIL                             =   119,
    /// Is a named type file
    EISNAM                              =   120,
    /// Remote I/O error
    EREMOTEIO                           =   121,
    /// Quota exceeded
    EDQUOT                              =   122,
    /// No medium found
    ENOMEDIUM                           =   123,
    /// Wrong medium type
    EMEDIUMTYPE                         =   124,
    /// Operation Canceled
    ECANCELED                           =   125,
    /// Required key not available
    ENOKEY                              =   126,
    /// Key has expired
    EKEYEXPIRED                         =   127,
    /// Key has been revoked
    EKEYREVOKED                         =   128,
    /// Key was rejected by service
    EKEYREJECTED                        =   129,
    /// Owner died
    EOWNERDEAD                          =   130,
    /// State not recoverable
    ENOTRECOVERABLE                     =   131,
  }
}

impl          Failure
{
  /// Human readable Error Messages.
  const       MESSAGES:
  [ &'static str; Self::ENOTRECOVERABLE.0 - 1 ]
  =   [
        "Operation not permitted.",
        "No such file or directory.",
        "No such process.",
        "Interrupted system call.",
        "I/O error.",
        "No such device or address.",
        "Argument list too long.",
        "Exec format error.",
        "Bad file number.",
        "No child processes.",
        "Try again / Operation would block.",
        "Out of memory.",
        "Permission denied.",
        "Bad address.",
        "Block device required.",
        "Device or resource busy.",
        "File exists.",
        "Cross-device link.",
        "No such device.",
        "Not a directory.",
        "Is a directory.",
        "Invalid argument.",
        "File table overflow.",
        "Too many open files.",
        "Not a typewriter.",
        "Text file busy.",
        "File too large.",
        "No space left on device.",
        "Illegal seek.",
        "Read-only file system.",
        "Too many links.",
        "Broken pipe.",
        "Math argument out of domain of func.",
        "Math result not representable.",
        "Resource deadlock would occur.",
        "File name too long.",
        "No record locks available.",
        "Function not implemented.",
        "Directory not empty.",
        "Too many symbolic links encountered.",
        "Operation would block.",
        "No message of desired type.",
        "Identifier removed.",
        "Channel number out of range.",
        "Level 2 not synchronized.",
        "Level 3 halted.",
        "Level 3 reset.",
        "Link number out of range.",
        "Protocol driver not attached.",
        "No CSI structure available.",
        "Level 2 halted.",
        "Invalid exchange.",
        "Invalid request descriptor.",
        "Exchange full.",
        "No anode.",
        "Invalid request code.",
        "Invalid slot.",
        "Bad font file format.",
        "Device not a stream.",
        "No data available.",
        "Timer expired.",
        "Out of streams resources.",
        "Machine is not on the network.",
        "Package not installed.",
        "Object is remote.",
        "Link has been severed.",
        "Advertise error.",
        "Srmount error.",
        "Communication error on send.",
        "Protocol error.",
        "Multihop attempted.",
        "RFS specific error.",
        "Not a data message.",
        "Value too large for defined data type.",
        "Name not unique on network.",
        "File descriptor in bad state.",
        "Remote address changed.",
        "Can not access a needed shared library.",
        "Accessing a corrupted shared library.",
        ".lib section in a.out corrupted.",
        "Attempting to link in too many shared libraries.",
        "Cannot exec a shared library directly.",
        "Illegal byte sequence.",
        "Interrupted system call should be restarted.",
        "Streams pipe error.",
        "Too many users.",
        "Socket operation on non-socket.",
        "Destination address required.",
        "Message too long.",
        "Protocol wrong type for socket.",
        "Protocol not available.",
        "Protocol not supported.",
        "Socket type not supported.",
        "Operation not supported on transport endpoint.",
        "Protocol family not supported.",
        "Address family not supported by protocol.",
        "Address already in use.",
        "Cannot assign requested address.",
        "Network is down.",
        "Network is unreachable.",
        "Network dropped connection because of reset.",
        "Software caused connection abort.",
        "Connection reset by peer.",
        "No buffer space available.",
        "Transport endpoint is already connected.",
        "Transport endpoint is not connected.",
        "Cannot send after transport endpoint shutdown.",
        "Too many references: cannot splice.",
        "Connection timed out.",
        "Connection refused.",
        "Host is down.",
        "No route to host.",
        "Operation now in progress.",
        "Stale NFS file handle.",
        "Structure needs cleaning.",
        "Not a XENIX named type file.",
        "No XENIX semaphores available.",
        "Is a named type file.",
        "Operation already in progress.",
        "Remote I/O error.",
        "Quota exceeded.",
        "No medium found.",
        "Wrong medium type.",
        "Operation Canceled.",
        "Required key not available.",
        "Key has expired.",
        "Key has been revoked.",
        "Key was rejected by service.",
        "Owner died.",
        "State not recoverable.",
      ];

  #[inline(always)]
  pub ( super ) fn wrap
  (
    failure:                            isize,
  )
  ->  Failure
  {
    Failure ( -failure  as  usize )
  }
}

impl          fmt::Display              for Failure
{
  fn            fmt
  (
    &self,
    formatter:                          &mut fmt::Formatter,
  )
  ->  fmt::Result
  {
    formatter
    .write_str
    (
      if  self.0  - 1 < Self::MESSAGES.len  ( )
      {
        Self::MESSAGES  [ self.0  - 1 ]
      }
      else
      {
        "Unknown Error."
      }
    )
  }
}
