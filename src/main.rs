// Stoped at #5 of https://git.sr.ht/~gruberb/onetutorial
// Will use clap to solve that


use std::collections::HashMap;
use serde::Deserialize;
extern crate dotenv;
use std::env;
use std::fmt;

#[derive(Debug, Deserialize)]
struct CMCResponse{
    data: HashMap<String, Currency>
}

#[derive(Debug, Deserialize)]
struct Currency{
    name: String,
    symbol: String,
    quote: HashMap<String, Quote>
}


#[derive(Debug, Deserialize)]
struct Quote {
    price: f64,
    percent_change_7d: f64,
}


#[derive(Debug)]
struct Crypto{
    name: String,
    symbol: String,
    price: f64,
    percentage_7d: f64
}

impl fmt::Display for Crypto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {}, Symbol: {}, Price: {}, Change(7d): {}.",
            self.name,
            self.symbol,
            self.price.to_string(),
            self.percentage_7d.to_string()
        )
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    let mut params = HashMap::new();
    params.insert("symbol", "BTC");
    dotenv::dotenv().expect("Failed to read .env file");
    let client = reqwest::Client::new();
    let content = client.get("https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest")
        .header("X-CMC_PRO_API_KEY",env::var("CMC_PRO_API_KEY")?)
        .query(&params)
        .send()
        .await?
        .json::<CMCResponse>()
        .await?;
    let BTC = content.data.get("BTC").expect("BTC not found");
    let result = Crypto {
        name: BTC.name.clone(),
        symbol: BTC.symbol.clone(),
        price: BTC.quote.get("USD").expect("USD not found").price,
        percentage_7d: BTC.quote.get("USD").expect("USD not found").percent_change_7d
    };
    println!("{}", result);
    Ok(())
}
