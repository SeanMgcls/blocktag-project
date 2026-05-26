#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, BytesN, Env, String};

// Keys for managing contract state
#[contracttype]
pub enum DataKey {
    Admin,
    Owner(BytesN<32>), // Maps a Certificate ID to its Owner's Address
    Uri(BytesN<32>),   // Maps a Certificate ID to its metadata URI
}

#[contract]
pub struct BlockTagContract;

#[contractimpl]
impl BlockTagContract {
    /// Initializes the contract by setting the trusted Authenticator (Admin).
    /// This ensures only verified authenticators can mint physical item certificates.
    pub fn init(env: Env, admin: Address) {
        admin.require_auth();
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Contract already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    /// Authenticator mints a digital certificate NFT to the seller's wallet.
    pub fn mint(env: Env, admin: Address, to: Address, cert_id: BytesN<32>, uri: String) {
        admin.require_auth();
        
        let stored_admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        if admin != stored_admin {
            panic!("Only the authenticator can mint");
        }
        
        if env.storage().persistent().has(&DataKey::Owner(cert_id.clone())) {
            panic!("Certificate already exists");
        }
        
        // Mint to seller and store metadata URI
        env.storage().persistent().set(&DataKey::Owner(cert_id.clone()), &to);
        env.storage().persistent().set(&DataKey::Uri(cert_id), &uri);
    }

    /// Seller transfers the digital certificate to the buyer upon cash payment.
    pub fn transfer(env: Env, from: Address, to: Address, cert_id: BytesN<32>) {
        from.require_auth();
        
        let owner: Address = env.storage().persistent().get(&DataKey::Owner(cert_id.clone())).expect("Certificate not found");
        if owner != from {
            panic!("Not the authorized owner");
        }
        
        // Transfer ownership to the buyer
        env.storage().persistent().set(&DataKey::Owner(cert_id), &to);
    }

    /// Buyer scans the QR code to verify who currently owns the physical item's digital certificate.
    pub fn get_owner(env: Env, cert_id: BytesN<32>) -> Address {
        env.storage().persistent().get(&DataKey::Owner(cert_id)).expect("Certificate not found")
    }
}