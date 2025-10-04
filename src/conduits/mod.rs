mod request;
mod response;
mod types;

pub use request::{Shard, Transport, UpdateConduitShardsRequest};
pub use response::ConduitResponse;
pub use types::Conduit;

use crate::{
    request::NoContent,
    types::{constants::EVENTSUB, ConduitId, Status},
};

const CONDUITS: &str = "conduits";

endpoints! {
    ConduitsAPI {
        /// <https://dev.twitch.tv/docs/api/reference/#get-conduits>
        fn get_conduits (&self) -> ConduitResponse {
            endpoint_type: GetConduits,
            method: GET,
            path: [EVENTSUB, CONDUITS],
        }

        /// <https://dev.twitch.tv/docs/api/reference/#create-conduits>
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

        /// <https://dev.twitch.tv/docs/api/reference/#update-conduits>
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

        /// <https://dev.twitch.tv/docs/api/reference/#delete-conduit>
        fn delete_conduits(&self, conduit_id: &ConduitId) -> NoContent {
            endpoint_type: DeleteConduit,
            method: DELETE,
            path: [EVENTSUB, CONDUITS],
            query_params: {
                query("id", conduit_id)
            }
        }

        /// <https://dev.twitch.tv/docs/api/reference/#get-conduit-shards>
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

        /// <https://dev.twitch.tv/docs/api/reference/#update-conduit-shards>
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
