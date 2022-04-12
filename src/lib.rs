#![feature(once_cell)]
#![feature(result_flattening)]
mod utils;

use crate::bitcoin::Script;
use crate::bitcoin::XOnlyPublicKey;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

use core::str::FromStr;
use sapio_miniscript::*;
#[wasm_bindgen]
pub fn compile(s: &str) -> Result<String, JsValue> {
    let pol = policy::Concrete::from_str(s).map_err(|e| e.to_string())?;
    let ms: Miniscript<String, Tap> = pol.compile().map_err(|e| e.to_string())?;
    Ok(ms.to_string())
}


use sapio_miniscript::{ descriptor::DescriptorPublicKey, Descriptor, descriptor::DescriptorType, descriptor::Sh};

// get descriptor types in javascript. copy from rust-miniscript's DescriptorType enum
#[derive(Serialize, Deserialize)]
pub struct DescriptorTypeStruct {
    /// Bare descriptor(Contains the native P2pk)
    pub Bare: &'static str,
    /// Pure Sh Descriptor. Does not contain nested Wsh/Wpkh
    pub Sh: &'static str,
    /// Pkh Descriptor
    pub Pkh: &'static str,
    /// Wpkh Descriptor
    Wpkh: &'static str,
    /// Wsh
    Wsh: &'static str,
    /// Sh Wrapped Wsh
    ShWsh: &'static str,
    /// Sh wrapped Wpkh
    ShWpkh: &'static str,
    /// Sh Sorted Multi
    ShSortedMulti: &'static str,
    /// Wsh Sorted Multi
    WshSortedMulti: &'static str,
    /// Sh Wsh Sorted Multi
    ShWshSortedMulti: &'static str,
}

const BARE: &'static str = "Bare";
const SH: &'static str = "Sh";
const PKH: &'static str = "Pkh";
const WPKH: &'static str = "Wpkh";
const WSH: &'static str = "Wsh";
const SH_WSH: &'static str = "ShWsh";
const SH_WPKH: &'static str = "ShWpkh";
const SH_SORTED_MULTI: &'static str = "ShSortedMulti";
const WSH_SORTED_MULTI: &'static str = "WshSortedMulti";
const SH_WSH_SORTED_MULTI: &'static str = "ShWshSortedMulti";

pub const DESCRIPTOR_TYPES: DescriptorTypeStruct = DescriptorTypeStruct {
        Bare: BARE,
        Sh: SH,
        Pkh: PKH,
        Wpkh: WPKH,
        Wsh: WSH,
        ShWsh: SH_WSH,
        ShWpkh: SH_WPKH,
        ShSortedMulti: SH_SORTED_MULTI,
        WshSortedMulti: WSH_SORTED_MULTI,
        ShWshSortedMulti: SH_WSH_SORTED_MULTI,
    };

#[wasm_bindgen]
pub fn get_descriptor_types() -> Result<JsValue, JsValue> {
    return Ok(serde_wasm_bindgen::to_value(&DESCRIPTOR_TYPES)?);
}

#[wasm_bindgen]
pub fn get_script_type(s: &str) -> Result<String, String> {
    let descriptor =  match Descriptor::<DescriptorPublicKey>::from_str(s) {
        Ok(desc) => desc,
        Err(e) => return Err(e.to_string()),
    };

    let desc_type = match descriptor.desc_type() {
        DescriptorType::Bare => BARE,
        DescriptorType::Pkh => PKH,
        DescriptorType::Wpkh => WPKH,
        DescriptorType::Wsh => WSH,
        DescriptorType::ShWsh => SH_WSH,
        DescriptorType::ShSortedMulti => SH_SORTED_MULTI,
        DescriptorType::WshSortedMulti => WSH_SORTED_MULTI,
        DescriptorType::ShWshSortedMulti => SH_WSH_SORTED_MULTI,
        _ => ""
    };
    Ok(desc_type.to_string())
}


use sapio_miniscript::Miniscript;
#[wasm_bindgen]
pub fn get_threshold_count(s: &str) -> Result<String, String> {
    let descriptor =  match Descriptor::<DescriptorPublicKey>::from_str(s) {
        Ok(desc) => desc,
        Err(e) => return Err(e.to_string()),
    };

    let threshold = match descriptor {
        Descriptor::Sh (desc) => {
            match desc.into_inner() {
                descriptor::ShInner::SortedMulti(inner) => inner.k,
                descriptor::ShInner::Wsh(sh_inner) => {
                    match sh_inner.as_inner() {
                        descriptor::WshInner::SortedMulti(inner) => inner.k,
                        _ => return Err("no multisig found in nested wsh".to_string())
                    }
                }
                _ => return Err("No threshold".to_string())
            }
        },
        Descriptor::Wsh (desc) => {
            match desc.into_inner() {
                descriptor::WshInner::SortedMulti(inner) => inner.k,
                _ => return Err("No threshold".to_string())
            }
        },
        _ => return Err("Descriptor type does not have threshold".to_string())
    };

    Ok(threshold.to_string())
}

use bitcoin::secp256k1::VerifyOnly;
use sapio_miniscript::bitcoin::secp256k1::Secp256k1;
use std::lazy::SyncLazy;
static SECP: SyncLazy<Secp256k1<VerifyOnly>> = SyncLazy::new(|| Secp256k1::verification_only());
use bitcoin::util::taproot::TaprootSpendInfo;
use sapio_miniscript::TranslatePk;
#[wasm_bindgen]
#[derive(Debug)]
pub struct KeyTab {
    v: HashMap<String, String>,
}

#[wasm_bindgen]
impl KeyTab {
    pub fn new() -> KeyTab {
        KeyTab { v: HashMap::new() }
    }
    pub fn add(&mut self, k: String, v: String) {
        self.v.insert(k, v);
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Fragments {
    v: Vec<String>,
}

#[wasm_bindgen]
impl Fragments {
    pub fn new() -> Self {
        Fragments { v: vec![] }
    }
    pub fn add(&mut self, s: String) {
        self.v.push(s)
    }
    pub fn add_all(&mut self, s: Box<[JsValue]>) -> bool {
        for v in s.iter() {
            if let Some(st) = v.as_string() {
                self.v.push(st)
            } else {
                return false;
            }
        }
        return true;
    }
}

#[wasm_bindgen]
pub fn taproot(frags: Fragments, keytab: &KeyTab) -> Result<String, JsValue> {
    let key = keytab
        .v
        .iter()
        .map(|(k, v)| XOnlyPublicKey::from_str(&v).map(|key| (k, key)))
        .collect::<Result<HashMap<_, _>, _>>()
        .map_err(|e| e.to_string())?;
    let ms: Vec<Miniscript<_, _>> = frags
        .v
        .iter()
        .map(|s| Miniscript::<String, Tap>::from_str(&s).map_err(|e| e.to_string()))
        .collect::<Result<Vec<Miniscript<_, _>>, _>>()
        .map_err(|e| e.to_string())?;

    let fpk: &Fn(&String) -> Result<XOnlyPublicKey, _> = &|k| {
        key.get(&k)
            .cloned()
            .ok_or_else(|| format!("Missing Key: {}", k))
    };
    let scripts: Vec<(u32, Script)> = ms
        .iter()
        .map(|s| {
            s.translate_pk(fpk, |k| Err(format!("No PKH Support for {}", k)))
                .map(|s: Miniscript<XOnlyPublicKey, Tap>| (1, s.encode()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    use bitcoin::hashes::Hash;
    let nums: XOnlyPublicKey = {
        let mut b = bitcoin::hashes::sha256::Hash::hash("Hello".as_bytes()).into_inner();
        loop {
            if let Ok(k) = XOnlyPublicKey::from_slice(&b[..]) {
                break k;
            } else {
                b = bitcoin::hashes::sha256::Hash::hash(&b[..]).into_inner();
            }
        }
    };
    let tsi =
        TaprootSpendInfo::with_huffman_tree(&SECP, nums, scripts).map_err(|e| e.to_string())?;
    use sapio_miniscript::bitcoin::hashes::hex::ToHex;
    let js = serde_json::json! {{
        "tweak": tsi.tap_tweak().as_hash().to_hex(),
        "internal_key": tsi.internal_key().to_hex(),
        "merkle_root": tsi.merkle_root().map(|m|m.to_hex()),
        "scripts": tsi.as_script_map().iter().collect::<Vec<_>>(),
        "address":{
            "main": sapio_miniscript::bitcoin::Address::p2tr_tweaked(tsi.output_key(),    bitcoin::network::constants::Network::Bitcoin),
            "test": sapio_miniscript::bitcoin::Address::p2tr_tweaked(tsi.output_key(),    bitcoin::network::constants::Network::Testnet),
            "regtest": sapio_miniscript::bitcoin::Address::p2tr_tweaked(tsi.output_key(), bitcoin::network::constants::Network::Regtest),
            "signet": sapio_miniscript::bitcoin::Address::p2tr_tweaked(tsi.output_key(),  bitcoin::network::constants::Network::Signet),
        }
    }};
    Ok(serde_json::to_string_pretty(&js).map_err(|e| e.to_string())?)
}
