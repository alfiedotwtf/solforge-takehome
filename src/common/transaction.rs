use serde::{Deserialize, Serialize};
use solana_sdk::transaction::TransactionError;
use solana_transaction_status::{EncodedTransaction, EncodedTransactionWithStatusMeta, UiMessage};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransactionErrors {
    None,
    AccountBorrowOutstanding,
    AccountInUse,
    AccountLoadedTwice,
    AccountNotFound,
    AddressLookupTableNotFound,
    AlreadyProcessed,
    BlockhashNotFound,
    CallChainTooDeep,
    ClusterMaintenance,
    DuplicateInstruction,
    InstructionError,
    InsufficientFundsForFee,
    InsufficientFundsForRent,
    InvalidAccountForFee,
    InvalidAccountIndex,
    InvalidAddressLookupTableData,
    InvalidAddressLookupTableIndex,
    InvalidAddressLookupTableOwner,
    InvalidLoadedAccountDataSizeLimit,
    InvalidProgramForExecution,
    InvalidRentPayingAccount,
    InvalidWritableAccount,
    MaxLoadedAccountsDataSizeExceeded,
    MissingSignatureForFee,
    ProgramAccountNotFound,
    ProgramCacheHitMaxLimit,
    ProgramExecutionTemporarilyRetsricted,
    ResanitizationNeeded,
    SanitizeFailure,
    SignatureFailure,
    TooManyAccountLocks,
    UnbalancedTransaction,
    UnsupportedVersion,
    WoulbExceedMaxAccountCostLimit,
    WouldExceedAccountDataBlockLimit,
    WouldExceedAccountDataTotalLimit,
    WouldExceedMaxBlockCostLimit,
    WouldExceedMaxVoteCostLimit,
}

impl From<TransactionError> for TransactionErrors {
    fn from(error: TransactionError) -> Self {
        match error {
            TransactionError::AccountBorrowOutstanding => {
                TransactionErrors::AccountBorrowOutstanding
            }
            TransactionError::AccountInUse => TransactionErrors::AccountInUse,
            TransactionError::AccountLoadedTwice => TransactionErrors::AccountLoadedTwice,
            TransactionError::AccountNotFound => TransactionErrors::AccountNotFound,
            TransactionError::AddressLookupTableNotFound => {
                TransactionErrors::AddressLookupTableNotFound
            }
            TransactionError::AlreadyProcessed => TransactionErrors::AlreadyProcessed,
            TransactionError::BlockhashNotFound => TransactionErrors::BlockhashNotFound,
            TransactionError::CallChainTooDeep => TransactionErrors::CallChainTooDeep,
            TransactionError::ClusterMaintenance => TransactionErrors::ClusterMaintenance,
            TransactionError::DuplicateInstruction(_) => TransactionErrors::DuplicateInstruction,
            TransactionError::InstructionError(_, _) => TransactionErrors::InstructionError,
            TransactionError::InsufficientFundsForFee => TransactionErrors::InsufficientFundsForFee,
            TransactionError::InsufficientFundsForRent { .. } => {
                TransactionErrors::InsufficientFundsForRent
            }
            TransactionError::InvalidAccountForFee => TransactionErrors::InvalidAccountForFee,
            TransactionError::InvalidAccountIndex => TransactionErrors::InvalidAccountIndex,
            TransactionError::InvalidAddressLookupTableData => {
                TransactionErrors::InvalidAddressLookupTableData
            }
            TransactionError::InvalidAddressLookupTableIndex => {
                TransactionErrors::InvalidAddressLookupTableIndex
            }
            TransactionError::InvalidAddressLookupTableOwner => {
                TransactionErrors::InvalidAddressLookupTableOwner
            }
            TransactionError::InvalidLoadedAccountsDataSizeLimit => {
                TransactionErrors::InvalidLoadedAccountDataSizeLimit
            }
            TransactionError::InvalidProgramForExecution => {
                TransactionErrors::InvalidProgramForExecution
            }
            TransactionError::InvalidRentPayingAccount => {
                TransactionErrors::InvalidRentPayingAccount
            }
            TransactionError::InvalidWritableAccount => TransactionErrors::InvalidWritableAccount,
            TransactionError::MaxLoadedAccountsDataSizeExceeded => {
                TransactionErrors::MaxLoadedAccountsDataSizeExceeded
            }
            TransactionError::MissingSignatureForFee => TransactionErrors::MissingSignatureForFee,
            TransactionError::ProgramAccountNotFound => TransactionErrors::ProgramAccountNotFound,
            TransactionError::ProgramCacheHitMaxLimit => TransactionErrors::ProgramCacheHitMaxLimit,
            TransactionError::ProgramExecutionTemporarilyRestricted { .. } => {
                TransactionErrors::ProgramExecutionTemporarilyRetsricted
            }
            TransactionError::ResanitizationNeeded => TransactionErrors::ResanitizationNeeded,
            TransactionError::SanitizeFailure => TransactionErrors::SanitizeFailure,
            TransactionError::SignatureFailure => TransactionErrors::SignatureFailure,
            TransactionError::TooManyAccountLocks => TransactionErrors::TooManyAccountLocks,
            TransactionError::UnbalancedTransaction => TransactionErrors::UnbalancedTransaction,
            TransactionError::UnsupportedVersion => TransactionErrors::UnsupportedVersion,
            TransactionError::WouldExceedMaxAccountCostLimit => {
                TransactionErrors::WoulbExceedMaxAccountCostLimit
            }
            TransactionError::WouldExceedAccountDataBlockLimit => {
                TransactionErrors::WouldExceedAccountDataBlockLimit
            }
            TransactionError::WouldExceedMaxBlockCostLimit => {
                TransactionErrors::WouldExceedMaxBlockCostLimit
            }
            TransactionError::WouldExceedMaxVoteCostLimit => {
                TransactionErrors::WouldExceedMaxVoteCostLimit
            }
            TransactionError::WouldExceedAccountDataTotalLimit => {
                TransactionErrors::WouldExceedAccountDataTotalLimit
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub err: Option<TransactionErrors>,
    pub recent_blockhash: String,
    pub signatures: Vec<String>,
    pub accounts: Vec<String>,
    pub balances: Vec<u64>,
    pub fees: u64,
}

impl From<EncodedTransactionWithStatusMeta> for Transaction {
    fn from(transaction: EncodedTransactionWithStatusMeta) -> Self {
        let meta = transaction.meta.expect("Error decoding transaction meta");

        let transaction = match transaction.transaction {
            EncodedTransaction::Json(transaction) => transaction,
            _ => panic!("Error decoding transaction"),
        };

        let message = match transaction.message {
            UiMessage::Raw(message) => message,
            _ => panic!("Error decoding transaction message"),
        };

        Transaction {
            id: transaction.signatures[0].to_string(),
            err: meta.err.map(|e| e.into()),
            recent_blockhash: message.recent_blockhash,
            signatures: transaction
                .signatures
                .iter()
                .map(|s| s.to_string())
                .collect(),
            accounts: message.account_keys,
            balances: meta.post_balances,
            fees: meta.fee,
        }
    }
}

impl From<&EncodedTransactionWithStatusMeta> for Transaction {
    fn from(transaction: &EncodedTransactionWithStatusMeta) -> Self {
        (transaction.clone()).into()
    }
}
