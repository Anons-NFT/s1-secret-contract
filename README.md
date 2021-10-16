**Snip-721 Protocal by Baedrik template with several edits**

**Minting Limits**

1. `mint()` caps tokens max at 580

2. `mint()` will keep count of how many anons each address has minted, if it has reached a max of 3, the address can no longer mint new tokens

3. `mint()` requires user to send sSCRT which is deivided amongst several wallets


**Whitelist Minters**

1. `init()` accepts an array of addresses and saves it to reserve 1 Anon for each address on it

2. Whitelist expires after 3 days from contract initialization

4. `mint()` checks whitelist and removes whitelisted minters after they make their reserved mint, and checks if there are enough tokens left for the non whitelisted to mint


**Telegram Integration**

1. `mint()` memo field is used to indlude your telgram handle. It will be placed in private metadata

2. `transfer_impl()` erases previous private metadata when token moves from one address to another

3. `set_metadata()` disables public metadata setting but leaves private metadata editable so you can update your TG handle

4. `query_private_meta()` checks first to see if admin, used for the telegram whitelist bot, is the viewer, before checking normal viewing permissions


**Query Possible Mints**

1. `query_possible_mints()` gives user a number between 0 and 3 of how many nfts they can mint


**Preloading Tokens and Randomization**

1. `init()` takes a vec of `PreLoad` containing the Arweave urls and the ids for each token and saves them

2. `mint()` uses the contract entropy to randomly select data from the preload list, applies it to the metadata, then removes it from the list


**Grand Reveal**

1. `init()` declares a bool flag, `reveal`, as `false`

2. While reveal is `false`, `query_nft_info()` and `query_all_nft_info()` will fail, however `query_private_metadata()` will function normally, so the telegram bot can still see who to whitelist


3. `reveal_all_tokens()` sets `reveal` to `true` and allows all public metadata to be queriable. This is only accessible by the admin