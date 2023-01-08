// This is the action module.
use base64::{decode, encode};
pub fn bencode(data: String) -> String {
    encode(data)
}
pub fn bdecode(data: String) -> String {
    let unparsed = &decode(data).unwrap()[..];
    let mut rslt: String = String::new();
    for i in unparsed {
        rslt.push(*i as char);
    }
    rslt
}
