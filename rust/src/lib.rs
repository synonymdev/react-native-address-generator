uniffi::setup_scaffolding!();
extern crate bitcoin;
extern crate bip39;
extern crate bip32;
extern crate secp256k1;

use bitcoin::{Address, Network, PublicKey as BitcoinPublicKey};
use bip39::{Mnemonic, Language};
use bip32::{XPrv, DerivationPath};
use secp256k1::{Secp256k1, PublicKey};
use bitcoin::key::UntweakedPublicKey;
use serde::{Serialize, Deserialize};
use serde_json;
use hex;
use std::{error::Error, str::FromStr};
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize, Deserialize)]
struct GetAddressResponse {
    address: String,
    path: String,
    public_key: String,
}

#[uniffi::export]
fn get_address(
    mnemonic: String,
    derivation_path: String,
    network_type: String,
    bip39_passphrase: String,
) -> Vec<String> {
    match generate_bitcoin_address_from_mnemonic(&mnemonic, &derivation_path, &network_type, &bip39_passphrase) {
        Ok(response) => match serde_json::to_string(&response) {
            Ok(serialized) => create_response_vector(false, serialized),
            Err(_) => create_response_vector(true, "Error in serialization".to_string()),
        },
        Err(e) => create_response_vector(true, e.to_string()),
    }
}

#[uniffi::export]
fn get_script_hash(
    address: String,
    network: String,
) -> Vec<String> {
    match generate_script_hash(&address, &network) {
        Ok(response) => create_response_vector(false, response),
        Err(e) => create_response_vector(true, e.to_string()),
    }
}


#[uniffi::export]
fn get_private_key(
    mnemonic: String,
    derivation_path: String,
    network_type: String,
    bip39_passphrase: String,
) -> Vec<String> {
    match generate_private_key(&mnemonic, &derivation_path, &network_type, &bip39_passphrase) {
        Ok(response) => create_response_vector(false, response),
        Err(e) => create_response_vector(false, e.to_string()),
    }
}

fn create_response_vector(error: bool, data: String) -> Vec<String> {
    if error {
        vec!["error".to_string(), data]
    } else {
        vec!["success".to_string(), data]
    }
}

fn determine_network(network: &str) -> Result<Network, Box<dyn Error>> {
    match network {
        "bitcoin" | "mainnet" => Ok(Network::Bitcoin),
        "testnet" | "bitcoinTestnet" => Ok(Network::Testnet),
        "regtest" | "bitcoinRegtest" => Ok(Network::Regtest),
        _ => Err("Unsupported network type".into()),
    }
}


fn generate_bitcoin_address_from_mnemonic(
    mnemonic_phrase: &str,
    derivation_path_str: &str,
    network_type: &str,
    bip39_passphrase: &str,
) -> Result<GetAddressResponse, Box<dyn Error>> {
    let network = determine_network(network_type)?;

    // Check if the derivation path is in the correct format
    let path_parts: Vec<&str> = derivation_path_str.split('/').collect();
    if path_parts.len() != 6 || path_parts[0] != "m" {
        return Err("Invalid derivation path format. Expected format example: m/84'/0'/0'/0/0".into());
    }

    // Check if the second number is correct based on the network type
    let second_number = path_parts[2].trim_end_matches('\'').parse::<u32>()?;
    match network_type {
        "bitcoin" | "mainnet" => {
            if second_number != 0 {
                return Err(format!("Invalid Coin number in the derivation path for {}. Expected 0.", network_type).into());
            }
        }
        "testnet" | "bitcoinTestnet" | "regtest" | "bitcoinRegtest" => {
            if second_number != 1 {
                return Err(format!("Invalid Coin number in the derivation path for {}. Expected 1.", network_type).into());
            }
        }
        _ => return Err("Unsupported network type".into()),
    }

    // Parse the mnemonic phrase and derive the private key
    let mnemonic = Mnemonic::parse_in(Language::English, mnemonic_phrase)?;
    let bip39_seed = mnemonic.to_seed(bip39_passphrase);
    let derivation_path = derivation_path_str.parse::<DerivationPath>()?;
    let xprv = XPrv::derive_from_path(&bip39_seed, &derivation_path)?;

    // Create a new Secp256k1 context and derive the public key
    let secp = Secp256k1::new();
    let secret_key = secp256k1::SecretKey::from_slice(&xprv.private_key().to_bytes())?;
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);

    // Convert secp256k1 public key to bitcoin public key
    let bitcoin_public_key = BitcoinPublicKey {
        compressed: true,
        inner: secp256k1::PublicKey::from_slice(&public_key.serialize())?,
    };

    let purpose = derivation_path_str.split('/').nth(1).unwrap_or(""); // Extracts the purpose field

    // Determine the address type based on the derivation path
    let address = match purpose {
        "44'" => Address::p2pkh(&bitcoin_public_key, network),
        "49'" => Address::p2shwpkh(&bitcoin_public_key, network)?,
        "84'" => Address::p2wpkh(&bitcoin_public_key, network)?,
        "86'" => {
            let uncompressed_pubkey = PublicKey::from_slice(&public_key.serialize_uncompressed())?;
            let internal_key = UntweakedPublicKey::from_slice(&uncompressed_pubkey.serialize()[1..]).unwrap();
            let merkle_root = None; // Set the merkle_root to None for a single public key
            Address::p2tr(&secp, internal_key, merkle_root, network)
        }
        _ => Err("Unsupported derivation path")?,
    };

    let address_string = address.to_string();
    let public_key_string = bitcoin_public_key.to_string();

    Ok(GetAddressResponse {
        address: address_string,
        path: derivation_path_str.to_string(),
        public_key: public_key_string,
    })
}

fn generate_script_hash(address: &str, network_type: &str) -> Result<String, Box<dyn Error>> {
    let network = determine_network(&network_type)?;

    // Parse the address
    let addr = Address::from_str(&address)?.require_network(network)?;

    // Get the script from the address
    let script = addr.script_pubkey();
    let script_bytes = script.as_bytes();

    // Calculate the script hash
    let hash = Sha256::digest(script_bytes);

    // Reverse the bytes of the hash
    let mut reversed_hash = hash.to_vec();
    reversed_hash.reverse();

    // Convert the reversed hash to hexadecimal representation
    let script_hash_hex = hex::encode(reversed_hash);

    Ok(script_hash_hex.to_string())
}

fn generate_private_key(
    mnemonic_phrase: &str,
    derivation_path_str: &str,
    network_type: &str,
    bip39_passphrase: &str,
) -> Result<String, Box<dyn Error>> {
    let network = determine_network(&network_type)?;

    // Parse the mnemonic phrase and derive the private key
    let mnemonic = Mnemonic::parse_in(Language::English, mnemonic_phrase)?;
    let bip39_seed = mnemonic.to_seed(bip39_passphrase);
    let derivation_path = derivation_path_str.parse::<DerivationPath>()?;
    let xprv = XPrv::derive_from_path(&bip39_seed, &derivation_path)?;

    let secret_key = secp256k1::SecretKey::from_slice(&xprv.private_key().to_bytes())?;

    // Convert the private key to WIF format
    let private_key_wif = bitcoin::PrivateKey {
        compressed: true,
        network,
        inner: secret_key,
    };
    let private_key_string = private_key_wif.to_wif();

    Ok(private_key_string)
}
