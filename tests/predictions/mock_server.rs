new_fn_mock_server_f!(
    name: get_predictions,
    oauth: {
        @user,
        module: PredictionsScopes,
        scopes: with_predictions_read
    },
    api: {
        modules: [
            twitch_highway::predictions::PredictionsAPI,
            twitch_highway::types::Id
        ],
        endpoint: get_predictions,
        args:|broadcaster_id|{
            broadcaster_id,
                Some(vec![Id::new("11265581")]), None
        }
    }
);

new_fn_mock_server_f!(
    name: create_prediction,
    oauth: {
        @user,
        module: PredictionsScopes,
        scopes: with_predictions_create
    },
    api: {
        modules: [
            twitch_highway::predictions::PredictionsAPI,
            twitch_highway::types::Title
        ],
        endpoint: create_prediction,
        args: |broadcaster_id|{
            broadcaster_id,
            "Any leeks in the stream?",
            vec![
                Title::new("Yes, give it time."),
                Title::new("Definitely not.")
            ],
            120
        }
    }
);

new_fn_mock_server_f!(
    name: end_prediction,
    oauth: {
        @user,
        module: PredictionsScopes,
        scopes: with_predictions_end
    },
    api: {
        modules: [
            twitch_highway::predictions::PredictionsAPI,
            twitch_highway::predictions::types::PredictionStatus
        ],
        endpoint: end_prediction,
        args: |broadcaster_id|{
            broadcaster_id.clone(),
            broadcaster_id.to_id(),
            PredictionStatus::CANCELED,
            None
        }
    }
);
