use rust_transiter_types::public_api_types::Stop;
use serde_json;

#[derive(serde::Deserialize, Debug, Clone)]
struct ApiResponse {
    stops: Vec<Stop>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response =
        reqwest::blocking::get("https://demo.transiter.dev/systems/us-ny-subway/stops/R09")?;

    if !response.status().is_success() {
        return Err("request failed".into());
    }

    let text = response.text()?;

    let api_response: Stop = serde_json::from_str(&text)?;
    print!("{:#?}", api_response);

  
    Ok(())
}
