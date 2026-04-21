# `watwowmap-pogo-data`

Before this package, a Rust consumer had to build raw URLs and deserialize JSON manually:

```rust
let response = reqwest::get("https://raw.githubusercontent.com/WatWowMap/pogo-data-api/refs/heads/main/data/v1/pokemon/1.json").await?;
let bulbasaur: serde_json::Value = response.json().await?;
```

After adding the crate, the same lookup is a typed async method call:

```rust
let bulbasaur = pogo_data::PogoDataClient::builder()
    .build()
    .pokemon()
    .get(1)
    .await?;
```

## Install

```bash
cargo add watwowmap-pogo-data
```

## Usage

```rust
use pogo_data::{configure_pogo_data, pogo_data, PogoDataClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let shared_client = pogo_data();
    let shared_pokemon = shared_client.pokemon().get(1).await?;
    println!("{}", shared_pokemon.pokemon_name);

    configure_pogo_data(
        PogoDataClient::builder()
            .base_url("https://cdn.example.com/pogo/data/v1"),
    );

    let mirrored_client = pogo_data();
    let mirrored_pokemon = mirrored_client.pokemon().get(1).await?;
    println!("{}", mirrored_pokemon.pokemon_name);

    let client = PogoDataClient::builder()
        .base_url("https://raw.githubusercontent.com/WatWowMap/pogo-data-api/refs/heads/main/data/v1")
        .build();

    let all_moves = client.moves().list().await?;
    println!("{}", all_moves.len());

    let english_translations = client.translations().get_locale("en").await?;
    let misc_translations = client
        .translations()
        .get_category("en", "misc")
        .await?;

    println!("{}", english_translations["misc"]["hello"]);
    println!("{}", misc_translations["hello"]);

    Ok(())
}
```
