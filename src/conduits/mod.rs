mod builder;
mod response;
mod types;

pub use builder::{GetConduitShards, ShardUpdate, TransportConfig, UpdateConduitShards};
pub use response::{
    ConduitResponse, GetConduitShardsResponse, UpdateConduitShardsResponse, UpdateShardError,
};
pub use types::{Conduit, ConduitShard, Transport};

use types::{CreateConduitsBody, UpdateConduitsBody};

use std::future::Future;

use crate::{
    types::{
        constants::{CONDUITS, EVENTSUB, ID},
        ConduitId,
    },
    Client, Error,
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
    /// # use twitch_highway::Client;
    /// use twitch_highway::conduits::ConduitsAPI;
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_conduits()
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
    fn get_conduits(&self) -> impl Future<Output = Result<ConduitResponse, Error>> + Send;

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
    /// # use twitch_highway::Client;
    /// use twitch_highway::conduits::ConduitsAPI;
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .create_conduits(5)
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
    fn create_conduits(
        &self,
        shard_count: u64,
    ) -> impl Future<Output = Result<ConduitResponse, Error>> + Send;

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
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     conduits::ConduitsAPI,
    ///     types::ConduitId
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let conduit_id = ConduitId::from("1234");
    /// let response = api
    ///     .update_conduits(&conduit_id, 5)
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
    ) -> impl Future<Output = Result<ConduitResponse, Error>> + Send;

    /// Deletes a specified conduit
    ///
    /// # Arguments
    ///
    /// * `conduit_id` -
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     conduits::ConduitsAPI,
    ///     types::ConduitId
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .delete_conduits(&ConduitId::from("1234"))
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
    fn delete_conduits(
        &self,
        conduit_id: &ConduitId,
    ) -> impl Future<Output = Result<(), Error>> + Send;

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
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     conduits::ConduitsAPI,
    ///     types::{ConduitId, Status},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .get_conduit_shards(&ConduitId::from("1234"))
    ///     .status(Status::Enabled)
    ///     .after("eyJiI...")
    ///     .send()
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
    fn get_conduit_shards<'a>(&'a self, conduit_id: &'a ConduitId) -> GetConduitShards<'a>;

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
    /// # use twitch_highway::Client;
    /// use twitch_highway::{
    ///     conduits::ConduitsAPI,
    ///     types::{ConduitId, SessionId, ShardId},
    /// };
    ///
    /// # async fn example(api: Client) -> Result<(), twitch_highway::Error> {
    /// let response = api
    ///     .update_conduit_shards(ConduitId::from("1234"))
    ///     .webhook(ShardId::from("1234"), "callback", "secret")
    ///     .websocket(ShardId::from("5678"), SessionId::from("7890"))
    ///     .send()
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
    fn update_conduit_shards<'a>(&'a self, conduit_id: ConduitId) -> UpdateConduitShards<'a>;
}

impl ConduitsAPI for Client {
    async fn get_conduits(&self) -> Result<ConduitResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([EVENTSUB, CONDUITS]);

        self.json(self.http_client().get(url)).await
    }

    async fn create_conduits(&self, shard_count: u64) -> Result<ConduitResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([EVENTSUB, CONDUITS]);

        let req = self
            .http_client()
            .post(url)
            .json(&CreateConduitsBody { shard_count });
        self.json(req).await
    }

    async fn update_conduits(
        &self,
        conduit_id: &ConduitId,
        shard_count: u64,
    ) -> Result<ConduitResponse, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([EVENTSUB, CONDUITS]);

        let req = self.http_client().patch(url).json(&UpdateConduitsBody {
            conduit_id,
            shard_count,
        });
        self.json(req).await
    }

    async fn delete_conduits(&self, conduit_id: &ConduitId) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([EVENTSUB, CONDUITS]);

        url.query_pairs_mut().append_pair(ID, conduit_id);

        self.no_content(self.http_client().delete(url)).await
    }

    fn get_conduit_shards<'a>(&'a self, conduit_id: &'a ConduitId) -> GetConduitShards<'a> {
        GetConduitShards::new(self, conduit_id)
    }

    fn update_conduit_shards<'a>(&'a self, conduit_id: ConduitId) -> UpdateConduitShards<'a> {
        UpdateConduitShards::new(self, conduit_id)
    }
}
