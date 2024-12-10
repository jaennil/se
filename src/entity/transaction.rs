use chrono::NaiveDate;

use super::{account::Account, category::Category, payee::Payee};

#[derive(Default, Debug)]
pub struct Transaction {
    pub date: NaiveDate,
    pub status: Status,
    pub typ: Type,
    pub amount: String,
    pub account: Account,
    pub payee: Payee,
    pub category: Category,
}

#[derive(Default, Debug, PartialEq, Clone, strum::AsRefStr, strum::EnumIter)]
pub enum Status {
    #[default]
    Unreconciled,
    Reconciled,
    Void,
    FollowUp,
    Duplicate,
}

#[derive(Default, Debug, PartialEq, Clone, strum::EnumIter, strum::AsRefStr)]
pub enum Type {
    #[default]
    Withdrawal,
    Deposit
}
