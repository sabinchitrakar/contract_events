
use contract_transcode::ContractMessageTranscoder;
use std::path::Path;
use subxt::Config;
use subxt::OnlineClient;
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
