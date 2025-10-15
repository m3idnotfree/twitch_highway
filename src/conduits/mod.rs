mod builder;
mod response;
mod types;

pub use builder::{
    GetConduitShardsBuilder, ShardUpdate, TransportConfig, UpdateConduitShardsBuilder,
};
pub use response::{
    ConduitResponse, GetConduitShardsResponse, Pagination, UpdateConduitShardsResponse,
    UpdateShardError,
};
pub use types::{Conduit, ConduitShard, Transport};

use types::{CreateConduitsBody, UpdateConduitsBody};

use crate::{
    request::{NoContent, TwitchAPIRequest},
    types::{
        constants::{CONDUITS, EVENTSUB, ID},
        ConduitId,
    },
    TwitchAPI,
};

pub trait ConduitsAPI {
    /// Gets the conduits for a client ID
    ///
    /// # Returns
    ///
    /// Returns a [`ConduitResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::conduits::ConduitsAPI;
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_conduits()
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// No scope required
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-conduits>
    fn get_conduits(&self) -> TwitchAPIRequest<ConduitResponse>;

    /// Creates a new conduit
    ///
    /// # Arguments
    ///
    /// * `shard_count` - The number of shards to create for this conduit.
    ///
    /// # Returns
    ///
    /// Returns a [`ConduitResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::conduits::ConduitsAPI;
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .create_conduits(5)
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// App access token
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#create-conduits>
    fn create_conduits(&self, shard_count: u64) -> TwitchAPIRequest<ConduitResponse>;

    /// Updates a conduit’s shard count
    ///
    /// # Arguments
    ///
    /// * `conduit_id` -
    /// * `shard_count` - The new number of shards for this conduit.
    ///
    /// # Returns
    ///
    /// Returns a [`ConduitResponse`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     conduits::ConduitsAPI,
    ///     types::ConduitId
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let conduit_id = ConduitId::from("1234");
    /// let response = api
    ///     .update_conduits(&conduit_id, 5)
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// App access token
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#update-conduits>
    fn update_conduits(
        &self,
        conduit_id: &ConduitId,
        shard_count: u64,
    ) -> TwitchAPIRequest<ConduitResponse>;

    /// Deletes a specified conduit
    ///
    /// # Arguments
    ///
    /// * `conduit_id` -
    ///
    /// # Returns
    ///
    /// Returns a [`NoContent`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     conduits::ConduitsAPI,
    ///     types::ConduitId
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .delete_conduits(&ConduitId::from("1234"))
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// App access token
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#delete-conduit>
    fn delete_conduits(&self, conduit_id: &ConduitId) -> TwitchAPIRequest<NoContent>;

    /// Gets a lists of all shards for a conduit
    ///
    /// # Arguments
    ///
    /// * `conduit_id` -
    ///
    /// # Returns
    ///
    /// Returns a [`GetConduitShardsBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     conduits::ConduitsAPI,
    ///     types::{ConduitId, Status},
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .get_conduit_shards(&ConduitId::from("1234"))
    ///     .status(Status::Enabled)
    ///     .after("eyJiI...")
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// App access token
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#get-conduit-shards>
    fn get_conduit_shards<'a>(&'a self, conduit_id: &'a ConduitId) -> GetConduitShardsBuilder<'a>;

    /// Updates shard(s) for a conduit
    ///
    /// # Arguments
    ///
    /// * `conduit_id` -
    ///
    /// # Returns
    ///
    /// Returns a [`UpdateConduitShardsBuilder`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::TwitchAPI;
    /// use twitch_highway::{
    ///     conduits::ConduitsAPI,
    ///     types::{ConduitId, SessionId, ShardId},
    /// };
    ///
    /// # async fn example(api: TwitchAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let response = api
    ///     .update_conduit_shards(ConduitId::from("1234"))
    ///     .webhook(ShardId::from("1234"), "callback", "secret")
    ///     .websocket(ShardId::from("5678"), SessionId::from("7890"))
    ///     .json()
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Required Scope
    ///
    /// App access token
    ///
    /// API Reference
    ///
    /// <https://dev.twitch.tv/docs/api/reference/#update-conduit-shards>
    fn update_conduit_shards<'a>(&'a self, conduit_id: ConduitId)
        -> UpdateConduitShardsBuilder<'a>;
}

impl ConduitsAPI for TwitchAPI {
    fn get_conduits(&self) -> TwitchAPIRequest<ConduitResponse> {
        let mut url = self.build_url();

        url.path_segments_mut()
            .unwrap()
            .extend(&[EVENTSUB, CONDUITS]);

        TwitchAPIRequest::new(
            crate::request::EndpointType::GetConduits,
            url,
            reqwest::Method::GET,
            self.default_headers(),
            None,
            self.client.clone(),
        )
    }
    fn create_conduits(&self, shard_count: u64) -> TwitchAPIRequest<ConduitResponse> {
        let mut url = self.build_url();

        url.path_segments_mut()
            .unwrap()
            .extend(&[EVENTSUB, CONDUITS]);

        let body = serde_json::to_string(&CreateConduitsBody { shard_count }).ok();

        TwitchAPIRequest::new(
            crate::request::EndpointType::CreateConduits,
            url,
            reqwest::Method::POST,
            self.header_json(),
            body,
            self.client.clone(),
        )
    }
    fn update_conduits(
        &self,
        conduit_id: &ConduitId,
        shard_count: u64,
    ) -> TwitchAPIRequest<ConduitResponse> {
        let mut url = self.build_url();

        url.path_segments_mut()
            .unwrap()
            .extend(&[EVENTSUB, CONDUITS]);

        let body = serde_json::to_string(&UpdateConduitsBody {
            conduit_id,
            shard_count,
        })
        .ok();

        TwitchAPIRequest::new(
            crate::request::EndpointType::UpdateConduits,
            url,
            reqwest::Method::PATCH,
            self.header_json(),
            body,
            self.client.clone(),
        )
    }
    fn delete_conduits(&self, conduit_id: &ConduitId) -> TwitchAPIRequest<NoContent> {
        let mut url = self.build_url();

        url.path_segments_mut()
            .unwrap()
            .extend(&[EVENTSUB, CONDUITS]);

        let mut query = url.query_pairs_mut();

        query.append_pair(ID, conduit_id);

        drop(query);

        TwitchAPIRequest::new(
            crate::request::EndpointType::DeleteConduit,
            url,
            reqwest::Method::DELETE,
            self.default_headers(),
            None,
            self.client.clone(),
        )
    }
    fn get_conduit_shards<'a>(&'a self, conduit_id: &'a ConduitId) -> GetConduitShardsBuilder<'a> {
        GetConduitShardsBuilder::new(self, conduit_id)
    }

    fn update_conduit_shards<'a>(
        &'a self,
        conduit_id: ConduitId,
    ) -> UpdateConduitShardsBuilder<'a> {
        UpdateConduitShardsBuilder::new(self, conduit_id)
    }
}
