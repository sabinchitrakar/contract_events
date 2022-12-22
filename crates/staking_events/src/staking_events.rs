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
    id: String,
    created_at: String,
    deposit: String,
    interest: String,
    release: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StakingEvents {
    WithdrawSuccessful(WithdrawSuccessful),
    DepositSuccessful(DepositSuccessful),
    RedeemSuccessful(RedeemSuccessful),
}

