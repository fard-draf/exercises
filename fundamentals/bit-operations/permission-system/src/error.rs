use thiserror::Error;

#[derive(Debug, Error)]
pub enum PermError {
    #[error("Invalid bit pattern")]
    InvalidBitPattern,

    #[error("Insufficient permissions")]
    InsufficientPermissions,

    #[error("Conflicting permissions")]
    ConflictingPermissions,

    #[error("Unknown permission")]
    UnknownPermission,
}
