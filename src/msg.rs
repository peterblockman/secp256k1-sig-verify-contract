use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Binary;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
#[derive(QueryResponses)] // only for query
pub enum QueryMsg {
    /// Cosmos format (secp256k1 verification scheme).
    #[returns(VerifyResponse)]
    VerifyCosmosSignature {
        /// Message to verify.
        message: Binary,
        /// Serialized signature. Cosmos format (64 bytes).
        signature: Binary,
        /// Serialized compressed (33 bytes) or uncompressed (65 bytes) public key.
        public_key: Binary,
    },
}

#[cw_serde]
pub struct VerifyResponse {
    pub verifies: bool,
}

#[cw_serde]
pub enum ExecMsg {}
