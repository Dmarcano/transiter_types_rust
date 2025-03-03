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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_string() {
        let path = std::env::current_dir().unwrap();
        println!("The current directory is {}", path.display());
        let path = "./src/example_technical_problem.json";
        let contents = std::fs::read_to_string(path).unwrap();
        let _parsed: Stop = serde_json::from_str(&contents).unwrap();
    }

}
