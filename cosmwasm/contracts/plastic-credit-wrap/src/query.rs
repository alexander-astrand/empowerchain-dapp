use cosmwasm_std::{Deps, StdResult, Binary, to_json_binary};
use cosmwasm_std::Env;
use crate::state::TOKENS;
use cw721_base::QueryMsg;


// Helper function to get token info
pub fn query_token_info(deps: Deps, token_id: String) -> StdResult<Binary> {
    let token = TOKENS.load(deps.storage, &token_id)?;
    to_json_binary(&token)
}

pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg<()>,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Minter {} => {
            // Return the minter address
            to_json_binary(&"minter")
        },
        QueryMsg::OwnerOf { token_id, include_expired: _ } => {
            let token_info = TOKENS.load(deps.storage, &token_id)?;
            to_json_binary(&token_info.owner)
        },
        QueryMsg::Approval { spender: _, token_id: _, include_expired: _ } => {
            // Return the approval for the spender
            to_json_binary(&"approval")
        },
        QueryMsg::Approvals { token_id: _, include_expired: _ } => {
            // Return all approvals for the owner
            to_json_binary(&"approvals")
        },
        QueryMsg::Operator { operator: _, owner: _, include_expired: _ } => {
            // Return the operator for the token
            to_json_binary(&"operator")
        },
        QueryMsg::AllOperators { owner: _, include_expired: _, start_after: _, limit: _ } => {
            // Return all operators for the token
            to_json_binary(&"all_operators")
        },
        QueryMsg::NumTokens {} => {
            // Return the number of tokens
            to_json_binary(&"num_tokens")
        },
        QueryMsg::ContractInfo {} => {
            // Return the contract info
            to_json_binary(&"contract_info")
        },
        QueryMsg::NftInfo { token_id } => {
            let nft_info = TOKENS.load(deps.storage, &token_id)?;
            to_json_binary(&nft_info)
        },
        QueryMsg::AllNftInfo { token_id: _, include_expired: _ } => {
            // Return all nft info
            to_json_binary(&"all_nft_info")
        },
        QueryMsg::Tokens { owner: _, start_after: _, limit: _ } => {
            // Return all tokens for the owner
            to_json_binary(&"tokens")
        },
        QueryMsg::AllTokens { start_after: _, limit: _ } => {
            // Return all tokens
            to_json_binary(&"all_tokens")
        },
        QueryMsg::Extension { msg: _ } => {
            // Return the extension
            to_json_binary(&"extension")
        },
        QueryMsg::Ownership {} => {
            // Return the ownership
            to_json_binary(&"ownership")
        },
    }
}
