use rust_transiter_types::public_api_types::Stop;
use serde_json;

#[derive(serde::Deserialize, Debug, Clone)]
struct ApiResponse {
    stops: Vec<Stop>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response =
        reqwest::blocking::get("https://demo.transiter.dev/systems/us-ny-subway/stops?limit=1")?;

    if !response.status().is_success() {
        return Err("request failed".into());
    }

    // let api_response: ApiResponse = response.json()?;
    let text = response.text()?;
    print!("{}", text);

    let api_response: ApiResponse = serde_json::from_str(&text)?;
    let stations = api_response.stops;
    print!("{:?}", stations);

    // println!("Found {} NYC subway stations:", stations.len());

    // for stop in stations{
    //     println!("Name: {:#?}\nid: {:#?}\nchild stops: {:#?}\n", stop.name, stop.id, stop.stop_times );
    // }
    Ok(())
}
