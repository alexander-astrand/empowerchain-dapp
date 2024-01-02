use crate::error::ContractError;
use crate::state::{TOKENS, load_state, save_state};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cosmwasm_std::Addr;
use cw721_base::state::TokenInfo;
use crate::state::Metadata;
use cw721_base::ExecuteMsg;

pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg<(), ()>,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint { token_uri, owner, token_id: _, extension: _ } => {
            
            let mut state = load_state(deps.storage)?;
            let token_id = state.token_count.to_string(); // Generate token_id from token_count
            state.token_count += 1; // Increment the token count for the next mint
            save_state(deps.storage, &state)?; // Save the updated state


            println!("Token count: {:?}", state.token_count);

            let token_info = TokenInfo {
                owner: Addr::unchecked(owner),
                approvals: vec![],
                token_uri: token_uri, // Assuming token_uri is of type String
                extension: Metadata {}, // Metadata struct, if needed
            };
            println!("Token id: {:?}", token_id);

            TOKENS.save(deps.storage, &token_id, &token_info)?; // Save with token_id as key

            match TOKENS.load(deps.storage, &token_id) {
                Ok(nft) => println!("Minted NFT: {:?}", nft),
                Err(err) => println!("Error after minting: {:?}", err),
            }

            Ok(Response::new()
                .add_attribute("action", "mint")
                .add_attribute("minter", info.sender.to_string())
                .add_attribute("token_id", &token_id))
        },
        ExecuteMsg::TransferNft { recipient, token_id } => {
            let mut token_info = TOKENS.load(deps.storage, &token_id)?;
            if info.sender != token_info.owner {
                return Err(ContractError::Unauthorized {});
            }
            println!("owner before transfer {:?}, recipient {:?}", token_info.owner, recipient);
            token_info.owner = Addr::unchecked(recipient.clone());
            TOKENS.save(deps.storage, &token_id, &token_info)?;
            println!("owner after transfer {:?}", token_info.owner);

            Ok(Response::new()
                .add_attribute("action", "transfer_nft")
                .add_attribute("sender", info.sender.to_string())
                .add_attribute("recipient", recipient.clone())
                .add_attribute("token_id", token_id))
        },
        
        ExecuteMsg::Burn { token_id } => {
            let token_info = TOKENS.load(deps.storage, &token_id)?;
            if info.sender != token_info.owner {
                return Err(ContractError::Unauthorized {});
            }
        
            TOKENS.remove(deps.storage, &token_id);
        
            Ok(Response::new()
                .add_attribute("action", "burn")
                .add_attribute("burner", info.sender.to_string())
                .add_attribute("token_id", token_id))
        },
        
        ExecuteMsg::Approve { spender, token_id, expires: _ } => {
            // Approve logic
            Ok(Response::new()
                .add_attribute("action", "approve")
                .add_attribute("owner", info.sender.to_string())
                .add_attribute("spender", spender.to_string())
                .add_attribute("token_id", token_id))
        },
        ExecuteMsg::Revoke { spender, token_id } => {
            // Revoke logic
            Ok(Response::new()
                .add_attribute("action", "revoke")
                .add_attribute("owner", info.sender.to_string())
                .add_attribute("spender", spender.to_string())
                .add_attribute("token_id", token_id))
        },
        ExecuteMsg::ApproveAll { operator, expires: _ } => {
            // Approve all logic
            Ok(Response::new()
                .add_attribute("action", "approve_all")
                .add_attribute("owner", info.sender.to_string())
                .add_attribute("operator", operator.to_string()))
        },
        ExecuteMsg::RevokeAll { operator } => {
            // Revoke all logic
            Ok(Response::new()
                .add_attribute("action", "revoke_all")
                .add_attribute("owner", info.sender.to_string())
                .add_attribute("operator", operator.to_string()))
        },
        ExecuteMsg::Extension { msg: _ } => {
            // Extension logic
            Ok(Response::new()
                .add_attribute("action", "extension"))
        },
        ExecuteMsg::UpdateOwnership ( _token_id ) => {
            // Update ownership logic
            Ok(Response::new()
                .add_attribute("action", "update_ownership"))
        },
        ExecuteMsg::SendNft { contract: _, token_id: _, msg: _ } => {
            // Send NFT logic
            Ok(Response::new()
                .add_attribute("action", "send_nft"))
        },
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{from_binary, Addr, Binary, StdError};
    use cw721_base::QueryMsg;
    use cosmwasm_std::testing::MockQuerier;
    use cosmwasm_std::testing::MockApi;
    use cosmwasm_std::MemoryStorage;
    use cosmwasm_std::OwnedDeps;
    use crate::msg::InstantiateMsg;
    use crate::execute;
    use crate::my_instantiate;
    use cw721_base::entry::query;

   
    fn setup_contract() -> (OwnedDeps<MemoryStorage, MockApi, MockQuerier>, Env, String) {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            public_key: Binary::from(b"public_key"),
            admin_address: Addr::unchecked("admin"),
            name: "My NFT Collection".to_string(),
            symbol: "MYNFT".to_string(),
            minter: Addr::unchecked("minter"),
        };
        let info = mock_info("creator", &[]);
        let env = mock_env();
        let _res = my_instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        
        let token_id = token_count.to_string();
        // Mint the first NFT
        let mint_msg1 = ExecuteMsg::Mint {
            token_uri: Some("http://example.com/token_uri_1".to_string()),
            owner: "owner_address".to_string(),
            token_id: "0".to_string(),
            extension: (),
        };
        let _mint_res1 = execute(deps.as_mut(), env.clone(), info.clone(), mint_msg1).unwrap();

        // Mint the second NFT
        let mint_msg2 = ExecuteMsg::Mint {
            token_uri: Some("http://example.com/token_uri_2".to_string()),
            owner: "owner_address".to_string(),
            token_id: "1".to_string(),
            extension: (),
        };
        let _mint_res2 = execute(deps.as_mut(), env.clone(), info, mint_msg2).unwrap();

        (deps, env, "owner_address".to_string())
    }

    #[test]
    fn test_minting_nft() {
        let (deps, env, owner) = setup_contract();
    
        // Use the same token_id as used in setup_contract for minting
        let token_id = "0".to_string();
        println!("Token id: {:?}", token_id);
    
        // Query to check if the NFT is stored
        let query_result = query(deps.as_ref(), env.clone(), QueryMsg::NftInfo { token_id: token_id.clone() });

        match query_result {
            Ok(_) => println!("NFT stored successfully"),
            Err(ref e) => println!("Failed to store NFT: {:?}", e),
        }
        println!("Query result: {:?}", query_result);
    
        // Assert that the query is successful
        assert!(query_result.is_ok(), "Failed to query minted NFT");
    }
    #[test]
    fn test_minting_second_nft() {
        let (deps, env, _owner) = setup_contract();

        // Query to check if the second NFT (with token_id_1) is stored
        let query_result = query(deps.as_ref(), env.clone(), QueryMsg::NftInfo { token_id: "1".to_string() });

        match query_result {
            Ok(_) => println!("Second NFT stored successfully"),
            Err(ref e) => println!("Failed to store second NFT: {:?}", e),
        }

        assert!(query_result.is_ok(), "Failed to query second minted NFT (token_id_1)");
    }
    #[test]
    fn test_burn_function() {
        let (mut deps, env, owner) = setup_contract();

        // Use the same token_id as used in setup_contract for burning
        let token_id = "1".to_string();
        println!("Token id: {:?}", token_id);

        // Burn the NFT
        let burn_msg = ExecuteMsg::Burn {
            token_id: token_id.clone(),
        };
        let burn_res = execute(deps.as_mut(), env.clone(), mock_info(owner.as_str(), &[]), burn_msg).unwrap();

        println!("Burn response: {:?}", burn_res);

        // Query to check if the NFT is burned
        let query_result = query(deps.as_ref(), env.clone(), QueryMsg::NftInfo { token_id: token_id.clone() });

        match query_result {
            Ok(_) => println!("NFT found after burn"),
            Err(ref e) => println!("NFT not found after burn: {:?}", e),
        }

        assert!(query_result.is_err(), "Failed to query burned NFT");
    }
    #[test]
    fn test_transfer_nft_function() {
        let (mut deps, env, owner) = setup_contract();

        // Use the same token_id as used in setup_contract for transfer
        let token_id = "0".to_string();
        println!("Token id: {:?}", token_id);

        // Transfer the NFT
        let transfer_msg = ExecuteMsg::TransferNft {
            recipient: "recipient_address".to_string(),
            token_id: token_id.clone(),
        };
        let transfer_res = execute(deps.as_mut(), env.clone(), mock_info(owner.as_str(), &[]), transfer_msg).unwrap();

        println!("Transfer response: {:?}", transfer_res);

        // Query to check if the NFT is transferred
        let query_result = query(deps.as_ref(), env.clone(), QueryMsg::NftInfo { token_id: token_id.clone() });

        match query_result {
            Ok(_) => println!("NFT transferred successfully"),
            Err(ref e) => println!("Failed to transfer NFT: {:?}", e),
        }

        assert!(query_result.is_ok(), "Failed to query transferred NFT");
    }
    
    // Add more test functions for other ExecuteMsg variants
}
