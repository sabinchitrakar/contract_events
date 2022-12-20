mod contract_runtime;
use contract_transcode::ContractMessageTranscoder;
use contract_transcode::env_types::AccountId;
use std::path::Path;
use subxt::Config;
use subxt::OnlineClient;
use subxt::events::{Events,EventDetails,StaticEvent};
use crate::contract_runtime::api::contracts::events::ContractEmitted;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}


pub fn get_transcoder(path:&str)->ContractMessageTranscoder{
    let metadata_path = Path::new(path);
    let transcoder = ContractMessageTranscoder::load(metadata_path).unwrap();
    transcoder
}

pub struct ContractEventParser<T:Config> {
    transcoder:ContractMessageTranscoder,
    client:OnlineClient<T>
}

impl<T:Config> ContractEventParser<T>{
    pub fn new(metadata_path:&str,client:OnlineClient<T>)->Self{
        Self{
            client,
            transcoder:get_transcoder(metadata_path)
        }

    }

     pub async fn get_events_at(&self,block_number:u64)->Events<T>{
        let block_hash = self.client.rpc().block_hash(Some(block_number.into())).await.unwrap();
        let events = self.client.events().at(block_hash).await.unwrap();
        events

    }

    pub async fn get_contract_events(&self,contract_address:AccountId,block_number:u64){
        let events= self.get_events_at(block_number).await;
        for event_result in events.iter() {
            let event =event_result.unwrap();
            if <ContractEmitted as StaticEvent>::is_event(&event.pallet_name(), &event.variant_name()){
                 println!("found event {:?}",event);
            }
           
            
            
        }

    }
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
