use crate::multitest::contract::Secp256k1SigVerifyContract;
use cosmwasm_std::{coins, Addr, Binary};
use cw_multi_test::App;
const ATOM: &str = "atom";

#[test]
fn should_verify() {
    let owner = Addr::unchecked("owner");
    let sender = Addr::unchecked("sender");

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender, coins(10, ATOM))
            .unwrap();
    });

    let code_id = Secp256k1SigVerifyContract::store_code(&mut app);

    let contract = Secp256k1SigVerifyContract::instantiate(
        &mut app,
        code_id,
        &owner,
        "Counting contract",
        None,
    )
    .unwrap();

    // run `node client.js` to generate these values
    pub const SECP256K1_MESSAGE_HEX: &str = "48656c6c6f20776f726c6421";
    pub const SECP256K1_SIGNATURE_HEX: &str = "8298646e993bb7796a168a93ce811f0e197ae28185ac569370f0d99e9a1b8839651c54016354edccd26d0aefc81dab48abb55d14ab2e34c53b8c12f7c1cf5dff";
    pub const SECP256K1_PUBLIC_KEY_HEX: &str =
    "04ad1f844c06b0a58f53902af7675e819fbd1a2f02c058489dc9e51a4599490b4f3f7e465edc532dd687cb2d9a22da97a8569d2977c7bdea2c37f60a8306ff6042";

    let message = hex::decode(SECP256K1_MESSAGE_HEX).unwrap();
    let signature = hex::decode(SECP256K1_SIGNATURE_HEX).unwrap();
    let public_key = hex::decode(SECP256K1_PUBLIC_KEY_HEX).unwrap();

    let res = contract
        .query_verify_cosmos(
            &mut app,
            Binary::from(message),
            Binary::from(signature),
            Binary::from(public_key),
        )
        .unwrap();

    println!("res {:?}", res);

    assert_eq!(res.verifies, true)
}
