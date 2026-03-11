#[macro_use]
mod common;

use common::HttpMock;
use twitch_highway::{
    entitlements::{EntitlementsAPI, FulfillmentStatus},
    types::{EntitlementId, GameId, UserId},
};

#[tokio::test]
async fn get_drops_entitlements() {
    let suite = HttpMock::new().await;
    suite.get_drops_entitlements().await;

    let result = suite
        .api()
        .get_drops_entitlements()
        .user_id(&UserId::from("25009227"))
        .game_id(&GameId::from("33214"))
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn update_drops_entitlements() {
    let suite = HttpMock::new().await;
    suite.update_drops_entitlements().await;

    let result = suite
        .api()
        .update_drops_entitlements()
        .entitlement_ids(&[
            EntitlementId::from("fb78259e-fb81-4d1b-8333-34a06ffc24c0"),
            EntitlementId::from("862750a5-265e-4ab6-9f0a-c64df3d54dd0"),
            EntitlementId::from("d8879baa-3966-4d10-8856-15fdd62cce02"),
            EntitlementId::from("9a290126-7e3b-4f66-a9ae-551537893b65"),
        ])
        .fulfillment_status(FulfillmentStatus::FULFILLED)
        .send()
        .await;

    assert!(result.is_ok());
}
