#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Symbol};

#[contracttype]
#[derive(Clone)]
pub struct EscrowState {
    pub depositor: Address,
    pub beneficiary: Address,
    pub token: Address,
    pub amount: i128,
    pub expiry_ledger: u32,
    pub released: bool,
}

#[contracttype]
pub enum DataKey {
    Escrow(Symbol),
}

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    /// Lock `amount` of `token` into escrow identified by `escrow_id`.
    /// Funds are held until `release` is called or `refund` after expiry.
    pub fn lock(
        env: Env,
        escrow_id: Symbol,
        depositor: Address,
        beneficiary: Address,
        token: Address,
        amount: i128,
        expiry_ledger: u32,
    ) {
        depositor.require_auth();
        assert!(amount > 0, "amount must be positive");
        assert!(
            expiry_ledger > env.ledger().sequence(),
            "expiry must be in the future"
        );
        let key = DataKey::Escrow(escrow_id);
        assert!(
            !env.storage().persistent().has(&key),
            "escrow already exists"
        );
        token::Client::new(&env, &token).transfer(&depositor, &env.current_contract_address(), &amount);
        env.storage().persistent().set(
            &key,
            &EscrowState {
                depositor,
                beneficiary,
                token,
                amount,
                expiry_ledger,
                released: false,
            },
        );
    }

    /// Release funds to the beneficiary. Only the depositor may call this.
    pub fn release(env: Env, escrow_id: Symbol) {
        let key = DataKey::Escrow(escrow_id);
        let mut state: EscrowState = env.storage().persistent().get(&key).expect("not found");
        state.depositor.require_auth();
        assert!(!state.released, "already released");
        state.released = true;
        token::Client::new(&env, &state.token).transfer(
            &env.current_contract_address(),
            &state.beneficiary,
            &state.amount,
        );
        env.storage().persistent().set(&key, &state);
    }

    /// Refund depositor after expiry. Anyone may trigger this.
    pub fn refund(env: Env, escrow_id: Symbol) {
        let key = DataKey::Escrow(escrow_id);
        let mut state: EscrowState = env.storage().persistent().get(&key).expect("not found");
        assert!(!state.released, "already released");
        assert!(
            env.ledger().sequence() > state.expiry_ledger,
            "escrow not yet expired"
        );
        state.released = true;
        token::Client::new(&env, &state.token).transfer(
            &env.current_contract_address(),
            &state.depositor,
            &state.amount,
        );
        env.storage().persistent().set(&key, &state);
    }

    pub fn get_escrow(env: Env, escrow_id: Symbol) -> EscrowState {
        env.storage()
            .persistent()
            .get(&DataKey::Escrow(escrow_id))
            .expect("not found")
    }
}
