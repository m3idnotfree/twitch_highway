#![cfg(feature = "conduits")]

#[macro_use]
mod common;

use twitch_highway::{
    conduits::{ConduitsAPI, Shard, UpdateConduitShardsRequest},
    types::ConduitId,
};

api_test!(get_conduits, []);
api_test!(create_conduits, [5]);
api_test!(
    update_conduits,
    [&ConduitId::from("bfcfc993-26b1-b876-44d9-afe75a379dac"), 5]
);
api_test!(
    delete_conduits,
    [&ConduitId::from("bfcfc993-26b1-b876-44d9-afe75a379dac")]
);
api_test!(
    get_conduit_shards,
    [
        &ConduitId::from("bfcfc993-26b1-b876-44d9-afe75a379dac"),
        None,
        None
    ]
);
api_test!(
    update_conduit_shards,
    [
        UpdateConduitShardsRequest::new(ConduitId::from("bfcfc993-26b1-b876-44d9-afe75a379dac"))
            .extend_shards([
                Shard::webhook(
                    "0".to_string(),
                    Some("https://this-is-a-callback.com".to_string()),
                    Some("s3cre7".to_string())
                ),
                Shard::webhook(
                    "1".to_string(),
                    Some("https://this-is-a-callback-2.com".to_string()),
                    Some("s3cre7".to_string())
                ),
                Shard::webhook(
                    "3".to_string(),
                    Some("https://this-is-a-callback-3.com".to_string()),
                    Some("s3cre7".to_string())
                ),
            ])
    ]
);
