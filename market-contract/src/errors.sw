library;

pub enum AssetError {
    InvalidAsset: (),
}

pub enum ValueError {
    InvalidAmount: (),
    InvalidLength: (),
}

pub enum OrderError {
    OrderNotFound: (b256),
    DuplicateOrder: (),
    PriceCannotBeZero: (),
    AmountCannotBeZero: (),
    FailedToRemove: (),
}

pub enum MatchError {
    CantMatch: (b256, b256),
    CantBatchMatch: (),
}

pub enum AuthError {
    Unauthorized: (),
}

pub enum AccountError {
    InsufficientBalance: (u64, u64),
}
