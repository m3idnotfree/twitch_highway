fn_expected_request!(
    api: twitch_highway::ccls::CclsAPI,
    endpoint: get_content_classification_labels,
    token_type: Any,
    scopes: None,
    args: [None],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/content_classification_labels"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"id\": \"DebatedSocialIssuesAndPolitics\",\n      \"description\": \"Discussions or debates about politics or sensitive social issues such as elections, civic integrity, military conflict, and civil rights in a polarizing manner.\",\n      \"name\": \"Politics and Sensitive Social Issues\"\n    },\n    {\n      \"id\": \"DrugsIntoxication\",\n      \"description\": \"Excessive tobacco glorification or promotion, any marijuana consumption/use, legal drug and alcohol induced intoxication, discussions of illegal drugs.\",\n      \"name\": \"Drugs, Intoxication, or Excessive Tobacco Use\"\n    },\n    {\n      \"id\": \"Gambling\",\n      \"description\": \"Participating in online or in-person gambling, poker or fantasy sports, that involve the exchange of real money.\",\n      \"name\": \"Gambling\"\n    },\n    {\n      \"id\": \"MatureGame\",\n      \"description\": \"Games that are rated Mature or less suitable for a younger audience.\",\n      \"name\": \"Mature-rated game\"\n    },\n    {\n      \"id\": \"ProfanityVulgarity\",\n      \"description\": \"Prolonged, and repeated use of obscenities, profanities, and vulgarities, especially as a regular part of speech.\",\n      \"name\": \"Significant Profanity or Vulgarity\"\n    },\n    {\n      \"id\": \"SexualThemes\",\n      \"description\": \"Content that focuses on sexualized physical attributes and activities, sexual topics, or experiences.\",\n      \"name\": \"Sexual Themes\"\n    },\n    {\n      \"id\": \"ViolentGraphic\",\n      \"description\": \"Simulations and/or depictions of realistic violence, gore, extreme injury, or death.\",\n      \"name\": \"Violent and Graphic Depictions\"\n    }\n  ]\n}",
    module: twitch_highway::ccls::response::CclsResponse,
    de: CclsResponse
);
