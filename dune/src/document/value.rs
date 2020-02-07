#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Signed(i64),
    Unsigned(u64),
    Float(f64),
    String(String),
    Bytes(Vec<u8>),
    // DateTime,
}
