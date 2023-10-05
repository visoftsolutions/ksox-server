use axum::{body::Body, http::Request};
use chrono::Utc;
use futures::executor::block_on;
use hyper::StatusCode;
use proptest::prelude::*;
use seq_macro::seq;
use tower::ServiceExt;

use crate::{
    app::get_app,
    jwt::{Claims, JwtEncodeDecode},
};

seq!(N in 0..15 {
proptest! {
    #[test]
    fn test_me_endpoint~N(s in "[a-zA-Z0-9]{256}") {
        let app = get_app();
        let jwt = Claims {
            sub: s.to_owned(),
            exp: usize::try_from(Utc::now().timestamp()).unwrap() + 60,
        };
        let request = Request::builder()
            .method("GET")
            .uri("/me")
            .header("Authorization", format!("Bearer {}", jwt.encode().unwrap()))
            .body(Body::empty())
            .unwrap();

        let resp = block_on(async { app.oneshot(request).await.unwrap() });
        let (parts, body) = resp.into_parts();
        let bytes = block_on(hyper::body::to_bytes(body)).unwrap();
        let body_str = String::from_utf8(bytes.to_vec()).expect("Response body is not a valid UTF-8 string");

        assert_eq!(parts.status, StatusCode::OK);
        assert_eq!(body_str, s);
    }
}
});
