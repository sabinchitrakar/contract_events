mod contract_runtime;
use crate::contract_runtime::api::contracts::events::ContractEmitted;
use contract_transcode::ContractMessageTranscoder;
use contract_transcode::Value;
use std::path::Path;
use subxt::events::{Events, StaticEvent};
use subxt::ext::sp_runtime::AccountId32;
use subxt::Config;
use subxt::OnlineClient;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn get_transcoder(path: &str) -> ContractMessageTranscoder {
    let metadata_path = Path::new(path);
    let transcoder = ContractMessageTranscoder::load(metadata_path).unwrap();
    transcoder
}

pub struct ContractEventParser<T: Config> {
    transcoder: ContractMessageTranscoder,
    client: OnlineClient<T>,
}

impl<T: Config> ContractEventParser<T> {
    pub fn new(metadata_path: &str, client: OnlineClient<T>) -> Self {
        Self {
            client,
            transcoder: get_transcoder(metadata_path),
        }
    }

    pub async fn get_events_at(&self, block_number: u64) -> Events<T> {
        let block_hash = self
            .client
            .rpc()
            .block_hash(Some(block_number.into()))
            .await
            .unwrap();
        let events = self.client.events().at(block_hash).await.unwrap();
        events
    }

    pub async fn get_contract_events(
        &self,
        contract_address: AccountId32,
        block_number: u64,
    ) -> Vec<serde_json::Value> {
        let events = self.get_events_at(block_number).await;
        let mut json_events: Vec<serde_json::Value> = Vec::new();
        for event_result in events.iter() {
            let event = event_result.unwrap();
            if <ContractEmitted as StaticEvent>::is_event(
                &event.pallet_name(),
                &event.variant_name(),
            ) {
                let contract_emitted_event = event.as_event::<ContractEmitted>().unwrap().unwrap();
                if contract_emitted_event.contract.eq(&contract_address) {}
                println!("found event {:?}", &event.variant_name());
                println!("parsed event {:?}", &contract_emitted_event);
                let contract_event = self
                    .transcoder
                    .decode_contract_event(&mut event.bytes())
                    .unwrap();
                let json_value = to_json_value(contract_event);
                println!("decoded event {:?}", &json_value);
                json_events.push(json_value)
            }
        }
        return json_events;
    }
}

pub fn to_json_value(val: Value) -> serde_json::Value {
    match val {
        Value::Bool(b) => serde_json::Value::Bool(b),
        Value::Char(ch) => serde_json::Value::String(ch.to_string()),
        Value::Hex(hx) => serde_json::Value::String(String::from(hx.as_str())),
        Value::Int(i) => serde_json::Value::Number((i as i64).into()),
        Value::Literal(l) => serde_json::Value::String(l),
        Value::Seq(s) => {
            return s
                .elems()
                .into_iter()
                .map(|v| to_json_value(v.clone()))
                .collect::<serde_json::Value>()
        }
        Value::String(st) => serde_json::Value::String(st),
        Value::Tuple(t) => {
            return t
                .values()
                .into_iter()
                .map(|v| to_json_value(v.clone()))
                .collect::<serde_json::Value>()
        }
        Value::UInt(u) => serde_json::Value::String(u.to_string()),
        Value::Map(map) => {
            let mut new_map = serde_json::map::Map::new();
            if let Some(id) = map.ident() {
                let mut child_map = serde_json::map::Map::new();
                for (key, val) in map.iter() {
                    child_map.insert(key.to_string(), to_json_value(val.clone()));
                }
                new_map.insert(id, serde_json::Value::Object(child_map));
            } else {
                for (key, val) in map.iter() {
                    new_map.insert(key.to_string(), to_json_value(val.clone()));
                }
            }
            serde_json::Value::Object(new_map)
        }
        Value::Unit => serde_json::Value::Null,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use subxt::PolkadotConfig;
    #[subxt::subxt(runtime_metadata_path = "../../artifacts/snow.scale")]
    pub mod snow {}

    #[tokio::test]
    async fn it_works() {
        let api: OnlineClient<PolkadotConfig> =
            OnlineClient::from_url("wss://snow-rpc-1.icenetwork.io:443")
                .await
                .unwrap();
        let contract_parser =
            ContractEventParser::new("../../contracts/staking_rewards/metadata.json", api);
        contract_parser
            .get_contract_events(AccountId32::from([0u8; 32]), 547926)
            .await;
    }
}
