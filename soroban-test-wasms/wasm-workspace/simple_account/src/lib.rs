#![no_std]
use soroban_sdk::{auth::Context, contract, contractimpl, contracttype, BytesN, Env, Vec};

#[contract]
struct SimpleAccount;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Owner,
}

#[contractimpl]
impl SimpleAccount {
    pub fn init(env: Env, public_key: BytesN<32>) {
        env.storage()
            .persistent()
            .set(&DataKey::Owner, &public_key, None);
    }

    pub fn set_owner(env: Env, new_owner: BytesN<32>) {
        env.current_contract_address().require_auth();
        env.storage()
            .persistent()
            .set(&DataKey::Owner, &new_owner, None);
    }

    #[allow(non_snake_case)]
    pub fn __check_auth(
        env: Env,
        signature_payload: BytesN<32>,
        signature_args: Vec<BytesN<64>>,
        _auth_context: Vec<Context>,
    ) {
        if signature_args.len() != 1 {
            panic!("incorrect number of signature args");
        }
        let public_key: BytesN<32> = env.storage().persistent().get(&DataKey::Owner).unwrap();
        env.crypto().ed25519_verify(
            &public_key,
            &signature_payload.into(),
            &signature_args.get(0).unwrap(),
        );
    }
}
