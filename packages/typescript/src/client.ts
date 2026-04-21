import { datasetDefinitions } from "./generated/metadata";
import type {
	Costume,
	Form,
	Invasion,
	Item,
	LocationCard,
	Move,
	Pokemon,
	PokemonType,
	QuestCondition,
	QuestRewardType,
	QuestType,
	Raid,
	RouteType,
	Team,
	TranslationCategoryPayload,
	TranslationLocalePayload,
	Weather,
} from "./types";

export const DEFAULT_BASE_URL =
	"https://raw.githubusercontent.com/WatWowMap/pogo-data-api/refs/heads/main/v1";

export type FetchImplementation = (
	input: RequestInfo | URL,
	init?: RequestInit,
) => Promise<Response>;

interface ClientState {
	baseUrl: string;
	fetchImplementation?: FetchImplementation;
	headers?: HeadersInit;
}

interface DatasetValueMap {
	costumes: Costume;
	forms: Form;
	invasions: Invasion;
	items: Item;
	locationCards: LocationCard;
	moves: Move;
	pokemon: Pokemon;
	questConditions: QuestCondition;
	questRewardTypes: QuestRewardType;
	questTypes: QuestType;
	raids: Raid;
	routeTypes: RouteType;
	teams: Team;
	types: PokemonType;
	weather: Weather;
}

type DatasetKey = keyof DatasetValueMap;

export interface PogoDataClientOptions {
	baseUrl?: string;
	fetch?: FetchImplementation;
	headers?: HeadersInit;
}

export interface ResourceClient<TValue> {
	get(id: string | number): Promise<TValue>;
	list(): Promise<TValue[]>;
}

export interface TranslationClient {
	getCategory(
		locale: string,
		category: string,
	): Promise<TranslationCategoryPayload>;
	getLocale(locale: string): Promise<TranslationLocalePayload>;
}

export interface PogoDataClient {
	costumes: ResourceClient<Costume>;
	forms: ResourceClient<Form>;
	invasions: ResourceClient<Invasion>;
	items: ResourceClient<Item>;
	locationCards: ResourceClient<LocationCard>;
	moves: ResourceClient<Move>;
	pokemon: ResourceClient<Pokemon>;
	questConditions: ResourceClient<QuestCondition>;
	questRewardTypes: ResourceClient<QuestRewardType>;
	questTypes: ResourceClient<QuestType>;
	raids: ResourceClient<Raid>;
	routeTypes: ResourceClient<RouteType>;
	teams: ResourceClient<Team>;
	translations: TranslationClient;
	types: ResourceClient<PokemonType>;
	weather: ResourceClient<Weather>;
}

export interface PogoDataError extends Error {
	cause?: unknown;
	status?: number;
	url: string;
}

interface PogoDataErrorOptions {
	cause?: unknown;
	status?: number;
	url: string;
}

export class PogoDataError extends Error {
	override cause?: unknown;
	status?: number;
	url: string;

	constructor(message: string, options: PogoDataErrorOptions) {
		super(message);
		this.name = "PogoDataError";
		this.cause = options.cause;
		this.status = options.status;
		this.url = options.url;
	}
}

function normalizeBaseUrl(baseUrl: string): string {
	return baseUrl.replace(/\/+$/, "");
}

function createHeaders(headers?: HeadersInit): Headers {
	const requestHeaders = new Headers(headers);

	if (!requestHeaders.has("accept")) {
		requestHeaders.set("accept", "application/json");
	}

	return requestHeaders;
}

function createClientState(options: PogoDataClientOptions = {}): ClientState {
	return {
		baseUrl: normalizeBaseUrl(options.baseUrl ?? DEFAULT_BASE_URL),
		fetchImplementation: options.fetch,
		headers: options.headers,
	};
}

function updateClientState(
	state: ClientState,
	options: PogoDataClientOptions,
): void {
	if ("baseUrl" in options) {
		state.baseUrl = normalizeBaseUrl(options.baseUrl ?? DEFAULT_BASE_URL);
	}

	if ("fetch" in options) {
		state.fetchImplementation = options.fetch;
	}

	if ("headers" in options) {
		state.headers = options.headers;
	}
}

function buildUrl(baseUrl: string, path: string): string {
	return `${normalizeBaseUrl(baseUrl)}/${path.replace(/^\/+/, "")}`;
}

function getFetchImplementation(state: ClientState): FetchImplementation {
	if (state.fetchImplementation !== undefined) {
		return state.fetchImplementation;
	}

	if (typeof globalThis.fetch !== "function") {
		throw new PogoDataError(
			"Global fetch is not available. Pass a fetch implementation in PogoDataClientOptions.fetch.",
			{
				url: state.baseUrl,
			},
		);
	}

	return globalThis.fetch.bind(globalThis);
}

async function fetchJson<TValue>(
	state: ClientState,
	path: string,
): Promise<TValue> {
	const url = buildUrl(state.baseUrl, path);
	const fetchImplementation = getFetchImplementation(state);
	let response: Response;

	try {
		response = await fetchImplementation(url, {
			headers: createHeaders(state.headers),
		});
	} catch (error) {
		throw new PogoDataError(`Request failed for ${url}.`, {
			cause: error,
			url,
		});
	}

	if (!response.ok) {
		throw new PogoDataError(
			`Request failed for ${url} with status ${response.status}.`,
			{
				status: response.status,
				url,
			},
		);
	}

	try {
		return (await response.json()) as TValue;
	} catch (error) {
		throw new PogoDataError(`Failed to parse JSON from ${url}.`, {
			cause: error,
			status: response.status,
			url,
		});
	}
}

function createResourceClient<TValue>(
	state: ClientState,
	datasetPath: string,
): ResourceClient<TValue> {
	return {
		get(id) {
			return fetchJson<TValue>(
				state,
				`${datasetPath}/${encodeURIComponent(String(id))}.json`,
			);
		},
		list() {
			return fetchJson<TValue[]>(state, `${datasetPath}.json`);
		},
	};
}

function createTranslationClient(state: ClientState): TranslationClient {
	return {
		getCategory(locale, category) {
			return fetchJson<TranslationCategoryPayload>(
				state,
				`translations/${encodeURIComponent(locale)}/${encodeURIComponent(category)}.json`,
			);
		},
		getLocale(locale) {
			return fetchJson<TranslationLocalePayload>(
				state,
				`translations/${encodeURIComponent(locale)}.json`,
			);
		},
	};
}

function createDatasetClients(
	state: ClientState,
): Pick<PogoDataClient, DatasetKey> {
	const datasetClients = {} as Record<string, ResourceClient<unknown>>;

	for (const definition of datasetDefinitions) {
		datasetClients[definition.property] = createResourceClient(
			state,
			definition.path,
		);
	}

	return datasetClients as Pick<PogoDataClient, DatasetKey>;
}

function createClientFromState(state: ClientState): PogoDataClient {
	return {
		...createDatasetClients(state),
		translations: createTranslationClient(state),
	};
}

const singletonState = createClientState();

export function createPogoDataClient(
	options: PogoDataClientOptions = {},
): PogoDataClient {
	return createClientFromState(createClientState(options));
}

export function configurePogoData(options: PogoDataClientOptions): void {
	updateClientState(singletonState, options);
}

export const pogoData = createClientFromState(singletonState);
