// src/lib.rs or src/main.rs

use candid::{
    CandidType,
    Deserialize,
    Nat,
    Principal,
};
use ic_cdk::{
    api::{
        call::{
            call,
        },
        management_canister::{
            main::{
                CanisterIdRecord,
                UpdateSettingsArgument,
                CanisterSettings,
            },
        },
        id,
    },
};
use ic_cdk_macros::{
    query,
    update,
};

#[update]
async fn greet() -> String {
    let settings = CanisterSettings {
        controllers: None,
        compute_allocation: Some(Nat::from(100_u64)),
        memory_allocation: None,
        freezing_threshold: None,
        log_visibility: None,
        reserved_cycles_limit: None,
        wasm_memory_limit: None,
    };
    let request = UpdateSettingsArgument {
        canister_id: id(),
        settings,
    };

    let (_,): ((),) = call(
        Principal::management_canister(), "update_settings", (request,))
    .await
    .unwrap();

    "Updated settings.".to_string()
}
