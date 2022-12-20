
use contract_transcode::ContractMessageTranscoder;
use std::path::Path;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}


pub fn get_transcoder(path:&str)->ContractMessageTranscoder{
    let metadata_path = Path::new(path);
    let transcoder = ContractMessageTranscoder::load(metadata_path).unwrap();
    transcoder
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
