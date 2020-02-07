mod storage;

use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Index(Arc<InnerIndex>);

#[derive(Debug, Clone)]
pub(crate) struct InnerIndex {
    storage: self::storage::Storage,
}
