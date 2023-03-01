/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard-server
 */
use super::super::rocket;
use crate::test::{get_client, utils::get_auth_header};
use rocket::http::Status;

#[async_test]
async fn should_return_inventory() {
    let client = get_client().await;

    let response = client
        .get("/inventory")
        .header(get_auth_header(false).await)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    let r = response.into_string().await;
    assert!(r.is_some());
    let r = r.unwrap();

    assert!(r.contains("categories"));
    assert!(r.contains("items"));
}
