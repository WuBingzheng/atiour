//! gRPC status.

/// gRPC status.
///
/// There is no constructor method. You need to set the 2 members
/// to construct yourself.
///
/// Examples:
///
/// ```rust,ignore
/// return Err(Status{
///     code: Code::NotFound,
///     message: format!("key {} is not found", &req.key),
/// });
/// ```
pub struct Status {
    pub code: Code,
    pub message: String,
}

/// gRPC status code.
///
/// See [gRPC status codes](https://github.com/grpc/grpc/blob/master/doc/statuscodes.md#status-codes-and-their-use-in-grpc).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Code {
    Ok = 0,
    Cancelled = 1,
    Unknown = 2,
    InvalidArgument = 3,
    DeadlineExceeded = 4,
    NotFound = 5,
    AlreadyExists = 6,
    PermissionDenied = 7,
    ResourceExhausted = 8,
    FailedPrecondition = 9,
    Aborted = 10,
    OutOfRange = 11,
    Unimplemented = 12,
    Internal = 13,
    Unavailable = 14,
    DataLoss = 15,
    Unauthenticated = 16,
}
