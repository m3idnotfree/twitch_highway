use crate::{
    charity::CharityCampaignDonationResponse,
    types::{
        constants::{AFTER, BROADCASTER_ID, CHARITY, DONATIONS, FIRST},
        BroadcasterId,
    },
    Client, Error,
};

#[derive(Debug)]
pub struct GetCharityCampaignDonationBuilder<'a> {
    client: &'a Client,
    broadcaster_id: &'a BroadcasterId,
    first: Option<u8>,
    after: Option<&'a str>,
}

impl<'a> GetCharityCampaignDonationBuilder<'a> {
    pub fn new(client: &'a Client, broadcaster_id: &'a BroadcasterId) -> Self {
        Self {
            client,
            broadcaster_id,
            first: None,
            after: None,
        }
    }

    pub fn first(mut self, value: u8) -> Self {
        self.first = Some(value);
        self
    }

    pub fn after(mut self, value: &'a str) -> Self {
        self.after = Some(value);
        self
    }

    pub async fn send(self) -> Result<CharityCampaignDonationResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([CHARITY, DONATIONS]);

        url.query_pairs_mut()
            .append_pair(BROADCASTER_ID, self.broadcaster_id);
        if let Some(val) = self.first {
            url.query_pairs_mut().append_pair(FIRST, &val.to_string());
        }
        if let Some(val) = self.after {
            url.query_pairs_mut().append_pair(AFTER, val);
        }

        let req = self.client.http_client().get(url);
        self.client.json(req).await
    }
}
