//! HTTP layer tests against a mock upstream.

use std::time::Duration;

use akasha_service::{routes, AppState, Config};
use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn state(base_url: String) -> AppState {
    AppState::new(Config {
        port: 0,
        base_url,
        cache_ttl: Duration::from_secs(60),
    })
    .unwrap()
}

fn user_calc_response() -> serde_json::Value {
    serde_json::json!({
        "ttl": 0,
        "data": [{
            "_id": "x",
            "characterId": 10000002,
            "type": "current",
            "uid": "901211014",
            "constellation": 0,
            "md5": "abc",
            "name": "Kamisato Ayaka",
            "icon": "UI_AvatarIcon_Ayaka"
        }]
    })
}

#[tokio::test]
async fn health_is_ok() {
    let app = routes::router(state("http://127.0.0.1:1/api".to_owned()));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn characters_are_proxied() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/getCalculationsForUser/901211014"))
        .respond_with(ResponseTemplate::new(200).set_body_json(user_calc_response()))
        .mount(&server)
        .await;

    let app = routes::router(state(format!("{}/api", server.uri())));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/v1/users/901211014/characters")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["data"][0]["name"], "Kamisato Ayaka");
}

#[tokio::test]
async fn invalid_size_is_reported_as_json() {
    let app = routes::router(state("http://127.0.0.1:1/api".to_owned()));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/v1/leaderboards/1?size=0")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["error"], "bad_request");
}
