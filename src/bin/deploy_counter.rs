use ethers::{
    contract::ContractFactory,
    middleware::SignerMiddleware,
    prelude::{LocalWallet, Middleware, Provider, Signer},
    utils::{Anvil, AnvilInstance},
};
use ethers_solc::{
    Artifact, ConfigurableArtifacts, Project, ProjectCompileOutput, ProjectPathsConfig,
};
use eyre::{eyre, ContextCompat, Result};
use hex::ToHex;
use std::{path::PathBuf, time::Duration};

#[tokio::main]
async fn main() -> Result<()> {
    let mnemonic = "gas monster ski craft below illegal discover limit dog bundle bus artefact";
    let anvil: AnvilInstance = Anvil::new().mnemonic(mnemonic).spawn();

    println!("HTTP Endpoint: {}", anvil.endpoint());

    let wallet: LocalWallet = anvil.keys()[0].clone().into();
    let first_address = wallet.address();

    println!("Wallet Address: {}", first_address.encode_hex::<String>());

    let provider = Provider::try_from(anvil.endpoint())?.interval(Duration::from_millis(10));
    let chain_id = provider.get_chainid().await?.as_u64();

    println!("Chain ID: {}", chain_id);

    let project = compile("./", "contracts")?;

    let cwd = std::env::current_dir()?;

    let contract_path = cwd.join("contracts/Counter.sol");
    let contract_name = "Counter";
    let contract = project
        .find(contract_path.to_str().unwrap(), contract_name)
        .context("Contract not found")?
        .clone();

    let (abi, bytecode, _) = contract.into_parts();
    let abi = abi.context("ABI not found")?;
    let bytecode = bytecode.context("Bytecode not found")?;

    let wallet = wallet.with_chain_id(chain_id);
    let client = SignerMiddleware::new(provider.clone(), wallet).into();

    let factory = ContractFactory::new(abi, bytecode, client);
    let mut deployer = factory.deploy(())?;

    let gas_price = provider.get_gas_price().await?;
    println!("Gas Price: {}", gas_price);

    deployer.tx.set_gas_price(gas_price);

    println!("Tx: {:?}", deployer.tx);

    let deployed_contract = deployer.legacy().send().await?;

    println!(
        "Deployed Contract: {}",
        deployed_contract.address().encode_hex::<String>()
    );

    Ok(())
}

fn compile(root: &str, sources: &str) -> Result<ProjectCompileOutput<ConfigurableArtifacts>> {
    let root = PathBuf::from(root);
    if !root.exists() {
        return Err(eyre!("Project root does not exist: {:?}", root));
    }

    let sources = root.join(sources);

    let paths = ProjectPathsConfig::builder()
        .root(&root)
        .sources(&sources)
        .build()?;

    let project = Project::builder()
        .paths(paths)
        .set_auto_detect(true)
        .build()?;

    let output = project.compile()?;

    if output.has_compiler_errors() {
        Err(eyre!("Compiler errors: {:?}", output.output().errors))
    } else {
        Ok(output)
    }
}
