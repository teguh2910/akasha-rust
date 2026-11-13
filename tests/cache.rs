//! Regression tests for the response cache.

use akasha::{Akasha, Language};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn cached_responses_are_unwrapped() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/getCalculationsForUser/901211014"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
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
        })))
        .expect(1)
        .mount(&server)
        .await;

    let api = Akasha::builder()
        .language(Language::English)
        .base_url(format!("{}/api", server.uri()))
        .build()
        .unwrap();

    for _ in 0..2 {
        let calculations = api.get_calculations_for_user(901211014).await.unwrap();
        assert_eq!(calculations.len(), 1);
        assert_eq!(calculations[0].name, "Kamisato Ayaka");
    }
}
