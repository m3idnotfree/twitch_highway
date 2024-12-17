#[cfg(test)]
#[macro_export]
macro_rules! expect_APIRequest {
    (
        $expect_method:ident,
        $expect_headers:expr,
        $expect_url:literal,
        $input:expr
    ) => {
        assert_eq!(
            asknothingx2_util::api::Method::$expect_method,
            $input.method()
        );
        assert_eq!($expect_headers, $input.headers());
        assert_eq!($expect_url.to_string(), $input.url().to_string());
    };
    (
        $expect_method:ident,
        $expect_headers:expr,
        $expect_url:literal,
        json = $json:expr,
        text = $text:expr,
        urlencoded = $urlencoded:expr,
        $input:expr
    ) => {
        assert_eq!(
            asknothingx2_util::api::Method::$expect_method,
            $input.method()
        );
        assert_eq!($expect_headers, $input.headers());
        assert_eq!($expect_url.to_string(), $input.url().to_string());
        assert_eq!($json, $input.json());
        assert_eq!($text, $input.text());
        assert_eq!($urlencoded, $input.json());
    };
    (json = $json:expr, $input:expr) => {
        assert_eq!($json, $input.json());
    };
}

#[cfg(test)]
#[macro_export]
macro_rules! expect_headers {
    () => {
        asknothingx2_util::api::HeaderBuilder::new()
            .authorization("Bearer", "cfabdegwdoklmawdzdo98xt2fo512y")
            .append("Client-Id", "uo6dggojyb8d6soh92zknwmi5ej1q2")
            .unwrap()
            .build()
    };
    (json) => {
        asknothingx2_util::api::HeaderBuilder::new()
            .content_type_json()
            .authorization("Bearer", "cfabdegwdoklmawdzdo98xt2fo512y")
            .append("Client-Id", "uo6dggojyb8d6soh92zknwmi5ej1q2")
            .unwrap()
            .build()
    };
}

#[cfg(test)]
#[macro_export]
macro_rules! api_general {
    ($ty:ident,$url:literal) => {
        $ty::new(
            std::sync::Arc::new(asknothingx2_util::oauth::AccessToken::new(
                "cfabdegwdoklmawdzdo98xt2fo512y".to_string(),
            )),
            std::sync::Arc::new(asknothingx2_util::oauth::ClientId::new("uo6dggojyb8d6soh92zknwmi5ej1q2".to_string())),
           std::sync::Arc::new(url::Url::parse($url).unwrap()),
        )
    };
    ($ty:ident,$url:literal,$id:literal) => {
        $ty::new(
            std::sync::Arc::new(asknothingx2_util::oauth::AccessToken::new(
                "cfabdegwdoklmawdzdo98xt2fo512y".to_string(),
            )),
            std::sync::Arc::new(asknothingx2_util::oauth::ClientId::new("uo6dggojyb8d6soh92zknwmi5ej1q2".to_string())),
            std::sync::Arc::new(url::Url::parse($url).unwrap()),
            $id,
        )
    };
    ($ty:ident,$url:literal,$($ex:literal),+) => {
        $ty::new(
            std::sync::Arc::new(asknothingx2_util::oauth::AccessToken::new(
                "cfabdegwdoklmawdzdo98xt2fo512y".to_string(),
            )),
            std::sync::Arc::new(asknothingx2_util::oauth::ClientId::new("uo6dggojyb8d6soh92zknwmi5ej1q2".to_string())),
            std::sync::Arc::new(url::Url::parse($url).unwrap()),
            $($ex),+
        )
    };
}

#[cfg(test)]
#[macro_export]
macro_rules! expect_response_json {
    ($data:literal,$de:ident) => {
        let result = serde_json::from_str::<$de>($data);

        assert!(result.is_ok());
    };
}
