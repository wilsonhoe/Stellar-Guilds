pub mod management;
pub mod multisig;
pub mod storage;
pub mod types;

pub use management::{
    approve_transaction, deposit, emergency_pause, execute_milestone_payment, execute_transaction,
    get_balance, get_transaction_history, grant_allowance, initialize_treasury, propose_withdrawal,
    set_budget,
};

#[allow(unused_imports)]
pub use storage::initialize_treasury_storage;

#[allow(unused_imports)]
pub use types::{Allowance, Budget, Transaction, TransactionStatus, TransactionType, Treasury};
// Tests disabled pending fixes
#[cfg(test)]
mod tests;
