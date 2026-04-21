import type { Pokemon } from "../src";

const validPokemon: Pokemon = {
	attack: 118,
	chargedMoves: [59, 90, 118],
	defaultFormId: 163,
	defense: 111,
	eliteChargedMoves: [],
	eliteQuickMoves: [],
	forms: [163, 897],
	genId: 1,
	generation: "Kanto",
	legendary: false,
	mythic: false,
	pokedexId: 1,
	pokemonName: "Bulbasaur",
	quickMoves: [214, 221],
	sizeSettings: [
		{
			name: "xxsLowerBound",
			value: 0.343,
		},
	],
	stamina: 128,
	types: [4, 12],
};

void validPokemon;

// @ts-expect-error Pokemon records always include pokemonName in data/v1/pokemon.json.
const missingPokemonName: Pokemon = {
	attack: 118,
	chargedMoves: [59, 90, 118],
	defaultFormId: 163,
	defense: 111,
	eliteChargedMoves: [],
	eliteQuickMoves: [],
	forms: [163, 897],
	genId: 1,
	generation: "Kanto",
	legendary: false,
	mythic: false,
	pokedexId: 1,
	quickMoves: [214, 221],
	sizeSettings: [
		{
			name: "xxsLowerBound",
			value: 0.343,
		},
	],
	stamina: 128,
	types: [4, 12],
};

void missingPokemonName;

// @ts-expect-error Pokemon records always include eliteChargedMoves in data/v1/pokemon.json.
const missingEliteChargedMoves: Pokemon = {
	attack: 118,
	chargedMoves: [59, 90, 118],
	defaultFormId: 163,
	defense: 111,
	eliteQuickMoves: [],
	forms: [163, 897],
	genId: 1,
	generation: "Kanto",
	legendary: false,
	mythic: false,
	pokedexId: 1,
	pokemonName: "Bulbasaur",
	quickMoves: [214, 221],
	sizeSettings: [
		{
			name: "xxsLowerBound",
			value: 0.343,
		},
	],
	stamina: 128,
	types: [4, 12],
};

void missingEliteChargedMoves;
