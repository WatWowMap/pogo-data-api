# `@watwowmap/pogo-data`

Before this package, a consumer had to know the raw URL layout and fetch endpoints manually:

```ts
const response = await fetch(
  "https://raw.githubusercontent.com/WatWowMap/pogo-data-api/refs/heads/main/data/v1/pokemon/1.json",
);

const bulbasaur = await response.json();
```

After installing the SDK, the same lookup is a typed method call:

```ts
import { createPogoDataClient } from "@watwowmap/pogo-data";

const client = createPogoDataClient();
const bulbasaur = await client.pokemon.get(1);
```

## Install

```bash
npm install @watwowmap/pogo-data
```

## Usage

```ts
import {
  configurePogoData,
  createPogoDataClient,
  pogoData,
} from "@watwowmap/pogo-data";

const sharedPokemon = await pogoData.pokemon.get(1);

configurePogoData({
  baseUrl: "https://cdn.example.com/pogo/data/v1",
});

const mirroredPokemon = await pogoData.pokemon.get(1);

const client = createPogoDataClient({
  baseUrl: "https://raw.githubusercontent.com/WatWowMap/pogo-data-api/refs/heads/main/data/v1",
});

const allMoves = await client.moves.list();
const englishTranslations = await client.translations.getLocale("en");
const miscTranslations = await client.translations.getCategory("en", "misc");
```
