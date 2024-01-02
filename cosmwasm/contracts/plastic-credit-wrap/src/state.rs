use cosmwasm_std::{Addr, Storage, Binary};
use cw_storage_plus::Item;
use serde::{Deserialize, Serialize};
use cw721_base::state::TokenInfo; // Use TokenInfo from cw721_base

// Static variable for the key to access the stored state
static STATE: Item<State> = Item::new("state");

use cw_storage_plus::Map;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Metadata {}

// Define a Map to store each TokenInfo, keyed by its token_id
pub const TOKENS: Map<&str, TokenInfo<Metadata>> = Map::new("tokens");


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct State {
    pub admin_address: Addr,
    pub public_key: Binary,
    pub token_count: u64,
}


// Function to save the state to the contract's storage
pub fn save_state(storage: &mut dyn Storage, state: &State) -> Result<(), cosmwasm_std::StdError> {
    STATE.save(storage, state)
}

// Function to load the state from the contract's storage
pub fn load_state(storage: &dyn Storage) -> Result<State, cosmwasm_std::StdError> {
    STATE.load(storage)
}

