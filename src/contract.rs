use crate::msg::InstantiateMsg;
use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn instantiate(deps: DepsMut, _info: MessageInfo, _msg: InstantiateMsg) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new())
}

pub mod query {
    use cosmwasm_std::{Deps, StdResult};
    use sha2::{Digest, Sha256};

    use crate::msg::VerifyResponse;

    pub fn query_verify_cosmos(
        deps: Deps,
        message: &[u8],
        signature: &[u8],
        public_key: &[u8],
    ) -> StdResult<VerifyResponse> {
        // Hashing
        let hash = Sha256::digest(message);

        // Verification
        let result = deps
            .api
            .secp256k1_verify(hash.as_ref(), signature, public_key);
        match result {
            Ok(verifies) => Ok(VerifyResponse { verifies }),
            Err(err) => Err(err.into()),
        }
    }
}
