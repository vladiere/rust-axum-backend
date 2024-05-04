use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:4000")?;

    hc.do_get("/hello/Vlad").await?.print().await?;

    Ok(())
}
