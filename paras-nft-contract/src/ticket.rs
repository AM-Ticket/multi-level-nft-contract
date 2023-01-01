use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::{assert_one_yocto, env};
use serde_json::json;
use crate::*;


#[near_bindgen]
impl Contract {
    #[payable]
    pub fn redeem_nft(
        &mut self,
        token_id: TokenId
    ) -> Token {
        assert_one_yocto();
        let caller_id = env::predecessor_account_id();

        // let token_metadata = self.tokens.token_metadata_by_id.unwrap().get(&token_id).unwrap();
        let mut token = self.nft_token(token_id.clone()).unwrap();
        let mut token_metadata = token.metadata.as_mut().unwrap();

        assert_eq!(token.owner_id, caller_id, "Error: Token not owned by the caller");

        assert_eq!(token_metadata.extra, Some(json!({"attributes": [{"trait_type": "redeemed", "value": "false"}]}).to_string()));
        token_metadata.extra = Some(json!({"attributes": [{"trait_type": "redeemed", "value": "true"}]}).to_string());

        self.tokens.token_metadata_by_id.as_mut().unwrap().insert(&token_id, &token_metadata);

        token
    }

}