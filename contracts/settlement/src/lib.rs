#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Symbol};

#[contracttype]
#[derive(Clone)]
pub struct SettlementRecord {
    pub payer: Address,
    pub payee: Address,
    pub token: Address,
    pub amount: i128,
    pub provider_ref: Symbol,
    pub settled: bool,
}

#[contracttype]
pub enum DataKey {
    Settlement(Symbol),
}

#[contract]
pub struct SettlementContract;

#[contractimpl]
impl SettlementContract {
    /// Initiate a settlement, transferring `amount` of `token` from `payer` to `payee`.
    pub fn settle(
        env: Env,
        settlement_id: Symbol,
        payer: Address,
        payee: Address,
        token: Address,
        amount: i128,
        provider_ref: Symbol,
    ) {
        payer.require_auth();
        assert!(amount > 0, "amount must be positive");
        let key = DataKey::Settlement(settlement_id);
        assert!(
            !env.storage().persistent().has(&key),
            "settlement already exists"
        );
        token::Client::new(&env, &token).transfer(&payer, &payee, &amount);
        env.storage().persistent().set(
            &key,
            &SettlementRecord {
                payer,
                payee,
                token,
                amount,
                provider_ref,
                settled: true,
            },
        );
    }

    pub fn get_settlement(env: Env, settlement_id: Symbol) -> SettlementRecord {
        env.storage()
            .persistent()
            .get(&DataKey::Settlement(settlement_id))
            .expect("not found")
    }
}
