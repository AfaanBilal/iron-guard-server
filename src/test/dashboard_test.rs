/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard
 */
use super::super::rocket;
use crate::test::get_client;
use rocket::http::Status;
use serde_json::json;

#[async_test]
async fn should_return_dashboard() {
    let client = get_client().await;

    let response = client
        .get("/dashboard")
        .header(get_auth_header(false).await)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    let r = response.into_string().await;
    assert!(r.is_some());
    let r = r.unwrap();

    assert!(r.contains("user_count"));
    assert!(r.contains("category_count"));
    assert!(r.contains("item_count"));

    assert!(r.contains("users"));
    assert!(r.contains("categories"));
    assert!(r.contains("items"));
}
