export interface BaseStats {
	attack?: number;
	defense?: number;
	height?: number;
	stamina?: number;
	types?: number[];
	weight?: number;
}

export interface Costume {
	id: number;
	name: string;
	noEvolve: boolean;
	proto: string;
}

export interface Evolution {
	candyCost?: number;
	evoId?: number;
	formId?: number;
	genderRequirement?: number | string;
	itemRequirement?: number;
	mustBeBuddy?: boolean;
	onlyDaytime?: boolean;
	onlyNighttime?: boolean;
	questRequirement?: string;
	tradeBonus?: boolean;
}

export interface FormChangeBonusAttribute {
	breadMode?: string;
	clearBreadMode?: boolean;
	maxMoves?: BreadMoveSlot[];
	targetForm?: number;
}

export interface FormChangeBreadMoveRequirement {
	moveLevel?: string;
	moveTypes?: string[];
}

export interface FormChangeComponentLocationCardSettings {
	basePokemonLocationCard?: number;
	componentPokemonLocationCard?: number;
	fusionPokemonLocationCard?: number;
}

export interface FormChangeComponentPokemonSettings {
	componentCandyCost?: number;
	familyId?: number;
	formChangeType?: string;
	formId?: number;
	fusionMove1?: number;
	fusionMove2?: number;
	locationCardSettings?: FormChangeComponentLocationCardSettings[];
	pokedexId?: number;
}

export interface FormChangeLocationCardSettings {
	existingLocationCard?: number;
	replacementLocationCard?: number;
}

export interface FormChangeMoveReassignment {
	chargedMoves?: MoveReassignment[];
	quickMoves?: MoveReassignment[];
}

export interface FormChangeMoveRequirement {
	requiredMoves?: number[];
}

export interface FormChangeQuestRequirement {
	description?: string;
	questRequirement?: string;
	target?: number;
}

export interface FormChange {
	availableForms?: number[];
	candyCost?: number;
	componentPokemonSettings?: FormChangeComponentPokemonSettings;
	formChangeBonusAttributes?: FormChangeBonusAttribute[];
	itemCostCount?: number;
	itemRequirement?: number;
	locationCardSettings?: FormChangeLocationCardSettings[];
	moveReassignment?: FormChangeMoveReassignment;
	priority?: number;
	questRequirements?: FormChangeQuestRequirement[];
	requiredBreadMoves?: FormChangeBreadMoveRequirement[];
	requiredChargedMoves?: FormChangeMoveRequirement[];
	requiredQuickMoves?: FormChangeMoveRequirement[];
	stardustCost?: number;
}

export interface Form extends BaseStats {
	bonusCandyCapture?: number;
	bonusStardustCapture?: number;
	chargedMoves?: number[];
	costumeOverrideEvos?: CostumeOverrideEvolution[];
	eliteChargedMoves?: number[];
	eliteQuickMoves?: number[];
	evolutions?: Evolution[];
	family?: number;
	formChanges?: FormChange[];
	formId?: number;
	formName?: string;
	gmaxMove?: number;
	isCostume?: boolean;
	little?: boolean;
	proto?: string;
	purificationCandy?: number;
	purificationDust?: number;
	quickMoves?: number[];
	sizeSettings?: SizeSetting[];
	tempEvolutions?: TempEvolution[];
	tradable?: boolean;
	transferable?: boolean;
}

export interface Invasion {
	active: boolean;
	encounters?: InvasionEncounter[];
	firstReward: boolean;
	gender: number | string;
	grunt: string;
	id: number;
	proto: string;
	secondReward: boolean;
	thirdReward: boolean;
	type: string;
}

export interface InvasionEncounter {
	formId?: number;
	id?: number;
	position?: string;
}

export interface Item {
	category: string;
	itemId: number;
	itemName: string;
	minTrainerLevel: number;
	proto: string;
	type: string;
}

export interface LocationCard {
	cardType?: string;
	formatted: string;
	id: number;
	imageUrl?: string;
	proto: string;
	vfxAddress?: string;
}

export interface Move {
	durationMs?: number;
	energyDelta?: number;
	fast?: boolean;
	moveId: number;
	moveName: string;
	proto?: string;
	pvpBuffs?: MoveBuff[];
	pvpDurationTurns?: number;
	pvpEnergyDelta?: number;
	pvpPower?: number;
	power?: number;
	type?: number;
}

export interface MoveBuff {
	attackerAttackStatStageChange?: number;
	attackerDefenseStatStageChange?: number;
	buffActivationChance: number;
	targetAttackStatStageChange?: number;
	targetDefenseStatStageChange?: number;
}

export interface MoveReassignment {
	existingMoves?: number[];
	replacementMoves?: number[];
}

export interface Pokemon extends Form {
	attack: number;
	chargedMoves: number[];
	defaultFormId: number;
	defense: number;
	eliteChargedMoves: number[];
	eliteQuickMoves: number[];
	forms: number[];
	genId: number;
	generation: string;
	legendary: boolean;
	mythic: boolean;
	pokedexId: number;
	pokemonName: string;
	quickMoves: number[];
	sizeSettings: SizeSetting[];
	stamina: number;
	types: number[];
	buddyDistance?: number;
	buddyGroupNumber?: number;
	buddyMegaEnergy?: number;
	captureRate?: number;
	family?: number;
	fleeRate?: number;
	gymDefenderEligible?: boolean;
	height?: number;
	jungle?: boolean;
	thirdMoveCandy?: number;
	thirdMoveStardust?: number;
	tradable?: boolean;
	transferable?: boolean;
	ultraBeast?: boolean;
	unreleased?: boolean;
	weight?: number;
}

export interface PokemonType {
	immunes?: number[];
	resistances?: number[];
	strengths?: number[];
	typeId?: number;
	typeName?: string;
	veryWeakAgainst?: number[];
	weakAgainst?: number[];
	weaknesses?: number[];
}

export interface QuestCondition {
	formatted: string;
	proto: string;
	questId: number;
}

export interface QuestRewardType {
	formatted: string;
	proto: string;
	questId: number;
}

export interface QuestType {
	formatted: string;
	proto: string;
	questId: number;
}

export interface Raid {
	formatted: string;
	id: number;
	proto: string;
}

export interface RouteType {
	formatted: string;
	id: number;
	proto: string;
}

export interface SizeSetting {
	name: string;
	value: number;
}

export interface Team {
	formatted: string;
	id: number;
	proto: string;
}

export interface TempEvolution extends BaseStats {
	firstEnergyCost?: number;
	subsequentEnergyCost?: number;
	tempEvoId: number | string;
	unreleased?: boolean;
}

export interface TranslationCategoryPayload {
	[key: string]: string;
}

export interface TranslationLocalePayload {
	[category: string]: TranslationCategoryPayload;
}

export interface Weather {
	proto: string;
	types: number[];
	weatherId: number;
	weatherName: string;
}

export interface BreadMoveSlot {
	moveLevel?: string;
	moveType?: string;
}

export interface CostumeOverrideEvolution {
	costumeId: number;
	costumeName: string;
	costumeProto: string;
}
