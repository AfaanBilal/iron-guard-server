/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard
 */
use super::{success, ErrorResponder};

#[post("/sign-in")]
pub async fn sign_in() -> Result<String, ErrorResponder> {
    success()
}

#[post("/sign-out")]
pub async fn sign_out() -> Result<String, ErrorResponder> {
    success()
}
