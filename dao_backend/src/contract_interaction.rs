use std::process::Output;

use web3::contract::{Contract, Options};
use web3::types::{Address, U256};

async fn interact_with_contract() -> web3::Result<()> {
    let contract_address = "your_contract_address_here".parse::<Address>().unwrap();
    let contract = Contract::from_json(
        web3.eth(),
        contract_address,
        include_bytes!("path/to/your/contract_abi.json"),
    ).unwrap();

    
    let result: U256 = contract.query("totalProposals", (), None, Options::default(), None).await?;
    println!("Total proposals: {}", result);

    Ok(())
}
