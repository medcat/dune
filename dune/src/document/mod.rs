mod error;
mod multi;
mod value;
pub use self::error::{DocumentError, DocumentErrorKind};
pub use self::multi::MultiValue;
pub use self::value::Value;
use failure::ResultExt;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    #[serde(flatten)]
    pub values: HashMap<String, MultiValue>,
}

impl Document {
    pub fn compress(&self) -> Result<Vec<u8>, DocumentError> {
        let mut bottom = Vec::with_capacity(bincode::serialized_size(&self).unwrap_or(0) as usize);
        let snap = snap::Writer::new(std::io::Cursor::new(&mut bottom));
        bincode::serialize_into(snap, self).with_context(|_| DocumentErrorKind::SerializeError)?;
        bottom.shrink_to_fit();
        Ok(bottom)
    }
}
