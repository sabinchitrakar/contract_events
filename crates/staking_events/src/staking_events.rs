use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSuccessful {
    staker: String,
    lock_box: LockBox,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RedeemSuccessful {
    staker: String,
    lock_box: LockBox,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WithdrawSuccessful {
    staker: String,
    lock_box: LockBox,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LockBox {
    id: u128,
    created_at: u64,
    deposit: u128,
    interest: u64,
    release: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StakingEvents {
    WithdrawSuccessful(WithdrawSuccessful),
    DepositSuccessful(DepositSuccessful),
    RedeemSuccessful(RedeemSuccessful),
}
