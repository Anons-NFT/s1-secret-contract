#[cfg(test)]
mod tests {
    use crate::contract::{handle, init, query};
    use crate::mint_run::MintRunInfo;
    use crate::msg::{HandleMsg, InitMsg, QueryAnswer, QueryMsg};
    use cosmwasm_std::testing::*;
    use cosmwasm_std::{from_binary, Extern, HumanAddr, InitResponse, StdError, StdResult};
    use std::any::Any;

    // Helper functions

    fn init_helper() -> (
        StdResult<InitResponse>,
        Extern<MockStorage, MockApi, MockQuerier>,
    ) {
        let mut deps = mock_dependencies(20, &[]);
        let env = mock_env("instantiator", &[]);

        let init_msg = InitMsg {
            name: "sec721".to_string(),
            symbol: "S721".to_string(),
            admin: Some(HumanAddr("admin".to_string())),
            entropy: "We're going to need a bigger boat".to_string(),
            royalty_info: None,
            config: None,
            post_init_callback: None,
        };

        (init(&mut deps, env, init_msg), deps)
    }

    fn extract_error_msg<T: Any>(error: StdResult<T>) -> String {
        match error {
            Ok(_response) => panic!("Expected error, but had Ok response"),
            Err(err) => match err {
                StdError::GenericErr { msg, .. } => msg,
                _ => panic!(format!("Unexpected error result {:?}", err)),
            },
        }
    }

    // test mint run info
    #[test]
    fn test_mint_run_info() {
        let (init_result, mut deps) = init_helper();
        assert!(
            init_result.is_ok(),
            "Init failed: {}",
            init_result.err().unwrap()
        );

        // test non-minter attempt
        let handle_msg = HandleMsg::MintNft {
            owner: Some("secret1ntdya7huz8n4gsh3zlpapytqdqk0s5x8kru4nw".to_string()),
            memo: Some("luiseel".to_string())
        };
        let handle_result = handle(&mut deps, mock_env("alice", &[]), handle_msg);
        let error = extract_error_msg(handle_result);
        assert!(error.contains("Only designated minters are allowed to mint"));
    }
}
