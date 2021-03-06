use std::hash::{Hash, Hasher};

use rustc_serialize::hex::ToHex;
use twox_hash::XxHash;

use types::{H160, H256, U256};

/// A trait for all log-data-containing structures, but that are themselves
/// not (only) logs.  Used in several of Etherswap's private repositories.
pub trait LogLike {
    fn event_log(&self) -> &Log;

    fn event_address(&self) -> &H160 {
        &self.event_log().address
    }

    fn event_topics(&self) -> &Vec<H256> {
        &self.event_log().topics
    }

    fn raw_event_data(&self) -> &str {
        &self.event_log().data.as_str()
    }

    fn block_hash(&self) -> Option<&H256> {
        self.event_log().block_hash.as_ref()
    }

    fn block_hash_unchecked(&self) -> &H256 {
        self.event_log().block_hash.as_ref().unwrap()
    }

    fn block_number(&self) -> Option<&U256> {
        self.event_log().block_number.as_ref()
    }

    fn block_number_unchecked(&self) -> &U256 {
        self.event_log().block_number.as_ref().unwrap()
    }

    fn transaction_hash(&self) -> Option<&H256> {
        self.event_log().transaction_hash.as_ref()
    }

    fn transaction_hash_unchecked(&self) -> &H256 {
        self.event_log().transaction_hash.as_ref().unwrap()
    }

    fn transaction_index(&self) -> Option<&U256> {
        self.event_log().transaction_index.as_ref()
    }

    fn transaction_index_unchecked(&self) -> &U256 {
        self.event_log().transaction_index.as_ref().unwrap()
    }

    fn log_index(&self) -> Option<&U256> {
        self.event_log().log_index.as_ref()
    }

    fn log_index_unchecked(&self) -> &U256 {
        self.event_log().log_index.as_ref().unwrap()
    }

    fn transaction_log_index(&self) -> Option<&U256> {
        self.event_log().transaction_log_index.as_ref()
    }

    fn transaction_log_index_unchecked(&self) -> &U256 {
        self.event_log().transaction_log_index.as_ref().unwrap()
    }
}

/// A log produced after a transaction's execution.
#[derive(Debug, Hash, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    pub address: H160,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<H256>,
    pub data: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<H256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_number: Option<U256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_hash: Option<H256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_index: Option<U256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_index: Option<U256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_log_index: Option<U256>,
    #[serde(rename="type")]
    pub log_type: String
}

impl Log {
    pub fn to_hash(&self) -> u64 {
        let mut hasher = XxHash::default();
        self.hash(&mut hasher);
        hasher.finish()
    }

    pub fn hash_data(&self) -> u64 {
        let mut hasher = XxHash::default();
        let hash_string = format!(
            "{:?}{:?}{}", &self.address, &self.transaction_hash, &self.data
        );
        hash_string.hash(&mut hasher);
        hasher.finish()
    }
}


