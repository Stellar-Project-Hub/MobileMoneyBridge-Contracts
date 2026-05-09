#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, Vec};

#[contracttype]
#[derive(Clone)]
pub struct Provider {
    pub name: Symbol,
    pub admin: Address,
    pub active: bool,
}

#[contracttype]
pub enum DataKey {
    Provider(Symbol),
    ProviderList,
}

#[contract]
pub struct RegistryContract;

#[contractimpl]
impl RegistryContract {
    /// Register a new mobile money provider.
    pub fn register(env: Env, provider_id: Symbol, name: Symbol, admin: Address) {
        admin.require_auth();
        let key = DataKey::Provider(provider_id.clone());
        assert!(
            !env.storage().persistent().has(&key),
            "provider already registered"
        );
        env.storage()
            .persistent()
            .set(&key, &Provider { name, admin, active: true });

        let mut list: Vec<Symbol> = env
            .storage()
            .persistent()
            .get(&DataKey::ProviderList)
            .unwrap_or(Vec::new(&env));
        list.push_back(provider_id);
        env.storage().persistent().set(&DataKey::ProviderList, &list);
    }

    /// Deactivate a provider. Only the provider's admin may call this.
    pub fn deactivate(env: Env, provider_id: Symbol) {
        let key = DataKey::Provider(provider_id);
        let mut provider: Provider = env.storage().persistent().get(&key).expect("not found");
        provider.admin.require_auth();
        provider.active = false;
        env.storage().persistent().set(&key, &provider);
    }

    pub fn get_provider(env: Env, provider_id: Symbol) -> Provider {
        env.storage()
            .persistent()
            .get(&DataKey::Provider(provider_id))
            .expect("not found")
    }

    pub fn list_providers(env: Env) -> Vec<Symbol> {
        env.storage()
            .persistent()
            .get(&DataKey::ProviderList)
            .unwrap_or(Vec::new(&env))
    }
}
