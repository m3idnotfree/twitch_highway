use serde::{Deserialize, Serialize};

use crate::{
    base::{IntoQueryPairs, QueryParams},
    types::{constants::ID, Id},
};

request_struct!(
    #[derive(Serialize, Deserialize)]
    ChannelStreamScheduleRequest {
        string {
            start_time: String,
            utc_offset: String,
        },
        any {
            ids: Vec<Id>,

        }
    }
);

impl IntoQueryPairs for ChannelStreamScheduleRequest {
    fn into_query_pairs(self) -> Vec<(&'static str, String)> {
        let mut params = QueryParams::new();

        params
            .extend_opt(self.ids.map(|id| id.into_iter().map(|i| (ID, i))))
            .push_opt("start_time", self.start_time)
            .push_opt("utc_offset", self.utc_offset);

        params.build()
    }
}

request_struct!(
    #[derive( Serialize, Deserialize)]
    UpdateScheduleRequest {
        string {
            vacation_start_time: String,
            vacation_end_time: String,
            timezone: String
        },
        any {
            is_vacation_enabled: bool,
        }
    }
);
impl IntoQueryPairs for UpdateScheduleRequest {
    fn into_query_pairs(self) -> Vec<(&'static str, String)> {
        let mut params = QueryParams::new();
        params
            .push_opt(
                "is_vacation_enabled",
                self.is_vacation_enabled.map(|x| x.to_string()),
            )
            .push_opt("vacation_start_time", self.vacation_start_time)
            .push_opt("vacation_end_time", self.vacation_end_time)
            .push_opt("timezone", self.timezone);

        params.build()
    }
}

request_struct!(
    #[derive(Serialize, Deserialize)]
    CreateScheduleSegmentRequest {
        string {
            #[serde(skip_serializing_if = "Option::is_none")]
            category_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            title: String,
        },
        any {
            #[serde(skip_serializing_if = "Option::is_none")]
            is_recurring: bool,
        }
    };
    impl_body: true
);

request_struct!(
    #[derive(Serialize, Deserialize)]
    UpdateScheduleSegmentRequest {
        string {
            #[serde(skip_serializing_if = "Option::is_none")]
            start_time: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            duration: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            category_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            title: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            timezonee: String,
        },
        any {
            #[serde(skip_serializing_if = "Option::is_none")]
            is_canceled: bool,
        }
    };
    impl_body: true
);
