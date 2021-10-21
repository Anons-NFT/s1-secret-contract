#[cfg(test)]
mod tests {
    use crate::contract::{handle, init};
    use crate::msg::{HandleMsg, InitMsg};
    use crate::state::{PreLoad};
    use crate::royalties::{Royalty,RoyaltyInfo};
    use cosmwasm_std::testing::*;
    use cosmwasm_std::{ Extern, HumanAddr, InitResponse, StdError, StdResult};
    use std::any::Any;

    // Helper functions

    fn init_helper() -> (
        StdResult<InitResponse>,
        Extern<MockStorage, MockApi, MockQuerier>,
    ) {
        let mut deps = mock_dependencies(45, &[]);
        let env = mock_env("instantiator", &[]);
        let mut royalties = Vec::new();


        let royalty1 = Royalty {
            recipient:HumanAddr("secret1tv0wkl89th5eshh4cprrwwrncvprp0a7jdtr5n".to_string()),
            rate:25
        };
        let royalty2 = Royalty {
            recipient:HumanAddr("secret1hcakhdfaqfd9ja0h8vy5vxe3le0sfgaj7m9zpw".to_string()),
            rate:25
        };
        let royalty3 = Royalty {
            recipient:HumanAddr("secret17qvcesavppsmylnxdk8ae7ktl2lsfjnwlv09kv".to_string()),
            rate:2
        };
        let royalty4 = Royalty {
            recipient:HumanAddr("secret1vftdu34y7rq99xyd4xfj8xp5xs93mr643t40sy".to_string()),
            rate:25
        };

        royalties.push(royalty1);
        royalties.push(royalty2);
        royalties.push(royalty3);
        royalties.push(royalty4);

        let mut preload_tokens = Vec::new();
        let token = PreLoad{
            id:"1".to_string(),
            img_url:"https://i.picsum.photos/id/55/540/540.jpg?hmac=5zACDOmrhtVoeccua9y6b7U5cnYtfliOtLOQHvAPXx8".to_string(),
        };
        let token2 = PreLoad{
            id:"2".to_string(),
            img_url:"https://i.picsum.photos/id/719/540/540.jpg?hmac=NidNUaY7x2l143xSpoPBXZA7SHZoWfT8ILqpilrvfzg".to_string(),
        };
        preload_tokens.push(token);
        preload_tokens.push(token2);
        //Minters
        let mut minters = Vec::new();
        minters.push(HumanAddr("secret1ntdya7huz8n4gsh3zlpapytqdqk0s5x8kru4nw".to_string()));
        minters.push(HumanAddr("secret17qvcesavppsmylnxdk8ae7ktl2lsfjnwlv09kv".to_string()));
        minters.push(HumanAddr("secret1tv0wkl89th5eshh4cprrwwrncvprp0a7jdtr5n".to_string()));

        let init_msg = InitMsg {
            name: "Anons".to_string(),
            symbol: "ANON".to_string(),
            admin: None,
            entropy: "entropytest".to_string(),
            preload_tokens:preload_tokens,
            whitelist_minters: minters,
            royalty_info: Some(RoyaltyInfo {
                decimal_places_in_rates:2,
                royalties: royalties,
            }),
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
                _ => panic!("Unexpected error result {:?}", err),
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
        let handle_msg = HandleMsg::Mint {
            owner: Some(HumanAddr("secret1ntdya7huz8n4gsh3zlpapytqdqk0s5x8kru4nw".to_string())),
            memo: Some("luiseel".to_string()),
            padding:Some("sodhjenygr945aws439uu94fkgdmnlmsf409".to_string())
        };
        let handle_result = handle(&mut deps, mock_env("secret1s7c6xp9wltthk5r6mmavql4xld5me3g37guhsx", &[]), handle_msg);

        // let error = extract_error_msg(handle_result);
        print!("hola {:?} \n",handle_result.ok().unwrap());
        // assert!(handle_result.is_ok());
        // assert!(error.contains("Only designated minters are allowed to mint"));
        // print!("error here\n {}",error);
    }
}
