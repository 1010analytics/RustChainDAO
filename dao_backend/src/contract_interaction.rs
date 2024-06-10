use web3::{
    types::{Address, U256, Bytes},
    contract::{Contract, Options},
    accounts::Account,
    transports::Http,
    Web3,
    signing::{Key, SecretKey},
    error::Error as Web3Error,
};
use std::{env, error::Error, str::FromStr, result::Result as StdResult};

#[tokio::main]
async fn main() -> StdResult<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let infura_url = env::var("INFURA_URL")?;
    let http = Http::new(&infura_url)?;
    let web3 = Web3::new(http);
    let private_key = env::var("PRIVATE_KEY")?.parse::<SecretKey>()?;
    let account = Account::from_secret_key(&web3, private_key);

    let contract_address = env::var("CONTRACT_ADDRESS")?.parse::<Address>()?;
    let abi = include_bytes!("../path_to_your_ABI/GovernanceABI.json");
    let contract = Contract::from_json(web3.eth(), contract_address, abi)?;

    let proposal_description = "Add new community garden project";
    let tx_result = create_proposal(&contract, &account, proposal_description).await;

    match tx_result {
        Ok(tx_hash) => println!("Transaction successful with hash: {:?}", tx_hash),
        Err(e) => println!("Transaction failed: {:?}", e),
    }

    Ok(())
}

async fn create_proposal(contract: &Contract<Http>, account: &Account<SecretKey>, description: &str) -> StdResult<Bytes, Web3Error> {
    let options = Options::with(|opt| {
        opt.gas = Some(300_000.into());
        opt.nonce = None;
    });
    contract.call("createProposal", (description,), account.address(), options).await
}
