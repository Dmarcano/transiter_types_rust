# Transiter Rust types.

For anyone working with the [transiter API](https://github.com/jamespfennell/transiter) in Rust. This crate exposes a no-std rust types for the Transiter APIs in Rust. This is mostly to unblock development in other embedded projects that want to query a transiter server in a type safe way.

## Usage

A simple web request to an API could look like this.

```rust
use rust_transiter_types::public_api_types::Stop;

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
}
```

## Enum Serialization Support

As of now there might be some issues with using other structs that make use of enums besides Stop.

This is because prost will represent enums as an i32 and it requires custom serialization logic otherwise. I'm adding them one by one.
