use axum::{routing, Router};
use chrono::Utc;
use futures::executor::block_on;
use hyper::{Body, Request, StatusCode};
use proptest::prelude::*;
use tower::ServiceExt;

use crate::auth::jwt::traits::JwtEncodeDecode;

use super::models::Claims;

const TEST_ROUTE: &str = "/";

pub fn app() -> Router {
    Router::new().route(TEST_ROUTE, routing::get(get))
}

pub async fn get(claims: Claims) -> String {
    claims.sub
}

proptest! {
    #[test]
    fn test_jwt(s in "[a-zA-Z0-9]{256}") {
        let app = app();
        let jwt = Claims {
            sub: s.to_owned(),
            exp: usize::try_from(Utc::now().timestamp()).unwrap() + 60,
        };
        let request = Request::builder()
            .method("GET")
            .uri(TEST_ROUTE)
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
