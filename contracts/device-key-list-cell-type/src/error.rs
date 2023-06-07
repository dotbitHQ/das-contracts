use ckb_std::syscalls::SysError;
use das_types::VerificationError;

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(i8)]
pub(crate) enum ErrorCode {
    IndexOutOfBound = 1,
    ItemMissing,
    LengthNotEnough,
    Encoding,

    FoundKeyListInInput = 50,
    WitnessArgsInvalid,
    NoKeyListInOutput,
    LockArgLengthIncorrect,
    InvalidLock,
    KeyListParseError,
    InvalidTransactionStructure,
    KeyListNumberIncorrect,
    UpdateParamsInvalid,
    DestroyParamsInvalid,
    CapacityNotEnough,
    MustUseDasLock,
    InconsistentBalanceCellLocks,
    CapacityReduceTooMuch,
    ActionNotSupported,
    VerificationError
}

impl From<SysError> for ErrorCode {
    fn from(err: SysError) -> Self {
        use SysError::*;
        match err {
            IndexOutOfBound => Self::IndexOutOfBound,
            ItemMissing => Self::ItemMissing,
            LengthNotEnough(_) => Self::LengthNotEnough,
            Encoding => Self::Encoding,
            Unknown(err_code) => panic!("unexpected sys error {}", err_code),
        }
    }
}

impl Into<i8> for ErrorCode {
    fn into(self) -> i8 {
        self as i8
    }
}
