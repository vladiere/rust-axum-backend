#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:3000")?;

    hc.do_get("/vlad").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "user1",
            "password": "pass1",
        }),
    );
    req_login.await?.print().await?;

    let req_create_ticket = hc.do_post(
        "/api/tickets",
        json!({
            "title": "Ticket A1",
        }),
    );

    req_create_ticket.await?.print().await?;

    // hc.do_delete("/api/tickets/1").await?.print().await;

    hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}
