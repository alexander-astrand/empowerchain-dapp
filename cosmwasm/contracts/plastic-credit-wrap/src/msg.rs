use cosmwasm_std::{Binary, Addr};
use serde::{Deserialize, Serialize};
use cosmwasm_std::Uint128;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {
    pub public_key: Binary,
    pub admin_address: Addr,
    // cw-721 specific fields
    pub name: String, // Name of the NFT collection
    pub symbol: String, // Symbol of the NFT collection
    pub minter: Addr, // Account that is allowed to mint tokens
}

pub struct TransferPlasticCreditsMsg {
    amount: Uint128,
    recipient: Addr,
}


