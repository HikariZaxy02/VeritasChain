#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Env, String, Symbol, Vec, Address
};

#[contracttype]
#[derive(Clone)]
pub struct Certificate {
    id: u64,
    owner: Address,
    issuer: Address,
    hash: String,
    timestamp: u64,
}

const CERT_DATA: Symbol = symbol_short!("CERT_DATA");
const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct CertificateContract;

#[contractimpl]
impl CertificateContract {

    // Ambil semua sertifikat
    pub fn get_certificates(env: Env) -> Vec<Certificate> {
        env.storage().instance().get(&CERT_DATA).unwrap_or(Vec::new(&env))
    }

    // Buat sertifikat
    pub fn issue_certificate(env: Env, owner: Address, hash: String) -> String {
        let issuer = env.invoker();

        let mut counter: u64 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        counter += 1;

        let mut certs: Vec<Certificate> =
            env.storage().instance().get(&CERT_DATA).unwrap_or(Vec::new(&env));

        let cert = Certificate {
            id: counter,
            owner,
            issuer,
            hash,
            timestamp: env.ledger().timestamp(),
        };

        certs.push_back(cert);

        env.storage().instance().set(&CERT_DATA, &certs);
        env.storage().instance().set(&COUNTER, &counter);

        String::from_str(&env, "Sertifikat berhasil dibuat")
    }

    // Verifikasi sertifikat
    pub fn verify_certificate(env: Env, hash: String) -> String {
        let certs: Vec<Certificate> =
            env.storage().instance().get(&CERT_DATA).unwrap_or(Vec::new(&env));

        for i in 0..certs.len() {
            if certs.get(i).unwrap().hash == hash {
                return String::from_str(&env, "VALID");
            }
        }

        String::from_str(&env, "TIDAK VALID")
    }
}