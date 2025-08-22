use serde::Serialize;

use crate::types::BroadcasterId;

define_request!(
    #[derive(Serialize)]
    StartCommercialBody<'a> {
        req: {
             broadcaster_id: &'a BroadcasterId,
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
        let broadcaster_id = BroadcasterId::from("123456");
        let body = StartCommercialBody::new(&broadcaster_id, 30);
        let json = body.into_json().unwrap();
        let expected = json!({
            "broadcaster_id": "123456",
            "length": 30
        });

        assert_eq!(serde_json::from_str::<Value>(&json).unwrap(), expected);
    }
}
