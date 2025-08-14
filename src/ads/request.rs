use serde::Serialize;

use crate::types::BroadcasterId;

define_request!(
    #[derive(Serialize)]
    StartCommercialBody {
        req: {
             broadcaster_id: BroadcasterId,
             length: u64 ; u64
        };
        into_json
    }
);

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    use crate::{ads::request::StartCommercialBody, types::BroadcasterId};

    #[test]
    fn start_commercial_body_serialization() {
        let body = StartCommercialBody::new(BroadcasterId::new("123456"), 30);
        let json = body.into_json().unwrap();
        let expected = json!({
            "broadcaster_id": "123456",
            "length": 30
        });

        assert_eq!(serde_json::from_str::<Value>(&json).unwrap(), expected);
    }
}
