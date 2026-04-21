import type {
	datasetDefinitions,
	translationCategories,
	translationLocales,
} from "./generated/metadata";

export type {
	FetchImplementation,
	PogoDataClient,
	PogoDataClientOptions,
	ResourceClient,
	TranslationClient,
} from "./client";
export {
	configurePogoData,
	createPogoDataClient,
	DEFAULT_BASE_URL,
	PogoDataError,
	pogoData,
} from "./client";
export {
	datasetDefinitions,
	translationCategories,
	translationLocales,
} from "./generated/metadata";
export type {
	BaseStats,
	BreadMoveSlot,
	Costume,
	CostumeOverrideEvolution,
	Evolution,
	Form,
	FormChange,
	FormChangeBonusAttribute,
	FormChangeBreadMoveRequirement,
	FormChangeComponentLocationCardSettings,
	FormChangeComponentPokemonSettings,
	FormChangeLocationCardSettings,
	FormChangeMoveReassignment,
	FormChangeMoveRequirement,
	FormChangeQuestRequirement,
	Invasion,
	InvasionEncounter,
	Item,
	LocationCard,
	Move,
	MoveBuff,
	MoveReassignment,
	Pokemon,
	PokemonType,
	QuestCondition,
	QuestRewardType,
	QuestType,
	Raid,
	RouteType,
	SizeSetting,
	Team,
	TempEvolution,
	TranslationCategoryPayload,
	TranslationLocalePayload,
	Weather,
} from "./types";

export type DatasetDefinition = (typeof datasetDefinitions)[number];
export type DatasetName = DatasetDefinition["property"];
export type TranslationCategory = (typeof translationCategories)[number];
export type TranslationLocale = (typeof translationLocales)[number];
