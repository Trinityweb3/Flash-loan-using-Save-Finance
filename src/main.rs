mod types;
mod flashloan;

use types::AssetConfig;
use flashloan::execute_flash_loan;
use std::str::FromStr;
use solana_sdk::pubkey::Pubkey;

//Using Save Finance
// see https://docs.save.finance/architecture/addresses/mainnet/main-pools for more
const PROGRAM_ID_STR: &str = "So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo";
const LENDING_MARKET_STR: &str = "4UpD2fh7xH3VP9QQaXtsS1YY3bxzWhtfpks7FatyKvdY";
const VAULT_STR: &str = "DdZR6zRFiUt4S5mg7AV1uKB2z1f1WzcNYCaTEEWPAuby";

const USDC_RESERVE: &str = "BgxfHJDzm44T7XG68MYKx7YisTjZu73tVovyZSjJMpmw";
const SOL_RESERVE: &str = "8PbodeaosQP19SjYFx855UMqWxH2HynZLdBXmsrbac36";

const USDC_LIQUIDITY_SUPPLY: &str = "8SheGtsopRUDzdiD6v6BR9a6bqZ9QwywYQY99Fp5meNf";
const SOL_LIQUIDITY_SUPPLY: &str = "8UviNr47S8eL6J3WfDxMRa3hvLta1VDJwNWqsDgtN3Cv";

const USDC_LIQUIDITY_MINT: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
const SOL_LIQUIDITY_MINT: &str = "So11111111111111111111111111111111111111112";

const FEE_RECEIVER: &str = "9RuqAN42PTUi9ya59k9suGATrkqzvb9gk2QABJtQzGP5";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let rpc_url: String = std::env::var("RPC_URL")?;
    let private_key: String = std::env::var("PRIVATE_KEY")?;

    let program_id = Pubkey::from_str(PROGRAM_ID_STR)?;
    let lending_market = Pubkey::from_str(LENDING_MARKET_STR)?;
    let vault = Pubkey::from_str(VAULT_STR)?;

    
    let loan_amount: u64 = 1_000_000;
    let asset: AssetConfig = AssetConfig {
        reserve: Pubkey::from_str(USDC_RESERVE)?,
        liquidity_supply: Pubkey::from_str(USDC_LIQUIDITY_SUPPLY)?,
        liquidity_mint: Pubkey::from_str(USDC_LIQUIDITY_MINT)?,
        is_sol: false,
    };

    let fee_receiver: Pubkey = Pubkey::from_str(FEE_RECEIVER)?;

    match execute_flash_loan(
        &rpc_url, 
        &private_key, 
        loan_amount,
        program_id,
        lending_market,
        vault,
        asset,
        fee_receiver)
            .await 
    {
        Ok(_) => {},
        Err(e) => eprintln!("process crashed with error: {:?}", e),
    }
    Ok(())
   
}
