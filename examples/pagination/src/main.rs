#[tokio::main]
async fn main() -> Result<(), payjp::PayjpError> {
    use futures_util::TryStreamExt;
    use payjp::Client;
    use payjp_core::customer::ListCustomer;

    let secret_key = std::env::var("SECRET_KEY").expect("Missing SECRET_KEY in env");
    let client = Client::new(secret_key);
    let paginator = ListCustomer::new().paginate();
    let mut stream = paginator.stream(&client);

    // take a value out from the stream
    if let Some(val) = stream.try_next().await? {
        println!("GOT = {:?}", val);
    }
    if let Some(val) = stream.try_next().await? {
        println!("GOT = {:?}", val);
    }
    // 3つめないのに取得しようとすると異常終了 > pay
    let _ = stream.try_collect::<Vec<_>>().await?;
    Ok(())
}
