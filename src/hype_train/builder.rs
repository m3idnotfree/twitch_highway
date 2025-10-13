use crate::{
    hype_train::HypeTrainResponse,
    types::{
        constants::{AFTER, EVENTS, FIRST, HYPE_TRAIN},
        BroadcasterId,
    },
};

define_request_builder! {
    #[derive(Debug)]
    GetHypeTrainEventsBuilder<'a> {
        req: {broadcaster_id: &'a BroadcasterId},
        opts: {
            first: u8 [key = FIRST, convert = to_string],
            after: &'a str [key = AFTER],
        }
    } -> HypeTrainResponse;
    endpoint_type: GetHypeTrainEvents,
    method: GET,
    path: [HYPE_TRAIN, EVENTS],
}
