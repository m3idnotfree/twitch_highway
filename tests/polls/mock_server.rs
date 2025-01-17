new_fn_mock_server_f!(
    name: get_polls,
    oauth: {
        @user,
        module: PollsScopes,
        scopes: with_polls_read
    },
    api: {
        modules: [
            twitch_highway::polls::PollsAPI
        ],
        endpoint: get_polls,
        args: |broadcaster_id|{
            broadcaster_id,
            None,
            None
        },
    }
);

new_fn_mock_server_f!(
    name: create_poll,
    oauth: {
        @user,
        module: PollsScopes,
        scopes: with_polls_create
    },
    api: {
        modules: [
            twitch_highway::polls::PollsAPI,
            twitch_highway::types::Title
        ],
        endpoint: create_poll,
        args: |broadcaster_id|{
            broadcaster_id,
            "hello",
            vec![Title::new("Heads"), Title::new("Tails")],
            15,
            None
        }
    }
);

new_fn_mock_server_f!(
    name: end_poll,
    oauth: {
        @user,
        module: PollsScopes,
        scopes: with_polls_end
    },
    api: {
        modules: [
            twitch_highway::polls::PollsAPI,
            twitch_highway::polls::types::PollStatus,
            twitch_highway::types::Id,
        ],
        endpoint: end_poll,
        args: |broadcaster_id|{
            broadcaster_id,
            Id::new("hello"),
            PollStatus::TERMINATED
        }
    }
);
