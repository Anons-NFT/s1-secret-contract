#[cfg(test)]
mod tests {
    use crate::contract::{handle, init};
    use crate::msg::{HandleMsg, InitMsg};
    use cosmwasm_std::testing::*;
    use cosmwasm_std::{ Extern, HumanAddr, InitResponse, StdError, StdResult};
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
            preload_tokens:Vec::new(),
            whitelist_minters:Vec::new(),
            royalty_info: None,
            config: None,
            callback:"526843da19e865fc93130b3178cef8ac82c3956e72263017290b63c10cdaf595".to_string(),
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
            owner: Some(HumanAddr("secret1ntdya7huz8n4gsh3zlpapytqdqk0s5x8kru4nw".to_string())),
            memo: Some("luiseel".to_string()),
            padding:Some("sodhjenygr945aws439uu94fkgdmnlmsf409".to_string())
        };
        let handle_result = handle(&mut deps, mock_env("alice", &[]), handle_msg);
        let error = extract_error_msg(handle_result);
        assert!(error.contains("Only designated minters are allowed to mint"));
    }
}
