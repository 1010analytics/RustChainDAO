use web3::types::Address;
use web3::contract::{Contract, Options};
use web3::transports::Http;
use web3::Web3;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv().ok(); 
    let web3 = Web3::new(Http::new(&env::var("INFURA_URL").expect("INFURA_URL must be set"))?);

    
    let contract_address = env::var("GOVERNANCE_CONTRACT_ADDRESS")
        .expect("GOVERNANCE_CONTRACT_ADDRESS must be set")
        .parse::<Address>()
        .expect("Invalid contract address");
    let abi = include_bytes!("../path_to_ABI/GovernanceABI.json");

    
    let contract = Contract::from_json(web3.eth(), contract_address, abi)?;

    
    let result: u64 = contract.query("totalProposals", (), None, Options::default(), None).await?;

    
    println!("Total proposals: {}", result);

    Ok(())
}
