use crate::msg::{InstantiateMsg, QueryMsg, VerifyResponse};
use crate::{execute, instantiate, query};
use cosmwasm_std::{Addr, Binary, StdResult};
use cw_multi_test::{App, ContractWrapper, Executor};

pub struct Secp256k1SigVerifyContract(Addr); // new type represent the address

impl Secp256k1SigVerifyContract {
    pub fn addr(&self) -> &Addr {
        &self.0
    }

    pub fn store_code(app: &mut App) -> u64 {
        let contract = ContractWrapper::new(execute, instantiate, query);
        app.store_code(Box::new(contract))
    }

    #[track_caller]
    pub fn instantiate<'a>(
        app: &mut App,
        code_id: u64,
        sender: &Addr,
        label: &str,
        admin: impl Into<Option<&'a Addr>>,
    ) -> StdResult<Self> {
        let admin = admin.into();

        app.instantiate_contract(
            code_id,
            sender.clone(),
            &InstantiateMsg {},
            &[],
            label,
            admin.map(Addr::to_string),
        )
        .map(Secp256k1SigVerifyContract) // covert InstantiateResult to Secp256k1SigVerifyContract
        .map_err(|err| err.downcast().unwrap())
    }

    #[track_caller]
    pub fn query_verify_cosmos(
        &self,
        app: &App,
        message: Binary,
        signature: Binary,
        public_key: Binary,
    ) -> StdResult<VerifyResponse> {
        app.wrap().query_wasm_smart(
            self.0.clone(),
            &QueryMsg::VerifyCosmosSignature {
                message,
                signature,
                public_key,
            },
        )
    }
}

impl From<Secp256k1SigVerifyContract> for Addr {
    fn from(contract: Secp256k1SigVerifyContract) -> Self {
        contract.0
    }
}
