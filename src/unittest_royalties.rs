#[cfg(test)]
mod tests {
    use crate::contract::{handle, init, query};
    use crate::msg::{
        ContractStatus, HandleMsg, InitConfig, InitMsg, PostInitCallback, QueryAnswer, QueryMsg,
    };
    use crate::royalties::{Royalty, RoyaltyInfo};
    use crate::state::{load, Config, CONFIG_KEY};
    use cosmwasm_std::testing::*;
    use cosmwasm_std::{
        from_binary, to_binary, Api, Binary, Coin, CosmosMsg, Extern, HumanAddr, InitResponse,
        StdError, StdResult, Uint128, WasmMsg,
    };
    use std::any::Any;

    // Helper functions

    fn init_helper_royalties(
        royalty_info: Option<RoyaltyInfo>,
    ) -> (
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
            royalty_info,
            config: None,
            post_init_callback: None,
        };

        (init(&mut deps, env, init_msg), deps)
    }

    fn init_helper_royalties_with_config(
        royalty_info: Option<RoyaltyInfo>,
        public_token_supply: bool,
        public_owner: bool,
        enable_sealed_metadata: bool,
        unwrapped_metadata_is_private: bool,
        minter_may_update_metadata: bool,
        owner_may_update_metadata: bool,
        enable_burn: bool,
    ) -> (
        StdResult<InitResponse>,
        Extern<MockStorage, MockApi, MockQuerier>,
    ) {
        let mut deps = mock_dependencies(20, &[]);

        let env = mock_env("instantiator", &[]);
        let init_config: InitConfig = from_binary(&Binary::from(
            format!(
                "{{\"public_token_supply\":{},
            \"public_owner\":{},
            \"enable_sealed_metadata\":{},
            \"unwrapped_metadata_is_private\":{},
            \"minter_may_update_metadata\":{},
            \"owner_may_update_metadata\":{},
            \"enable_burn\":{}}}",
                public_token_supply,
                public_owner,
                enable_sealed_metadata,
                unwrapped_metadata_is_private,
                minter_may_update_metadata,
                owner_may_update_metadata,
                enable_burn
            )
            .as_bytes(),
        ))
        .unwrap();
        let init_msg = InitMsg {
            name: "sec721".to_string(),
            symbol: "S721".to_string(),
            admin: Some(HumanAddr("admin".to_string())),
            entropy: "We're going to need a bigger boat".to_string(),
            royalty_info,
            config: Some(init_config),
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

    // Init tests

    #[test]
    fn test_init_sanity() {
        let expected = RoyaltyInfo {
            decimal_places_in_rates: 2,
            royalties: vec![
                Royalty {
                    recipient: HumanAddr("alice".to_string()),
                    rate: 10,
                },
                Royalty {
                    recipient: HumanAddr("bob".to_string()),
                    rate: 5,
                },
            ],
        };

        // test default config
        let (init_result, deps) = init_helper_royalties(Some(expected.clone()));
        assert_eq!(init_result.unwrap(), InitResponse::default());
        let config: Config = load(&deps.storage, CONFIG_KEY).unwrap();
        assert_eq!(config.status, ContractStatus::Normal.to_u8());
        assert_eq!(config.mint_cnt, 0);
        assert_eq!(config.tx_cnt, 0);
        assert_eq!(config.name, "sec721".to_string());
        assert_eq!(
            config.admin,
            deps.api
                .canonical_address(&HumanAddr("admin".to_string()))
                .unwrap()
        );
        assert_eq!(config.symbol, "S721".to_string());
        assert_eq!(config.token_supply_is_public, false);
        assert_eq!(config.owner_is_public, false);
        assert_eq!(config.sealed_metadata_is_enabled, false);
        assert_eq!(config.unwrap_to_private, false);
        assert_eq!(config.minter_may_update_metadata, true);
        assert_eq!(config.owner_may_update_metadata, false);
        assert_eq!(config.burn_is_enabled, false);

        let query_msg = QueryMsg::RoyaltyInfo { token_id: None };
        let query_result = query(&deps, query_msg);
        assert!(
            query_result.is_ok(),
            "query failed: {}",
            query_result.err().unwrap()
        );
        let query_answer: QueryAnswer = from_binary(&query_result.unwrap()).unwrap();
        match query_answer {
            QueryAnswer::RoyaltyInfo { royalty_info } => {
                assert_eq!(royalty_info, Some(expected.clone()));
            }
            _ => panic!("unexpected"),
        }

        // test config specification
        let (init_result, deps) = init_helper_royalties_with_config(
            Some(expected.clone()),
            true,
            true,
            true,
            true,
            false,
            true,
            false,
        );
        assert_eq!(init_result.unwrap(), InitResponse::default());
        let config: Config = load(&deps.storage, CONFIG_KEY).unwrap();
        assert_eq!(config.status, ContractStatus::Normal.to_u8());
        assert_eq!(config.mint_cnt, 0);
        assert_eq!(config.tx_cnt, 0);
        assert_eq!(config.name, "sec721".to_string());
        assert_eq!(
            config.admin,
            deps.api
                .canonical_address(&HumanAddr("admin".to_string()))
                .unwrap()
        );
        assert_eq!(config.symbol, "S721".to_string());
        assert_eq!(config.token_supply_is_public, true);
        assert_eq!(config.owner_is_public, true);
        assert_eq!(config.sealed_metadata_is_enabled, true);
        assert_eq!(config.unwrap_to_private, true);
        assert_eq!(config.minter_may_update_metadata, false);
        assert_eq!(config.owner_may_update_metadata, true);
        assert_eq!(config.burn_is_enabled, false);

        let query_msg = QueryMsg::RoyaltyInfo { token_id: None };
        let query_result = query(&deps, query_msg);
        assert!(
            query_result.is_ok(),
            "query failed: {}",
            query_result.err().unwrap()
        );
        let query_answer: QueryAnswer = from_binary(&query_result.unwrap()).unwrap();
        match query_answer {
            QueryAnswer::RoyaltyInfo { royalty_info } => {
                assert_eq!(royalty_info, Some(expected));
            }
            _ => panic!("unexpected"),
        }

        // test post init callback
        let mut deps = mock_dependencies(20, &[]);
        let env = mock_env("instantiator", &[]);
        // just picking a random short HandleMsg that wouldn't really make sense
        let post_init_msg = to_binary(&HandleMsg::MakeOwnershipPrivate { padding: None }).unwrap();
        let post_init_send = vec![Coin {
            amount: Uint128(100),
            denom: "uscrt".to_string(),
        }];
        let post_init_callback = Some(PostInitCallback {
            msg: post_init_msg.clone(),
            contract_address: HumanAddr("spawner".to_string()),
            code_hash: "spawner hash".to_string(),
            send: post_init_send.clone(),
        });

        let init_msg = InitMsg {
            name: "sec721".to_string(),
            symbol: "S721".to_string(),
            admin: Some(HumanAddr("admin".to_string())),
            entropy: "We're going to need a bigger boat".to_string(),
            royalty_info: None,
            config: None,
            post_init_callback,
        };

        let init_response = init(&mut deps, env, init_msg).unwrap();
        assert_eq!(
            init_response.messages,
            vec![CosmosMsg::Wasm(WasmMsg::Execute {
                msg: post_init_msg,
                contract_addr: HumanAddr("spawner".to_string()),
                callback_code_hash: "spawner hash".to_string(),
                send: post_init_send,
            })]
        );
    }

    // test setting royalty info
    #[test]
    fn test_set_royalty_info() {
        let (init_result, mut deps) = init_helper_royalties(None);
        assert!(
            init_result.is_ok(),
            "Init failed: {}",
            init_result.err().unwrap()
        );

        let expected = RoyaltyInfo {
            decimal_places_in_rates: 2,
            royalties: vec![
                Royalty {
                    recipient: HumanAddr("steven".to_string()),
                    rate: 10,
                },
                Royalty {
                    recipient: HumanAddr("thomas".to_string()),
                    rate: 5,
                },
            ],
        };

        // test non-minter attempting to set default royalties
        let handle_msg = HandleMsg::SetRoyaltyInfo {
            token_id: None,
            royalty_info: Some(expected.clone()),
            padding: None,
        };
        let handle_result = handle(&mut deps, mock_env("alice", &[]), handle_msg);
        let error = extract_error_msg(handle_result);
        assert!(
            error.contains("Only designated minters can set default royalties for the contract")
        );

        // test royalties more than 100%
        let royalty_info_for_failure = RoyaltyInfo {
            decimal_places_in_rates: 2,
            royalties: vec![
                Royalty {
                    recipient: HumanAddr("steven".to_string()),
                    rate: 80,
                },
                Royalty {
                    recipient: HumanAddr("thomas".to_string()),
                    rate: 20,
                },
                Royalty {
                    recipient: HumanAddr("uriel".to_string()),
                    rate: 1,
                },
            ],
        };
        let handle_msg = HandleMsg::SetRoyaltyInfo {
            token_id: None,
            royalty_info: Some(royalty_info_for_failure.clone()),
            padding: None,
        };
        let handle_result = handle(&mut deps, mock_env("admin", &[]), handle_msg);
        let error = extract_error_msg(handle_result);
        assert!(error.contains("The sum of royalty rates must not exceed 100%"));

        // verify no default royalties are set
        let query_msg = QueryMsg::RoyaltyInfo { token_id: None };
        let query_result = query(&deps, query_msg);
        assert!(
            query_result.is_ok(),
            "query failed: {}",
            query_result.err().unwrap()
        );
        let query_answer: QueryAnswer = from_binary(&query_result.unwrap()).unwrap();
        match query_answer {
            QueryAnswer::RoyaltyInfo { royalty_info } => {
                assert_eq!(royalty_info, None);
            }
            _ => panic!("unexpected"),
        }

        // set default royalties sanity check
        let handle_msg = HandleMsg::SetRoyaltyInfo {
            token_id: None,
            royalty_info: Some(expected.clone()),
            padding: None,
        };
        let handle_result = handle(&mut deps, mock_env("admin", &[]), handle_msg);
        assert!(handle_result.is_ok());
        let query_msg = QueryMsg::RoyaltyInfo { token_id: None };
        let query_result = query(&deps, query_msg);
        assert!(
            query_result.is_ok(),
            "query failed: {}",
            query_result.err().unwrap()
        );
        let query_answer: QueryAnswer = from_binary(&query_result.unwrap()).unwrap();
        match query_answer {
            QueryAnswer::RoyaltyInfo { royalty_info } => {
                assert_eq!(royalty_info, Some(expected.clone()));
            }
            _ => panic!("unexpected"),
        }

        // test unknown token error during query when supply is private
        let query_msg = QueryMsg::RoyaltyInfo {
            token_id: Some("NFT".to_string()),
        };
        let query_result = query(&deps, query_msg);
        assert!(
            query_result.is_ok(),
            "query failed: {}",
            query_result.err().unwrap()
        );
        let query_answer: QueryAnswer = from_binary(&query_result.unwrap()).unwrap();
        match query_answer {
            QueryAnswer::RoyaltyInfo { royalty_info } => {
                assert_eq!(royalty_info, Some(expected.clone()));
            }
            _ => panic!("unexpected"),
        }

        // verify default gets deleted
        let handle_msg = HandleMsg::SetRoyaltyInfo {
            token_id: None,
            royalty_info: None,
            padding: None,
        };
        let handle_result = handle(&mut deps, mock_env("admin", &[]), handle_msg);
        assert!(handle_result.is_ok());
        let query_msg = QueryMsg::RoyaltyInfo { token_id: None };
        let query_result = query(&deps, query_msg);
        assert!(
            query_result.is_ok(),
            "query failed: {}",
            query_result.err().unwrap()
        );
        let query_answer: QueryAnswer = from_binary(&query_result.unwrap()).unwrap();
        match query_answer {
            QueryAnswer::RoyaltyInfo { royalty_info } => {
                assert_eq!(royalty_info, None);
            }
            _ => panic!("unexpected"),
        }

        // test unknown token id error when supply is private
        let handle_msg = HandleMsg::SetRoyaltyInfo {
            token_id: Some("NFT".to_string()),
            royalty_info: Some(expected.clone()),
            padding: None,
        };
        let handle_result = handle(&mut deps, mock_env("admin", &[]), handle_msg);
        let error = extract_error_msg(handle_result);
        assert!(error.contains("A token's RoyaltyInfo may only be set by the token creator when they are also the token owner"));

        let default = RoyaltyInfo {
            decimal_places_in_rates: 2,
            royalties: vec![Royalty {
                recipient: HumanAddr("default".to_string()),
                rate: 10,
            }],
        };
        let individual = RoyaltyInfo {
            decimal_places_in_rates: 3,
            royalties: vec![Royalty {
                recipient: HumanAddr("individual".to_string()),
                rate: 10,
            }],
        };
        let handle_msg = HandleMsg::SetRoyaltyInfo {
            token_id: None,
            royalty_info: Some(default.clone()),
            padding: None,
        };
        let handle_result = handle(&mut deps, mock_env("admin", &[]), handle_msg);
        assert!(handle_result.is_ok());

        let handle_msg = HandleMsg::MintNft {
            token_id: Some("default".to_string()),
            owner: Some(HumanAddr("alice".to_string())),
            public_metadata: None,
            private_metadata: None,
            royalty_info: None,
            serial_number: None,
            memo: None,
            padding: None,
        };
        let handle_result = handle(&mut deps, mock_env("admin", &[]), handle_msg);
        assert!(handle_result.is_ok());

        // verify it has the default royalties
        let query_msg = QueryMsg::RoyaltyInfo {
            token_id: Some("default".to_string()),
        };
        let query_result = query(&deps, query_msg);
        let query_answer: QueryAnswer = from_binary(&query_result.unwrap()).unwrap();
        match query_answer {
            QueryAnswer::RoyaltyInfo { royalty_info } => {
                assert_eq!(royalty_info, Some(default.clone()));
            }
            _ => panic!("unexpected"),
        }

        let handle_msg = HandleMsg::MintNft {
            token_id: Some("specified".to_string()),
            owner: None,
            public_metadata: None,
            private_metadata: None,
            royalty_info: Some(individual.clone()),
            serial_number: None,
            memo: None,
            padding: None,
        };
        let handle_result = handle(&mut deps, mock_env("admin", &[]), handle_msg);
        assert!(handle_result.is_ok());
        // verify it has the individual royalties
        let query_msg = QueryMsg::RoyaltyInfo {
            token_id: Some("specified".to_string()),
        };
        let query_result = query(&deps, query_msg);
        let query_answer: QueryAnswer = from_binary(&query_result.unwrap()).unwrap();
        match query_answer {
            QueryAnswer::RoyaltyInfo { royalty_info } => {
                assert_eq!(royalty_info, Some(individual.clone()));
            }
            _ => panic!("unexpected"),
        }

        let query_msg = QueryMsg::RoyaltyInfo { token_id: None };
        let query_result = query(&deps, query_msg);
        let query_answer: QueryAnswer = from_binary(&query_result.unwrap()).unwrap();
        match query_answer {
            QueryAnswer::RoyaltyInfo { royalty_info } => {
                assert_eq!(royalty_info, Some(default.clone()));
            }
            _ => panic!("unexpected"),
        }

        // test setting royalties for a token if not owner
        let handle_msg = HandleMsg::SetRoyaltyInfo {
            token_id: Some("default".to_string()),
            royalty_info: Some(individual.clone()),
            padding: None,
        };
        let handle_result = handle(&mut deps, mock_env("admin", &[]), handle_msg);
        let error = extract_error_msg(handle_result);
        assert!(error.contains("A token's RoyaltyInfo may only be set by the token creator when they are also the token owner"));

        // test trying to set royalties for a token if not the creator
        let handle_msg = HandleMsg::SetRoyaltyInfo {
            token_id: Some("default".to_string()),
            royalty_info: Some(individual.clone()),
            padding: None,
        };
        let handle_result = handle(&mut deps, mock_env("alice", &[]), handle_msg);
        let error = extract_error_msg(handle_result);
        assert!(error.contains("A token's RoyaltyInfo may only be set by the token creator when they are also the token owner"));

        // test that deleting individual royalties updates the token to use the default
        let handle_msg = HandleMsg::SetRoyaltyInfo {
            token_id: Some("specified".to_string()),
            royalty_info: None,
            padding: None,
        };
        let handle_result = handle(&mut deps, mock_env("admin", &[]), handle_msg);
        assert!(handle_result.is_ok());
        let query_msg = QueryMsg::RoyaltyInfo {
            token_id: Some("specified".to_string()),
        };
        let query_result = query(&deps, query_msg);
        let query_answer: QueryAnswer = from_binary(&query_result.unwrap()).unwrap();
        match query_answer {
            QueryAnswer::RoyaltyInfo { royalty_info } => {
                assert_eq!(royalty_info, Some(default.clone()));
            }
            _ => panic!("unexpected"),
        }

        // delete the default royalties
        let handle_msg = HandleMsg::SetRoyaltyInfo {
            token_id: None,
            royalty_info: None,
            padding: None,
        };
        let handle_result = handle(&mut deps, mock_env("admin", &[]), handle_msg);
        assert!(handle_result.is_ok());

        // test that deleting a token's royalties when there is no default, results in no royalties
        let handle_msg = HandleMsg::SetRoyaltyInfo {
            token_id: Some("specified".to_string()),
            royalty_info: None,
            padding: None,
        };
        let handle_result = handle(&mut deps, mock_env("admin", &[]), handle_msg);
        assert!(handle_result.is_ok());
        let query_msg = QueryMsg::RoyaltyInfo {
            token_id: Some("specified".to_string()),
        };
        let query_result = query(&deps, query_msg);
        let query_answer: QueryAnswer = from_binary(&query_result.unwrap()).unwrap();
        match query_answer {
            QueryAnswer::RoyaltyInfo { royalty_info, .. } => {
                assert_eq!(royalty_info, None);
            }
            _ => panic!("unexpected"),
        }

        // test unknown token id when supply is public
        let (init_result, mut deps) = init_helper_royalties_with_config(
            Some(expected.clone()),
            true,
            true,
            true,
            true,
            false,
            true,
            false,
        );
        assert_eq!(init_result.unwrap(), InitResponse::default());
        // test unknown token id error when supply is private
        let handle_msg = HandleMsg::SetRoyaltyInfo {
            token_id: Some("NFT".to_string()),
            royalty_info: Some(expected),
            padding: None,
        };
        let handle_result = handle(&mut deps, mock_env("admin", &[]), handle_msg);
        let error = extract_error_msg(handle_result);
        assert!(error.contains("Token ID: NFT not found"));

        let query_msg = QueryMsg::RoyaltyInfo {
            token_id: Some("NFT".to_string()),
        };
        let query_result = query(&deps, query_msg);
        let error = extract_error_msg(query_result);
        assert!(error.contains("Token ID: NFT not found"));
    }
}
