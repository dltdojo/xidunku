use bip39::{Language, Mnemonic, MnemonicType, Seed};
use geopattern::geo_pattern::GeoPattern;
use wasm_bindgen::prelude::*;

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
pub fn bip39_string(s: &str) -> String {
    // https://docs.rs/tiny-bip39/0.6.2/bip39/index.html
    /// create a new randomly generated mnemonic phrase
    let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);

    /// get the phrase
    let phrase: &str = mnemonic.phrase();
    println!("phrase: {}", phrase);

    /// get the HD wallet seed
    let seed = Seed::new(&mnemonic, "");

    // get the HD wallet seed as raw bytes
    let seed_bytes: &[u8] = seed.as_bytes();

    // print the HD wallet seed as a hex string
    // println!("{:X}", seed);
    format!("{:X}", seed)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
