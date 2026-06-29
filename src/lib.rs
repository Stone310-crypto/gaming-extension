//! Stone Gaming Extension
//!
//! Bietet Spiele-Registrierung, Item-Trading und Marktplatz-Funktionen
//! für das Stone Dashboard. Wird als WASM-Modul geladen.

use serde::{Deserialize, Serialize};

// ─── Init / Shutdown ────────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn init() {
    // Extension initialisieren
}

#[no_mangle]
pub extern "C" fn shutdown() {
    // Cleanup
}

#[no_mangle]
pub extern "C" fn name() -> *const u8 {
    b"Gaming Extension\0".as_ptr()
}

#[no_mangle]
pub extern "C" fn version() -> *const u8 {
    b"1.0.0\0".as_ptr()
}

// ─── Game-Registrierung ─────────────────────────────────────────────────────

#[derive(Serialize, Deserialize)]
pub struct GameRegistration {
    pub game_id: String,
    pub name: String,
    pub genre: String,
    pub description: String,
    pub website: String,
}

#[derive(Serialize, Deserialize)]
pub struct GameInfo {
    pub game_id: String,
    pub name: String,
    pub genre: String,
    pub players_online: u32,
    pub total_items: u32,
}

#[no_mangle]
pub extern "C" fn register_game(json_ptr: *const u8, json_len: usize) -> *const u8 {
    let json = unsafe { std::slice::from_raw_parts(json_ptr, json_len) };
    let json_str = std::str::from_utf8(json).unwrap_or("{}");

    let result = match serde_json::from_str::<GameRegistration>(json_str) {
        Ok(_game) => b"{\"ok\": true}\0".as_ptr(),
        Err(e) => {
            let err = format!("{{\"ok\": false, \"error\": \"{}\"}}\0", e);
            err.as_ptr()
        }
    };
    result
}

// ─── Item-Trading ───────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize)]
pub struct TradeOffer {
    pub offer_id: String,
    pub from_wallet: String,
    pub to_wallet: String,
    pub item_id: String,
    pub amount: String,
}

#[no_mangle]
pub extern "C" fn create_trade_offer(json_ptr: *const u8, json_len: usize) -> *const u8 {
    let json = unsafe { std::slice::from_raw_parts(json_ptr, json_len) };
    let json_str = std::str::from_utf8(json).unwrap_or("{}");

    let result = match serde_json::from_str::<TradeOffer>(json_str) {
        Ok(offer) => {
            let resp = format!(
                "{{\"ok\": true, \"offer_id\": \"{}\"}}\0",
                offer.offer_id
            );
            // In Produktion: Trade über StoneChain abwickeln
            resp.into_bytes()
        }
        Err(_) => b"{\"ok\": false, \"error\": \"invalid offer\"}\0".to_vec(),
    };
    // Leak für FFI (WASM-Lebensdauer)
    Box::leak(result.into_boxed_slice()).as_ptr()
}

// ─── Marktplatz ─────────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize)]
pub struct MarketplaceItem {
    pub item_id: String,
    pub name: String,
    pub game_id: String,
    pub price: String,
    pub seller_wallet: String,
}

#[no_mangle]
pub extern "C" fn list_marketplace_items() -> *const u8 {
    // In Produktion: Items von der Blockchain laden
    let items = vec![
        MarketplaceItem {
            item_id: "item_001".into(),
            name: "Legendäres Schwert".into(),
            game_id: "game_001".into(),
            price: "100.0".into(),
            seller_wallet: "abc123...".into(),
        },
    ];

    let json = serde_json::to_string(&items).unwrap_or_else(|_| "[]".into());
    let json_null = format!("{json}\0");
    json_null.into_bytes().leak().as_ptr()
}
