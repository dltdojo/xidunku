use bip39::{Language, Mnemonic, Seed};
use core::convert::TryFrom;
use ed25519_dalek::Keypair;
use ed25519_dalek::Signature;
use geopattern::geo_pattern::GeoPattern;
use libra_crypto::{traits::Uniform, vrf::ecvrf::*};
use sha2::{Digest, Sha512};
use sha3::Sha3_256;
use tiny_hderive::bip32::ExtendedPrivKey;
use wasm_bindgen::prelude::*;

macro_rules! libra_crypto_hex_string {
    ($e:expr) => {
        format!("{}", ::hex::encode($e.to_bytes().as_ref()))
    };
}

macro_rules! libra_crypto_from_hex {
    (CompressedEdwardsY, $e:expr) => {
        CompressedEdwardsY::from_slice(&::hex::decode($e).unwrap())
            .decompress()
            .unwrap()
    };
    (VRFPublicKey, $e:expr) => {{
        let v: &[u8] = &::hex::decode($e).unwrap();
        VRFPublicKey::try_from(v).unwrap()
    }};
    ($t:ty, $e:expr) => {
        <$t>::try_from(::hex::decode($e).unwrap().as_ref()).unwrap()
    };
}

#[wasm_bindgen]
pub fn ipfs_file_name(content: &str) -> String {
    let mh_sha256_ipfs =
        parity_multihash::encode(parity_multihash::Hash::SHA2256, content.as_bytes()).unwrap();
    bs58::encode(mh_sha256_ipfs.as_bytes()).into_string()
}

#[wasm_bindgen]
pub fn multihash_sha2_256(content: &str) -> String {
    let x = parity_multihash::encode(parity_multihash::Hash::SHA2256, content.as_bytes()).unwrap();
    hex::encode(x)
}

#[wasm_bindgen]
pub fn multihash_sha3_256(content: &str) -> String {
    let x = parity_multihash::encode(parity_multihash::Hash::SHA3256, content.as_bytes()).unwrap();
    hex::encode(x)
}

#[wasm_bindgen]
pub fn geopattern_gen_minified_svg_string(s: &str) -> String {
    GeoPattern::new(s)
        .build()
        .unwrap()
        .to_minified_svg()
        .unwrap()
}

#[wasm_bindgen]
pub fn geopattern_gen_base64_svg_string(s: &str) -> String {
    GeoPattern::new(s).build().unwrap().to_base64().unwrap()
}

#[wasm_bindgen]
pub fn bip39_seed(phrase: &str) -> String {
    // https://docs.rs/tiny-bip39/0.6.2/bip39/index.html
    // create a new randomly generated mnemonic phrase
    let seed = bip39_to_seed(phrase);
    format!("{:x}", seed)
}

pub fn bip39_to_seed(phrase: &str) -> Seed {
    // https://docs.rs/tiny-bip39/0.6.2/bip39/index.html
    // create a new randomly generated mnemonic phrase
    let lang = Language::ChineseTraditional;
    //let phrase: &str = "谷 遵 亦 園 焰 坡 复 虛 鋼 表 閃 糾";
    let mnemonic: Mnemonic = Mnemonic::from_phrase(phrase, lang).unwrap();
    let password = "";
    Seed::new(&mnemonic, password)
}

#[wasm_bindgen]
pub fn hd_256k1_key(phrase: &str, path: &str) -> String {
    // [tiny_hderive - Rust](https://docs.rs/tiny-hderive/0.2.1/tiny_hderive/)
    let seed = bip39_to_seed(phrase);
    // let ext = ExtendedPrivKey::derive(seed.as_bytes(), "m/44'/60'/0'/0/0").unwrap();
    let ext = ExtendedPrivKey::derive(seed.as_bytes(), path).unwrap();
    hex::encode(&ext.secret())
}

#[wasm_bindgen]
pub fn sha3msg256(msg: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.input(msg);
    let result = hasher.result();
    hex::encode(result)
}

#[wasm_bindgen]
pub fn sha2msg512(msg: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.input(msg);
    let result = hasher.result();
    hex::encode(result)
}

#[wasm_bindgen]
pub fn libra_vrf_proof(alpha: &str, sk: &str) -> String {
    //
    // https://github.com/libra/libra/blob/master/crypto/crypto/src/vrf/ecvrf.rs
    // https://tools.ietf.org/html/draft-irtf-cfrg-vrf-04
    // SK
    // let private_key = VRFPrivateKey::generate_for_testing(&mut rng);
    let private_key = libra_crypto_from_hex!(VRFPrivateKey, sk);
    // PK
    // let public_key: VRFPublicKey = (&private_key).into();
    //  pi = VRF_prove(SK, alpha)
    let proof = private_key.prove(alpha.as_bytes());
    // beta = VRF_proof_to_hash(pi)
    // OUTPUT_LENGTH: usize = 64;
    // ECVRF-ED25519-SHA512-TAI
    // SHA-2 SHA-512
    // let output: Output = (&proof).into();
    // Produce a pseudorandom output from a `Proof`:
    //let _pk = libra_crypto_hex_string!(public_key);
    // let _output = libra_crypto_hex_string!(output);
    let proof_hex = libra_crypto_hex_string!(proof);
    // VRF_verify(PK, alpha, pi)
    // public_key.verify(pi,alpha)
    // assert!(public_key.verify(&proof, alpha.as_bytes()).is_ok());
    format!("{}", proof_hex)
}

#[wasm_bindgen]
pub fn libra_vrf_output(proof_hex: &str) -> String {
    let proof: Proof = libra_crypto_from_hex!(Proof, proof_hex);
    let output: Output = (&proof).into();
    let output_hex = libra_crypto_hex_string!(output);
    format!("{}", output_hex)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};
    #[test]
    fn test_sha3msg256() {
        assert_eq!(
            "3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532",
            sha3msg256("abc")
        );
    }

    #[test]
    fn test_bip39_seed() {
        //let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::ChineseTraditional);
        //let phrase: &str = mnemonic.phrase();
        // let lang = Language::English;
        //let phrase: &str = "credit tone calm expire step neither scene waste hat wheel tone antenna";
        let lang = Language::ChineseTraditional;
        let phrase: &str = "谷 遵 亦 園 焰 坡 复 虛 鋼 表 閃 糾";
        let mnemonic: Mnemonic = Mnemonic::from_phrase(phrase, lang).unwrap();
        let password = "";
        let seed = Seed::new(&mnemonic, password);
        assert_eq!("805212f7a300b02a82ab79f8548204911404052a9f4d5353258799aa82af4760fab1cabc0c759b7771a4ef46792186a5cd2594c1b542c1d1d1edd4d857e7932b", format!("{:x}",seed));
    }
    #[test]
    fn test_hd_key() {
        // https://iancoleman.io/bip39/#chinese_traditional
        let phrase = "牌 按 幼 算 玄 液 施 老 檔 胺 督 傾";
        // bip39 seed 2e58df5995c301212001ec668a0887cbc3ec814ffab3a440b86b0c456e47146781ec2e724f42bfadb2b07934d3f1e244c59886e14097f7836f7c912aade7f918
        let path = "m/44'/0'/0'/0/0";
        // BIP32 Extended key = xprvA1kjJabh2XTxx64fGVA6RQziuCTvc7VnBL5UND9K7QXGZF9uCdJ88jtJzZZTzVYBjL8dDeZsLGbkj3G1LuSimB2BDtgXf8P3fPkUqPb9x5g
        // private key L4ak82DxxLF5QN7rtyVhyJ9gg4u4FBdqq24uBho84MYFNnfrAFWd
        // public key 03f345e815aa3583301e6b55d0213e6d54bb8abbfa2348c203791bf7184c8a7cf1
        let keyhex = hd_256k1_key(phrase, path);
        assert_eq!(
            "dbaea1b008afe6ad61ff16baf89698072853efd02982e9dd0a7525a82cbc662c",
            keyhex
        );
        let sk: secp256k1::key::SecretKey =
            secp256k1::key::SecretKey::from_slice(&hex::decode(keyhex).unwrap()).unwrap();

        let secp = secp256k1::Secp256k1::new();
        let pk: secp256k1::key::PublicKey = secp256k1::key::PublicKey::from_secret_key(&secp, &sk);
        assert_eq!(
            "dbaea1b008afe6ad61ff16baf89698072853efd02982e9dd0a7525a82cbc662c",
            format!("{:x}", sk)
        );
        assert_eq!(
            "03f345e815aa3583301e6b55d0213e6d54bb8abbfa2348c203791bf7184c8a7cf1",
            format!("{:x}", pk)
        );
        let compressed = true;
        let network = bitcoin::network::constants::Network::Bitcoin;
        let bsk = bitcoin::util::key::PrivateKey {
            compressed: compressed,
            network: network,
            key: sk,
        };
        assert_eq!(
            "L4ak82DxxLF5QN7rtyVhyJ9gg4u4FBdqq24uBho84MYFNnfrAFWd",
            bsk.to_wif()
        );
    }

    #[test]
    fn test_vrf_proof() {
        // https://github.com/libra/libra/blob/master/crypto/crypto/src/vrf/unit_tests/vrf_test.rs#L47
        const SK: &str = "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60";
        let proof = libra_vrf_proof("", SK);
        assert_eq!(proof, "9275df67a68c8745c0ff97b48201ee6db447f7c93b23ae24cdc2400f52fdb08a1a6ac7ec71bf9c9c76e96ee4675ebff60625af28718501047bfd87b810c2d2139b73c23bd69de66360953a642c2a330a");
    }

    #[test]
    fn test_ipfs_file_name() {
        let filename = ipfs_file_name("hello");
        assert_eq!("QmRN6wdp1S2A5EtjW9A3M1vKSBuQQGcgvuhoMUoEz4iiT5", filename);
    }

    #[test]
    fn test_multihash() {
        // multiformats/multihash: Self describing hashes - for future proofing https://github.com/multiformats/multihash
        assert_eq!(
            "12209cbc07c3f991725836a3aa2a581ca2029198aa420b9d99bc0e131d9f3e2cbe47",
            multihash_sha2_256("multihash")
        );
    }
}
