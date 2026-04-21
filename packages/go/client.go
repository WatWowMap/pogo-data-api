package pogodata

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"net/url"
	"strings"
	"sync"
)

const DefaultBaseURL = "https://raw.githubusercontent.com/WatWowMap/pogo-data-api/refs/heads/main/data/v1"

type HTTPClient interface {
	Do(request *http.Request) (*http.Response, error)
}

type ClientOptions struct {
	BaseURL    string
	HTTPClient HTTPClient
	Headers    http.Header
}

type Client struct {
	state *clientState

	Costumes         ResourceClient[Costume]
	Forms            ResourceClient[Form]
	Invasions        ResourceClient[Invasion]
	Items            ResourceClient[Item]
	LocationCards    ResourceClient[LocationCard]
	Moves            ResourceClient[Move]
	Pokemon          ResourceClient[Pokemon]
	QuestConditions  ResourceClient[QuestCondition]
	QuestRewardTypes ResourceClient[QuestRewardType]
	QuestTypes       ResourceClient[QuestType]
	Raids            ResourceClient[Raid]
	RouteTypes       ResourceClient[RouteType]
	Teams            ResourceClient[Team]
	Translations     TranslationClient
	Types            ResourceClient[PokemonType]
	Weather          ResourceClient[Weather]
}

type ResourceClient[T any] struct {
	state       *clientState
	datasetPath string
}

type TranslationClient struct {
	state *clientState
}

type PogoDataError struct {
	message    string
	StatusCode int
	URL        string
	cause      error
}

var sharedState = newClientState(ClientOptions{})
var PogoData = newClientFromState(sharedState)

func ConfigurePogoData(options ClientOptions) {
	sharedState.configure(options)
}

func NewClient(options ClientOptions) *Client {
	return newClientFromState(newClientState(options))
}

func (error *PogoDataError) Error() string {
	return error.message
}

func (error *PogoDataError) Unwrap() error {
	return error.cause
}

func (client ResourceClient[T]) Get(ctx context.Context, id any) (T, error) {
	return fetchJSON[T](
		ctx,
		client.state,
		fmt.Sprintf("%s/%s.json", client.datasetPath, url.PathEscape(fmt.Sprint(id))),
	)
}

func (client ResourceClient[T]) List(ctx context.Context) ([]T, error) {
	return fetchJSON[[]T](ctx, client.state, client.datasetPath+".json")
}

func (client TranslationClient) GetCategory(
	ctx context.Context,
	locale string,
	category string,
) (TranslationCategoryPayload, error) {
	return fetchJSON[TranslationCategoryPayload](
		ctx,
		client.state,
		fmt.Sprintf(
			"translations/%s/%s.json",
			url.PathEscape(locale),
			url.PathEscape(category),
		),
	)
}

func (client TranslationClient) GetLocale(
	ctx context.Context,
	locale string,
) (TranslationLocalePayload, error) {
	return fetchJSON[TranslationLocalePayload](
		ctx,
		client.state,
		fmt.Sprintf("translations/%s.json", url.PathEscape(locale)),
	)
}

type clientState struct {
	mu         sync.RWMutex
	baseURL    string
	httpClient HTTPClient
	headers    http.Header
}

type clientSnapshot struct {
	baseURL    string
	httpClient HTTPClient
	headers    http.Header
}

func newClientState(options ClientOptions) *clientState {
	state := &clientState{}
	state.configure(options)
	return state
}

func newClientFromState(state *clientState) *Client {
	return &Client{
		state:            state,
		Costumes:         newResourceClient[Costume](state, "costumes"),
		Forms:            newResourceClient[Form](state, "forms"),
		Invasions:        newResourceClient[Invasion](state, "invasions"),
		Items:            newResourceClient[Item](state, "items"),
		LocationCards:    newResourceClient[LocationCard](state, "location-cards"),
		Moves:            newResourceClient[Move](state, "moves"),
		Pokemon:          newResourceClient[Pokemon](state, "pokemon"),
		QuestConditions:  newResourceClient[QuestCondition](state, "quest-conditions"),
		QuestRewardTypes: newResourceClient[QuestRewardType](state, "quest-reward-types"),
		QuestTypes:       newResourceClient[QuestType](state, "quest-types"),
		Raids:            newResourceClient[Raid](state, "raids"),
		RouteTypes:       newResourceClient[RouteType](state, "route-types"),
		Teams:            newResourceClient[Team](state, "teams"),
		Translations:     TranslationClient{state: state},
		Types:            newResourceClient[PokemonType](state, "types"),
		Weather:          newResourceClient[Weather](state, "weather"),
	}
}

func (state *clientState) configure(options ClientOptions) {
	baseURL := normalizeBaseURL(options.BaseURL)
	if baseURL == "" {
		baseURL = DefaultBaseURL
	}

	httpClient := options.HTTPClient
	if httpClient == nil {
		httpClient = http.DefaultClient
	}

	state.mu.Lock()
	defer state.mu.Unlock()

	state.baseURL = baseURL
	state.httpClient = httpClient
	state.headers = cloneHeaders(options.Headers)
}

func (state *clientState) snapshot() clientSnapshot {
	state.mu.RLock()
	defer state.mu.RUnlock()

	return clientSnapshot{
		baseURL:    state.baseURL,
		httpClient: state.httpClient,
		headers:    cloneHeaders(state.headers),
	}
}

func newResourceClient[T any](
	state *clientState,
	datasetPath string,
) ResourceClient[T] {
	return ResourceClient[T]{
		state:       state,
		datasetPath: datasetPath,
	}
}

func normalizeBaseURL(baseURL string) string {
	return strings.TrimRight(baseURL, "/")
}

func cloneHeaders(headers http.Header) http.Header {
	if headers == nil {
		return nil
	}

	return headers.Clone()
}

func buildURL(baseURL string, path string) string {
	return normalizeBaseURL(baseURL) + "/" + strings.TrimLeft(path, "/")
}

func fetchJSON[T any](
	ctx context.Context,
	state *clientState,
	path string,
) (T, error) {
	var zero T

	snapshot := state.snapshot()
	requestURL := buildURL(snapshot.baseURL, path)
	request, err := http.NewRequestWithContext(ctx, http.MethodGet, requestURL, nil)
	if err != nil {
		return zero, &PogoDataError{
			message: fmt.Sprintf("Failed to create request for %s.", requestURL),
			URL:     requestURL,
			cause:   err,
		}
	}

	for headerName, values := range snapshot.headers {
		for _, value := range values {
			request.Header.Add(headerName, value)
		}
	}

	response, err := snapshot.httpClient.Do(request)
	if err != nil {
		return zero, &PogoDataError{
			message: fmt.Sprintf("Request failed for %s.", requestURL),
			URL:     requestURL,
			cause:   err,
		}
	}
	defer response.Body.Close()

	if response.StatusCode < http.StatusOK || response.StatusCode >= http.StatusMultipleChoices {
		return zero, &PogoDataError{
			message:    fmt.Sprintf("Request failed for %s with status %d.", requestURL, response.StatusCode),
			StatusCode: response.StatusCode,
			URL:        requestURL,
		}
	}

	var payload T
	if err := json.NewDecoder(response.Body).Decode(&payload); err != nil {
		return zero, &PogoDataError{
			message:    fmt.Sprintf("Failed to parse JSON from %s.", requestURL),
			StatusCode: response.StatusCode,
			URL:        requestURL,
			cause:      err,
		}
	}

	return payload, nil
}
