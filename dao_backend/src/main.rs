mod contract_interaction;
use web3::transports::Http;
use web3::Web3;

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();
    let infura_url = std::env::var("INFURA_URL").expect("INFURA_URL must be set");

    let http = Http::new(&infura_url)?;
    let web3 = Web3::new(http);

    println!("Connected to Ethereum network");
    Ok(())
}
