mod error;
mod execute;
mod msg;
mod query;
mod state;

use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use crate::error::ContractError;
use crate::msg::InstantiateMsg;
use crate::state::{save_state, State};
use cw721_base::InstantiateMsg as Cw721InstantiateMsg;// Rename to avoid conflict
use ::cw721_base::entry;

pub fn my_instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Set up your custom state here
    let state = State {
        public_key: msg.public_key,
        admin_address: msg.admin_address,
        token_count: 0,
    };
    save_state(deps.storage, &state)?;

    // Prepare CW721 base instantiation message
    let cw721_msg = Cw721InstantiateMsg {
        name: msg.name,
        symbol: msg.symbol,
        minter: info.sender.to_string(), // Use the sender as the minter
    };

    // Call the CW721 base instantiate function
    entry::instantiate(deps, env, info, cw721_msg)?;

    Ok(Response::new().add_attribute("method", "my_instantiate"))
}
