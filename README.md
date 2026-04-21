# pogo-data-api

Generated Pokemon GO data as static JSON files, ready to serve from `/data/v1`.

This project takes data from [`pogo-data-generator`](https://www.npmjs.com/package/pogo-data-generator), normalizes the output into a file-based API, and writes the results into the local [`data/v1/`](/Users/rin/GitHub/pogo-data-api/data/v1) directory.

If you deploy this folder behind any static file host, the file paths become your endpoints.

## Packages

This repository now also contains language bindings under `packages/`.

Before, a TypeScript consumer had to fetch raw files directly:

```ts
const response = await fetch(
  "https://raw.githubusercontent.com/WatWowMap/pogo-data-api/refs/heads/main/data/v1/pokemon/1.json",
);

const bulbasaur = await response.json();
```

After, the SDK packages wrap those same endpoints behind typed methods:

```ts
import { createPogoDataClient } from "@watwowmap/pogo-data";

const client = createPogoDataClient();
const bulbasaur = await client.pokemon.get(1);
const englishMisc = await client.translations.getCategory("en", "misc");
```

Current packages:

- [`packages/typescript/`](/Users/rin/GitHub/pogo-data-api/packages/typescript) publishes `@watwowmap/pogo-data`
- [`packages/go/`](/Users/rin/GitHub/pogo-data-api/packages/go) publishes `github.com/WatWowMap/pogo-data-api/packages/go`

The Go package gives the same endpoint coverage with `context.Context` and typed structs:

```go
client := pogodata.NewClient(pogodata.ClientOptions{})
bulbasaur, err := client.Pokemon.Get(context.Background(), 1)
```

The TypeScript package defaults to the canonical hosted `/data/v1` URL, and you can override it once when you want to point at your own mirror:

```ts
import { configurePogoData, pogoData } from "@watwowmap/pogo-data";

configurePogoData({
  baseUrl: "https://cdn.example.com/pogo/data/v1",
});

const mirroredBulbasaur = await pogoData.pokemon.get(1);
```

## What You Get

Before this repo runs, there is no API surface beyond the generator call itself.

After running it, you get a predictable JSON tree like:

```text
data/v1/
  pokemon.json
  pokemon/1.json
  pokemon/150.json
  moves.json
  moves/13.json
  items.json
  items/1.json
  translations/
    en.json
    en/misc.json
    en/pokemon.json
    ja.json
    ja/moves.json
```

That means you can do both:

- bulk reads, like "give me all Pokemon"
- focused reads, like "give me just Bulbasaur"

## Endpoint Model

This repository does not currently run an HTTP server but can be used with GitHub raw.

- `https://raw.githubusercontent.com/WatWowMap/pogo-data-api/refs/heads/main/data/v1/pokemon.json`
- `https://raw.githubusercontent.com/WatWowMap/pogo-data-api/refs/heads/main/data/v1/pokemon/1.json`
- `https://raw.githubusercontent.com/WatWowMap/pogo-data-api/refs/heads/main/data/v1/moves/13.json`
- `https://raw.githubusercontent.com/WatWowMap/pogo-data-api/refs/heads/main/data/v1/translations/en.json`
- `https://raw.githubusercontent.com/WatWowMap/pogo-data-api/refs/heads/main/data/v1/translations/en/misc.json`

## Endpoint Patterns

### 1. Category collection endpoints

These return arrays of values with the original top-level keys removed.

Examples:

- `/data/v1/pokemon.json`
- `/data/v1/moves.json`
- `/data/v1/items.json`
- `/data/v1/forms.json`

Use these when you want the whole dataset for a category in one request.

### 2. Category item endpoints

These return a single value from a category, using the original object key as the filename.

Examples:

- `/data/v1/pokemon/1.json`
- `/data/v1/forms/163.json`
- `/data/v1/moves/13.json`
- `/data/v1/items/1.json`

Use these when you want one record without downloading the full collection.

### 3. Translation locale endpoints

These return the full translation object for a single locale.

Examples:

- `/data/v1/translations/en.json`
- `/data/v1/translations/de.json`
- `/data/v1/translations/ja.json`

### 4. Translation locale-category endpoints

These return a single translation category for a locale.

Examples:

- `/data/v1/translations/en/misc.json`
- `/data/v1/translations/en/pokemon.json`
- `/data/v1/translations/en/items.json`
- `/data/v1/translations/ja/moves.json`

This extra layer is useful when you want only one translation group instead of the full locale payload.

## Current Datasets

The generated output currently includes these top-level datasets:

| Dataset            | File                          | Records |
| ------------------ | ----------------------------- | ------: |
| Costumes           | `/data/v1/costumes.json`           |      87 |
| Forms              | `/data/v1/forms.json`              |    1478 |
| Invasions          | `/data/v1/invasions.json`          |     123 |
| Items              | `/data/v1/items.json`              |     141 |
| Location Cards     | `/data/v1/location-cards.json`     |     180 |
| Moves              | `/data/v1/moves.json`              |     433 |
| Pokemon            | `/data/v1/pokemon.json`            |    1025 |
| Quest Conditions   | `/data/v1/quest-conditions.json`   |      80 |
| Quest Reward Types | `/data/v1/quest-reward-types.json` |      21 |
| Quest Types        | `/data/v1/quest-types.json`        |     102 |
| Raids              | `/data/v1/raids.json`              |      20 |
| Route Types        | `/data/v1/route-types.json`        |       5 |
| Teams              | `/data/v1/teams.json`              |       4 |
| Types              | `/data/v1/types.json`              |      19 |
| Weather            | `/data/v1/weather.json`            |       8 |

Translations are split by locale instead of having a single `/data/v1/translations.json` collection file.

## Translation Coverage

Current locales:

- `de`
- `en`
- `es`
- `es-mx`
- `fr`
- `hi`
- `id`
- `it`
- `ja`
- `ko`
- `pt-br`
- `ru`
- `th`
- `tr`
- `zh-tw`

Current nested translation categories:

- `bonuses`
- `character-categories`
- `costumes`
- `descriptions`
- `evolution-quests`
- `forms`
- `grunt-quotes`
- `grunts`
- `items`
- `lures`
- `misc`
- `moves`
- `pokemon-categories`
- `pokemon`
- `quest-conditions`
- `quest-reward-types`
- `quest-titles`
- `quest-types`
- `types`
- `weather`

## Real Examples

### Example: all Pokemon

`GET /data/v1/pokemon.json`

Response shape:

```json
[
  {
    "pokemonName": "Bulbasaur",
    "pokedexId": 1,
    "defaultFormId": 163,
    "types": [4, 12]
  }
]
```

### Example: one Pokemon

`GET /data/v1/pokemon/1.json`

Response excerpt:

```json
{
  "pokemonName": "Bulbasaur",
  "pokedexId": 1,
  "defaultFormId": 163,
  "types": [4, 12],
  "quickMoves": [214, 221],
  "chargedMoves": [59, 90, 118],
  "generation": "Kanto"
}
```

### Example: one move

`GET /data/v1/moves/13.json`

Response excerpt:

```json
{
  "moveId": 13,
  "moveName": "Wrap",
  "proto": "WRAP",
  "fast": false,
  "type": 1,
  "power": 60,
  "durationMs": 3000
}
```

### Example: full English translations

`GET /data/v1/translations/en.json`

Response shape:

```json
{
  "misc": {
    "alola": "Alola"
  },
  "pokemon": {
    "bulbasaur": "Bulbasaur"
  }
}
```

### Example: one English translation category

`GET /data/v1/translations/en/misc.json`

Response excerpt:

```json
{
  "alola": "Alola",
  "egg_0": "Unset Egg"
}
```

## How The Files Are Named

- top-level category names are converted to kebab-case
- item files use the original object key converted to kebab-case
- translation locale files use the locale key directly, such as `en.json` or `pt-br.json`
- nested translation category files also use kebab-case

Examples:

- `questRewardTypes` becomes `/data/v1/quest-reward-types.json`
- `locationCards` becomes `/data/v1/location-cards.json`
- translation locale `pt-br` becomes `/data/v1/translations/pt-br.json`
- translation category `characterCategories` becomes `/data/v1/translations/en/character-categories.json`

## Generate The API

Install dependencies:

```bash
bun install
```

Generate the JSON output:

```bash
bun run index.ts
```

Or:

```bash
bun run start
```

The generator:

- fetches raw Pokemon GO data through `pogo-data-generator`
- writes fresh output into [`data/v1/`](/Users/rin/GitHub/pogo-data-api/data/v1)
- removes stale generated files before writing new ones
- creates per-record files for direct lookup
- creates nested translation files for locale and locale-category access

## Workspace And Releases

- the repo root remains responsible for generating the static JSON API
- `packages/` is where publishable language bindings live
- `.changeset/` tracks package releases for JavaScript packages
- `.github/workflows/ci.yml` verifies metadata generation, builds, typechecks, and tests the workspace
- `.github/workflows/release-packages.yml` opens or publishes NPM releases for package changes on `main`

## Implementation Notes

The generation script in [index.ts](/Users/rin/GitHub/pogo-data-api/index.ts):

- uses Bun for the runtime and file writes
- writes files with bounded concurrency for better throughput
- builds the API as static JSON rather than a live server
- generates raw data with translations split by locale and category

## When To Use Which Endpoint

- Use collection endpoints like `/data/v1/pokemon.json` when you need everything in one request.
- Use item endpoints like `/data/v1/pokemon/1.json` when you only need one record.
- Use `/data/v1/translations/<locale>.json` when you need a full locale pack.
- Use `/data/v1/translations/<locale>/<category>.json` when you want smaller translation payloads and faster client startup.
