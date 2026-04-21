# `github.com/WatWowMap/pogo-data-api/packages/go`

Before this package, a Go consumer had to build raw URLs and decode JSON manually:

```go
response, err := http.Get("https://raw.githubusercontent.com/WatWowMap/pogo-data-api/refs/heads/main/data/v1/pokemon/1.json")
if err != nil {
	log.Fatal(err)
}
defer response.Body.Close()

var bulbasaur map[string]any
if err := json.NewDecoder(response.Body).Decode(&bulbasaur); err != nil {
	log.Fatal(err)
}
```

After importing the package, the same lookup is a typed method call:

```go
client := pogodata.NewClient(pogodata.ClientOptions{})
bulbasaur, err := client.Pokemon.Get(context.Background(), 1)
if err != nil {
	log.Fatal(err)
}
```

## Install

```bash
go get github.com/WatWowMap/pogo-data-api/packages/go
```

## Usage

```go
package main

import (
	"context"
	"fmt"
	"log"

	pogodata "github.com/WatWowMap/pogo-data-api/packages/go"
)

func main() {
	sharedPokemon, err := pogodata.PogoData.Pokemon.Get(context.Background(), 1)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println(sharedPokemon.PokemonName)

	pogodata.ConfigurePogoData(pogodata.ClientOptions{
		BaseURL: "https://cdn.example.com/pogo/data/v1",
	})

	mirroredPokemon, err := pogodata.PogoData.Pokemon.Get(context.Background(), 1)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println(mirroredPokemon.PokemonName)

	client := pogodata.NewClient(pogodata.ClientOptions{
		BaseURL: "https://raw.githubusercontent.com/WatWowMap/pogo-data-api/refs/heads/main/data/v1",
	})

	allMoves, err := client.Moves.List(context.Background())
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println(len(allMoves))

	englishTranslations, err := client.Translations.GetLocale(context.Background(), "en")
	if err != nil {
		log.Fatal(err)
	}

	miscTranslations, err := client.Translations.GetCategory(context.Background(), "en", "misc")
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println(englishTranslations["misc"]["hello"])
	fmt.Println(miscTranslations["hello"])
}
```
