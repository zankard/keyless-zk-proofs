// Copyright © Aptos Foundation

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct CircuitConfig {
    pub max_lengths: BTreeMap<String, usize>,
}
