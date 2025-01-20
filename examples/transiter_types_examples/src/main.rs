use transiter_types_no_std::transiter_public_types::Stop;

#[derive(serde::Deserialize, Debug, Clone)]
struct ApiResponse {
    stops: Vec<Stop>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response =
        reqwest::blocking::get("https://demo.transiter.dev/systems/us-ny-subway/stops?limit=3")?;

    if !response.status().is_success() {
        return Err("request failed".into());
    }

    let api_response: ApiResponse = response.json()?;
    let stations = api_response.stops;

    println!("Found {} NYC subway stations:", stations.len());

    for stop in stations{
        println!("Name: {:#?}\nid: {:#?}\nchild stops: {:#?}\n", stop.name, stop.id, stop.child_stops );
    }
    Ok(())
}
