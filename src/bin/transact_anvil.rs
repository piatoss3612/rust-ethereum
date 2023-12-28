use ethers::{
    prelude::{Address, LocalWallet, Middleware, Provider, Signer, TransactionRequest, U256},
    utils::{Anvil, AnvilInstance},
};
use eyre::{ContextCompat, Result};
use hex::ToHex;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let mnemonic = "gas monster ski craft below illegal discover limit dog bundle bus artefact";
    let anvil: AnvilInstance = Anvil::new().mnemonic(mnemonic).spawn();
    println!("HTTP Endpoint: {}", anvil.endpoint());

    let wallet: LocalWallet = anvil.keys()[0].clone().into();
    let first_address = wallet.address();
    println!("Wallet Address: {}", first_address.encode_hex::<String>());

    let provider = Provider::try_from(anvil.endpoint())?.interval(Duration::from_millis(10));

    let first_balance = provider.get_balance(first_address, None).await?;

    println!("First Address Balance: {}", first_balance);

    let other_address_hex = "0xaf206dCE72A0ef76643dfeDa34DB764E2126E646";
    let other_address = other_address_hex.parse::<Address>()?;

    let other_balance = provider.get_balance(other_address, None).await?;

    println!("Other Address Balance: {}", other_balance);

    let tx = TransactionRequest::pay(other_address, U256::from(1000u64)).from(first_address);

    let receipt = provider
        .send_transaction(tx, None)
        .await?
        .log_msg("Pending Transaction Sent")
        .confirmations(1)
        .await?
        .context("Transaction Failed")?;

    println!(
        "Tx mined at block {}",
        receipt.block_number.context("Could not get block number")?
    );
    println!(
        "Balance of {}: {}",
        other_address_hex,
        provider.get_balance(other_address, None).await?
    );

    Ok(())
}
