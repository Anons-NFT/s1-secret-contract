Snip-721 Protocal by Baedrik template with several edits

1. mint() caps the number of tokens to 580
2. mint() will keep count of how many anons each address has minted, if it has reached a max of 3, the address can no longer mint
3. transfer_impl() erases previous private metadata

4. Added whitelist storage key to state.rs
5. Saves whitelist in init msg
6. mint() checks whitelist and removes whitelisted minters when minting, and figures figures out if non whitelisted are able to mint
7. set_metadata() disables public metadata setting by commenting it out
8. mint() checks to see if minter has sent the correct amount of scrt
9. query_private_meta() checks first to see if admin bot is the viewer, before checking normal viewing permissions

10. query_possible_mints() gives user a number between 1 and 3 of how many nfts they can mint



11. For preloading token storage, struct PreLoad was added to state.rs
12. msg.rs added preload array to init. contract.rs saves preload array

13. Memo in mint() is where the user will place the tg handle


Randomizaton
Added rng to mint()
Added ChaChaRng to contract.rs
rand to cargo.toml