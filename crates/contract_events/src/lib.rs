mod contract_runtime;
use crate::contract_runtime::api::contracts::events::ContractEmitted;
use contract_transcode::ContractMessageTranscoder;
use contract_transcode::Value;
use std::collections::HashMap;
use std::path::Path;
use subxt::events::{Events, StaticEvent};
use subxt::ext::sp_runtime::AccountId32;
use subxt::Config;
use subxt::OnlineClient;
use hex_literal::hex;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn get_transcoder(path: &str) -> ContractMessageTranscoder {
    let metadata_path = Path::new(path);
    let transcoder = ContractMessageTranscoder::load(metadata_path).unwrap();
    transcoder
}

pub struct ContractEventParser<T: Config> {
    transcoders_map: HashMap<AccountId32, ContractMessageTranscoder>,
    client: OnlineClient<T>,
}

pub struct ContractInfo<'a> {
    pub address: AccountId32,
    pub metadata_path: &'a str,
}

pub struct ContractEvent {
    pub name: String,
    pub value: serde_json::Value,
    pub address: AccountId32
}

impl<T: Config> ContractEventParser<T> {
    pub fn new(contract_infos: Vec<ContractInfo>, client: OnlineClient<T>) -> Self {
        let mut transcoders_map: HashMap<AccountId32, ContractMessageTranscoder> = HashMap::new();
        for contract_info in contract_infos.into_iter() {
            let transcoder = get_transcoder(contract_info.metadata_path);
            transcoders_map.insert(contract_info.address, transcoder);
        }
        Self {
            client,
            transcoders_map,
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

    pub async fn get_contract_events_at(&self, block_number: u64) -> Vec<ContractEvent> {
        let events = self.get_events_at(block_number).await;
        let mut json_events: Vec<ContractEvent> = Vec::new();
        for event_result in events.iter() {
            let event = event_result.unwrap();
            if <ContractEmitted as StaticEvent>::is_event(
                &event.pallet_name(),
                &event.variant_name(),
            ) {
                let contract_emitted_event = event.as_event::<ContractEmitted>().unwrap().unwrap();
                println!("found event {:?}", &event.variant_name());
                println!("parsed event {:?}", &contract_emitted_event);
                if self
                    .transcoders_map
                    .contains_key(&contract_emitted_event.contract)
                {
                    let transcoder = self
                        .transcoders_map
                        .get(&contract_emitted_event.contract)
                        .unwrap();

                    let contract_event = transcoder
                        .decode_contract_event(&mut event.bytes())
                        .unwrap();

                    let event_name = get_contract_event_name(&contract_event);
                    let json_value = to_json_value(contract_event);
                    println!("decoded event {:?}", &json_value);
                    json_events.push(ContractEvent {
                        name: event_name,
                        value: json_value,
                        address:contract_emitted_event.contract,
                    })
                }
            }
        }
        return json_events;
    }
}

pub fn get_contract_event_name(val: &Value) -> String {
    match val {
        Value::Map(m) => String::from(m.ident().unwrap_or(String::from("Unnamed"))),
        _ => String::from("Unnamed"),
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
        let contract_parser = ContractEventParser::new(
            vec![ContractInfo {
                address: AccountId32::from(hex!("f2d85bfd0f776b82feb093c01a2e71cc24570d5fb5f09759b5ed09abab76d574")),
                metadata_path: "../../contracts/staking_rewards/metadata.json",
            }],
            api,
        );
        contract_parser.get_contract_events_at(547926).await;
    }
}
