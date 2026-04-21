package pogodata

import (
	"context"
	"errors"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
)

func TestNewClientProvidesTypedDatasetAndTranslationMethods(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(writer http.ResponseWriter, request *http.Request) {
		if request.Header.Get("X-Test") != "present" {
			t.Fatalf("expected X-Test header to be forwarded")
		}

		writer.Header().Set("content-type", "application/json")

		switch request.URL.Path {
		case "/pokemon/1.json":
			_, _ = writer.Write([]byte(`{"pokemonName":"Bulbasaur","pokedexId":1,"defaultFormId":163,"types":[4,12],"quickMoves":[214],"chargedMoves":[90],"eliteQuickMoves":[],"eliteChargedMoves":[],"forms":[163],"sizeSettings":[],"attack":118,"defense":111,"stamina":128,"legendary":false,"mythic":false,"genId":1,"generation":"Kanto"}`))
		case "/pokemon.json":
			_, _ = writer.Write([]byte(`[{"pokemonName":"Bulbasaur","pokedexId":1,"defaultFormId":163,"types":[4,12],"quickMoves":[214],"chargedMoves":[90],"eliteQuickMoves":[],"eliteChargedMoves":[],"forms":[163],"sizeSettings":[],"attack":118,"defense":111,"stamina":128,"legendary":false,"mythic":false,"genId":1,"generation":"Kanto"}]`))
		case "/translations/en.json":
			_, _ = writer.Write([]byte(`{"misc":{"hello":"Hello"}}`))
		case "/translations/en/misc.json":
			_, _ = writer.Write([]byte(`{"hello":"Hello"}`))
		default:
			http.NotFound(writer, request)
		}
	}))
	defer server.Close()

	client := NewClient(ClientOptions{
		BaseURL: server.URL,
		Headers: http.Header{
			"X-Test": []string{"present"},
		},
	})

	bulbasaur, err := client.Pokemon.Get(context.Background(), 1)
	if err != nil {
		t.Fatalf("expected pokemon lookup to succeed: %v", err)
	}

	if bulbasaur.PokemonName != "Bulbasaur" {
		t.Fatalf("expected Bulbasaur, got %q", bulbasaur.PokemonName)
	}

	allPokemon, err := client.Pokemon.List(context.Background())
	if err != nil {
		t.Fatalf("expected pokemon list lookup to succeed: %v", err)
	}

	if len(allPokemon) != 1 || allPokemon[0].PokedexID != 1 {
		t.Fatalf("expected one Bulbasaur entry, got %#v", allPokemon)
	}

	localePayload, err := client.Translations.GetLocale(context.Background(), "en")
	if err != nil {
		t.Fatalf("expected locale lookup to succeed: %v", err)
	}

	if localePayload["misc"]["hello"] != "Hello" {
		t.Fatalf("expected locale payload to include misc.hello")
	}

	categoryPayload, err := client.Translations.GetCategory(context.Background(), "en", "misc")
	if err != nil {
		t.Fatalf("expected category lookup to succeed: %v", err)
	}

	if categoryPayload["hello"] != "Hello" {
		t.Fatalf("expected category payload to include hello")
	}
}

func TestConfigurePogoDataUpdatesSharedClient(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(writer http.ResponseWriter, request *http.Request) {
		writer.Header().Set("content-type", "application/json")
		_, _ = writer.Write([]byte(`{"moveId":13,"moveName":"Wrap"}`))
	}))
	defer server.Close()

	ConfigurePogoData(ClientOptions{BaseURL: server.URL})
	t.Cleanup(func() {
		ConfigurePogoData(ClientOptions{})
	})

	move, err := PogoData.Moves.Get(context.Background(), 13)
	if err != nil {
		t.Fatalf("expected shared client lookup to succeed: %v", err)
	}

	if move.MoveName != "Wrap" {
		t.Fatalf("expected Wrap, got %q", move.MoveName)
	}
}

func TestReturnsTypedErrorOnNonOKResponse(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(writer http.ResponseWriter, request *http.Request) {
		http.Error(writer, "missing", http.StatusNotFound)
	}))
	defer server.Close()

	client := NewClient(ClientOptions{BaseURL: server.URL})

	_, err := client.Pokemon.Get(context.Background(), 999999)
	if err == nil {
		t.Fatalf("expected an error for a 404 response")
	}

	var pogoDataError *PogoDataError
	if !errors.As(err, &pogoDataError) {
		t.Fatalf("expected PogoDataError, got %T", err)
	}

	if pogoDataError.StatusCode != http.StatusNotFound {
		t.Fatalf("expected 404 status, got %d", pogoDataError.StatusCode)
	}

	if !strings.Contains(pogoDataError.URL, "/pokemon/999999.json") {
		t.Fatalf("expected request URL to be captured, got %q", pogoDataError.URL)
	}
}

func TestReturnsTypedErrorOnInvalidJSON(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(writer http.ResponseWriter, request *http.Request) {
		writer.Header().Set("content-type", "application/json")
		_, _ = writer.Write([]byte(`{"broken":`))
	}))
	defer server.Close()

	client := NewClient(ClientOptions{BaseURL: server.URL})

	_, err := client.Pokemon.Get(context.Background(), 1)
	if err == nil {
		t.Fatalf("expected an error for invalid JSON")
	}

	var pogoDataError *PogoDataError
	if !errors.As(err, &pogoDataError) {
		t.Fatalf("expected PogoDataError, got %T", err)
	}

	if pogoDataError.StatusCode != http.StatusOK {
		t.Fatalf("expected parse errors to keep the response status, got %d", pogoDataError.StatusCode)
	}
}

func TestPublishesDiscoveredMetadata(t *testing.T) {
	if len(DatasetDefinitions) == 0 {
		t.Fatalf("expected dataset metadata to be populated")
	}

	if len(TranslationLocales) == 0 || len(TranslationCategories) == 0 {
		t.Fatalf("expected translation metadata to be populated")
	}

	if TranslationLocales[0] == "" {
		t.Fatalf("expected the first locale entry to be non-empty")
	}

	foundEnglish := false
	for _, locale := range TranslationLocales {
		if locale == "en" {
			foundEnglish = true
			break
		}
	}

	if !foundEnglish {
		t.Fatalf("expected english locale metadata to be published")
	}
}
