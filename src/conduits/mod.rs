pub mod request;
pub mod response;
pub mod types;

use crate::{
    conduits::{request::UpdateConduitShardsRequest, response::ConduitResponse},
    request::NoContent,
    types::{ConduitId, Status},
};

const CONDUITS: &str = "conduits";
const EVENTSUB: &str = "eventsub";

endpoints! {
    ConduitsAPI {
        fn get_conduits (&self) -> ConduitResponse {
            endpoint_type: GetConduits,
            method: GET,
            path: [EVENTSUB, CONDUITS],
        }

        fn create_conduits(&self, shard_count: u64) -> ConduitResponse {
            endpoint_type: CreateConduits,
            method: POST,
            path: [EVENTSUB, CONDUITS],
            headers: [json],
            body: {
                let body = serde_json::json!({
                    "shard_count": shard_count
                });

                Some(serde_json::to_string(&body).unwrap())
            }
        }

        fn update_conduits(
            &self,
            conduit_id: &ConduitId,
            shard_count: u64,
        ) -> ConduitResponse {
            endpoint_type: UpdateConduits,
            method: PATCH,
            path: [EVENTSUB, CONDUITS],
            headers: [json],
            body: {
                let body = serde_json::json!({
                    "id": conduit_id,
                    "shard_count": shard_count
                });

                Some(serde_json::to_string(&body).unwrap())
            }
        }

        fn delete_conduits(&self, conduit_id: &ConduitId) -> NoContent {
            endpoint_type: DeleteConduit,
            method: DELETE,
            path: [EVENTSUB, CONDUITS],
            query_params: {
                query("id", conduit_id)
            }
        }

        fn get_conduit_shards(&self,
            conduit_id: &ConduitId,
            status: Option<Status>,
            after: Option<&str>,
        ) -> serde_json::Value {
            endpoint_type: GetConduitShards,
            method: GET,
            path: [EVENTSUB, CONDUITS, "shards"],
            query_params: {
                query("conduit_id", conduit_id),
                opt("status", status),
                opt("after", after)
            }
        }

        fn update_conduit_shards(
            &self,
            req: UpdateConduitShardsRequest,
        ) -> serde_json::Value {
            endpoint_type: UpdateConduitShards,
            method: PATCH,
            path: [EVENTSUB, CONDUITS, "shards"],
            headers: [json],
            body: req.into_json()
        }

    }
}

#[cfg(test)]
mod tests {
    use crate::{
        conduits::{
            request::{Shard, UpdateConduitShardsRequest},
            ConduitsAPI,
        },
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
            UpdateConduitShardsRequest::new(ConduitId::from(
                "bfcfc993-26b1-b876-44d9-afe75a379dac"
            ))
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
}
