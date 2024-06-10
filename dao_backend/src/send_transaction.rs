use web3::{
    types::{Address, U256},
    contract::{Contract, Options},
    ethabi::Token,
    transports::Http,
    Web3,
    accounts::Account,
    
};
use dotenv::dotenv;
use std::{env, str::FromStr};

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv().ok(); 

    
    let http = Http::new(&env::var("INFURA_URL").expect("INFURA_URL must be set"))?;
    let web3 = Web3::new(http);

   
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
    let account = Account::from_secret_key(&web3, private_key.parse().unwrap());

    
    let contract_address = env::var("GOVERNANCE_CONTRACT_ADDRESS")
        .expect("GOVERNANCE_CONTRACT_ADDRESS must be set")
        .parse::<Address>()
        .expect("Invalid contract address");
    let abi = include_bytes!("../path_to_ABI/GovernanceABI.json");

    
    let contract = Contract::from_json(web3.eth(), contract_address, abi)?;

    
    let proposal_id = U256::from(1); 

    
    let tx = contract.call("vote", (proposal_id,), account.address(), Options::default()).await?;

    println!("Transaction sent: {}", tx);

    Ok(())
}
