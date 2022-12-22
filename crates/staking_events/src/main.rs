mod db;
mod staking_events;
use bonsaidb::{
    core::schema::{Collection, SerializedCollection},
    local::{
        config::{Builder, StorageConfiguration},
        Database,
    },
};
use contract_events::{self, ContractEvent, ContractEventParser, ContractInfo};
use hex_literal::hex;
use serde::{Deserialize, Serialize};
use staking_events::{StakingEvents, WithdrawSuccessful};
use subxt::PolkadotConfig;
use subxt::{ext::sp_runtime::AccountId32, OnlineClient};
#[derive(Debug, Serialize, Deserialize, Collection)]
#[collection(name = "event_docs")]
pub struct EventDoc {
    name: String,
    address: String,
    block_number: u64,
    event: StakingEvents,
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let conn = db::get_connection::<EventDoc>("./data/events".to_owned()).unwrap();
    let api: OnlineClient<PolkadotConfig> =
        OnlineClient::from_url("wss://snow-rpc-1.icenetwork.io:443")
            .await
            .unwrap();

    let contract_parser = ContractEventParser::new(
        vec![ContractInfo {
            address: AccountId32::from(hex!(
                "f2d85bfd0f776b82feb093c01a2e71cc24570d5fb5f09759b5ed09abab76d574"
            )),
            metadata_path: "contracts/staking_rewards/metadata.json",
        }],
        api,
    );

    let events = contract_parser
        .get_contract_events_at(547926)
        .await
        .unwrap();
    let staking_events = events
        .into_iter()
        .map(|e| to_staking_event(e))
        .collect::<Vec<Option<StakingEvents>>>();
    println!("staking {:?}", staking_events);
}

pub fn to_staking_event(event: ContractEvent) -> Option<StakingEvents> {
    match event.name.as_str() {
        "WithdrawSuccessful" => {
            let withdraw: WithdrawSuccessful = serde_json::from_str(&event.value["WithdrawSuccessful"].to_string()).unwrap();
            Some(StakingEvents::WithdrawSuccessful(withdraw))
        }
        _ => None,
    }
}
