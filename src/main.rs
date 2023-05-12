use dotenv::dotenv;
use rocket::tokio;



#[tokio::main]
async fn main() -> Result<(), String> {
    dotenv().ok();

    if login_service::cli_handler::handle_console_command().await? {
        return Ok(());
    }

    let rocket_handle = login_service::rocket_main::rocket_main();
    let rabbit_handle = login_service::rabbitmq_main::rabbit_main();

    let (rocket_result, rabbit_result) = tokio::join!(rocket_handle, rabbit_handle);

    rocket_result.map_err(|err| -> String {err.to_string()})?;
    rabbit_result?;

    Ok(())
}