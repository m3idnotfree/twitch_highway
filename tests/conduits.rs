#![cfg(feature = "conduits")]

#[macro_use]
mod common;

use twitch_highway::{
    conduits::ConduitsAPI,
    types::{ConduitId, ShardId},
};

api_test!(
    get_conduits[],
    create_conduits[5],
    update_conduits[&ConduitId::from("bfcfc993-26b1-b876-44d9-afe75a379dac"), 5],
    delete_conduits[&ConduitId::from("bfcfc993-26b1-b876-44d9-afe75a379dac")]
);

api_test!(
    get_conduit_shards | api | {
        api.get_conduit_shards(&ConduitId::from("bfcfc993-26b1-b876-44d9-afe75a379dac"))
    }
);

api_test!(
    update_conduit_shards | api | {
        api.update_conduit_shards(ConduitId::from("bfcfc993-26b1-b876-44d9-afe75a379dac"))
            .webhook(
                ShardId::from("0"),
                "https://this-is-a-callback.com",
                "s3cre7",
            )
            .webhook(
                ShardId::from("1"),
                "https://this-is-a-callback-2.com",
                "s3cre7",
            )
            .webhook(
                ShardId::from("3"),
                "https://this-is-a-callback-3.com",
                "s3cre7",
            )
    }
);
