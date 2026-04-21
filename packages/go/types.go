package pogodata

import (
	"bytes"
	"encoding/json"
	"strconv"
)

type StringOrNumber string

func (value *StringOrNumber) UnmarshalJSON(data []byte) error {
	if bytes.Equal(data, []byte("null")) {
		*value = ""
		return nil
	}

	if len(data) > 0 && data[0] == '"' {
		var stringValue string
		if err := json.Unmarshal(data, &stringValue); err != nil {
			return err
		}

		*value = StringOrNumber(stringValue)
		return nil
	}

	var numberValue json.Number
	if err := json.Unmarshal(data, &numberValue); err != nil {
		return err
	}

	*value = StringOrNumber(numberValue.String())
	return nil
}

func (value StringOrNumber) String() string {
	return string(value)
}

func (value StringOrNumber) Int() (int, error) {
	return strconv.Atoi(string(value))
}

type BaseStats struct {
	Attack  *int     `json:"attack,omitempty"`
	Defense *int     `json:"defense,omitempty"`
	Height  *float64 `json:"height,omitempty"`
	Stamina *int     `json:"stamina,omitempty"`
	Types   []int    `json:"types,omitempty"`
	Weight  *float64 `json:"weight,omitempty"`
}

type Costume struct {
	ID       int    `json:"id"`
	Name     string `json:"name"`
	NoEvolve bool   `json:"noEvolve"`
	Proto    string `json:"proto"`
}

type Evolution struct {
	CandyCost         *int            `json:"candyCost,omitempty"`
	EvoID             *int            `json:"evoId,omitempty"`
	FormID            *int            `json:"formId,omitempty"`
	GenderRequirement *StringOrNumber `json:"genderRequirement,omitempty"`
	ItemRequirement   *int            `json:"itemRequirement,omitempty"`
	MustBeBuddy       *bool           `json:"mustBeBuddy,omitempty"`
	OnlyDaytime       *bool           `json:"onlyDaytime,omitempty"`
	OnlyNighttime     *bool           `json:"onlyNighttime,omitempty"`
	QuestRequirement  *string         `json:"questRequirement,omitempty"`
	TradeBonus        *bool           `json:"tradeBonus,omitempty"`
}

type FormChangeBonusAttribute struct {
	BreadMode      *string         `json:"breadMode,omitempty"`
	ClearBreadMode *bool           `json:"clearBreadMode,omitempty"`
	MaxMoves       []BreadMoveSlot `json:"maxMoves,omitempty"`
	TargetForm     *int            `json:"targetForm,omitempty"`
}

type FormChangeBreadMoveRequirement struct {
	MoveLevel *string  `json:"moveLevel,omitempty"`
	MoveTypes []string `json:"moveTypes,omitempty"`
}

type FormChangeComponentLocationCardSettings struct {
	BasePokemonLocationCard      *int `json:"basePokemonLocationCard,omitempty"`
	ComponentPokemonLocationCard *int `json:"componentPokemonLocationCard,omitempty"`
	FusionPokemonLocationCard    *int `json:"fusionPokemonLocationCard,omitempty"`
}

type FormChangeComponentPokemonSettings struct {
	ComponentCandyCost   *int                                      `json:"componentCandyCost,omitempty"`
	FamilyID             *int                                      `json:"familyId,omitempty"`
	FormChangeType       *string                                   `json:"formChangeType,omitempty"`
	FormID               *int                                      `json:"formId,omitempty"`
	FusionMove1          *int                                      `json:"fusionMove1,omitempty"`
	FusionMove2          *int                                      `json:"fusionMove2,omitempty"`
	LocationCardSettings []FormChangeComponentLocationCardSettings `json:"locationCardSettings,omitempty"`
	PokedexID            *int                                      `json:"pokedexId,omitempty"`
}

type FormChangeLocationCardSettings struct {
	ExistingLocationCard    *int `json:"existingLocationCard,omitempty"`
	ReplacementLocationCard *int `json:"replacementLocationCard,omitempty"`
}

type FormChangeMoveReassignment struct {
	ChargedMoves []MoveReassignment `json:"chargedMoves,omitempty"`
	QuickMoves   []MoveReassignment `json:"quickMoves,omitempty"`
}

type FormChangeMoveRequirement struct {
	RequiredMoves []int `json:"requiredMoves,omitempty"`
}

type FormChangeQuestRequirement struct {
	Description      *string `json:"description,omitempty"`
	QuestRequirement *string `json:"questRequirement,omitempty"`
	Target           *int    `json:"target,omitempty"`
}

type FormChange struct {
	AvailableForms            []int                               `json:"availableForms,omitempty"`
	CandyCost                 *int                                `json:"candyCost,omitempty"`
	ComponentPokemonSettings  *FormChangeComponentPokemonSettings `json:"componentPokemonSettings,omitempty"`
	FormChangeBonusAttributes []FormChangeBonusAttribute          `json:"formChangeBonusAttributes,omitempty"`
	ItemCostCount             *int                                `json:"itemCostCount,omitempty"`
	ItemRequirement           *int                                `json:"itemRequirement,omitempty"`
	LocationCardSettings      []FormChangeLocationCardSettings    `json:"locationCardSettings,omitempty"`
	MoveReassignment          *FormChangeMoveReassignment         `json:"moveReassignment,omitempty"`
	Priority                  *int                                `json:"priority,omitempty"`
	QuestRequirements         []FormChangeQuestRequirement        `json:"questRequirements,omitempty"`
	RequiredBreadMoves        []FormChangeBreadMoveRequirement    `json:"requiredBreadMoves,omitempty"`
	RequiredChargedMoves      []FormChangeMoveRequirement         `json:"requiredChargedMoves,omitempty"`
	RequiredQuickMoves        []FormChangeMoveRequirement         `json:"requiredQuickMoves,omitempty"`
	StardustCost              *int                                `json:"stardustCost,omitempty"`
}

type Form struct {
	Attack               *int                       `json:"attack,omitempty"`
	BonusCandyCapture    *int                       `json:"bonusCandyCapture,omitempty"`
	BonusStardustCapture *int                       `json:"bonusStardustCapture,omitempty"`
	ChargedMoves         []int                      `json:"chargedMoves,omitempty"`
	CostumeOverrideEvos  []CostumeOverrideEvolution `json:"costumeOverrideEvos,omitempty"`
	Defense              *int                       `json:"defense,omitempty"`
	EliteChargedMoves    []int                      `json:"eliteChargedMoves,omitempty"`
	EliteQuickMoves      []int                      `json:"eliteQuickMoves,omitempty"`
	Evolutions           []Evolution                `json:"evolutions,omitempty"`
	Family               *int                       `json:"family,omitempty"`
	FormChanges          []FormChange               `json:"formChanges,omitempty"`
	FormID               *int                       `json:"formId,omitempty"`
	FormName             *string                    `json:"formName,omitempty"`
	GMaxMove             *int                       `json:"gmaxMove,omitempty"`
	Height               *float64                   `json:"height,omitempty"`
	IsCostume            *bool                      `json:"isCostume,omitempty"`
	Little               *bool                      `json:"little,omitempty"`
	Proto                *string                    `json:"proto,omitempty"`
	PurificationCandy    *int                       `json:"purificationCandy,omitempty"`
	PurificationDust     *int                       `json:"purificationDust,omitempty"`
	QuickMoves           []int                      `json:"quickMoves,omitempty"`
	SizeSettings         []SizeSetting              `json:"sizeSettings,omitempty"`
	Stamina              *int                       `json:"stamina,omitempty"`
	TempEvolutions       []TempEvolution            `json:"tempEvolutions,omitempty"`
	Tradable             *bool                      `json:"tradable,omitempty"`
	Transferable         *bool                      `json:"transferable,omitempty"`
	Types                []int                      `json:"types,omitempty"`
	Weight               *float64                   `json:"weight,omitempty"`
}

type Invasion struct {
	Active       bool                `json:"active"`
	Encounters   []InvasionEncounter `json:"encounters,omitempty"`
	FirstReward  bool                `json:"firstReward"`
	Gender       StringOrNumber      `json:"gender"`
	Grunt        string              `json:"grunt"`
	ID           int                 `json:"id"`
	Proto        string              `json:"proto"`
	SecondReward bool                `json:"secondReward"`
	ThirdReward  bool                `json:"thirdReward"`
	Type         string              `json:"type"`
}

type InvasionEncounter struct {
	FormID   *int    `json:"formId,omitempty"`
	ID       *int    `json:"id,omitempty"`
	Position *string `json:"position,omitempty"`
}

type Item struct {
	Category        string `json:"category"`
	ItemID          int    `json:"itemId"`
	ItemName        string `json:"itemName"`
	MinTrainerLevel int    `json:"minTrainerLevel"`
	Proto           string `json:"proto"`
	Type            string `json:"type"`
}

type LocationCard struct {
	CardType   *string `json:"cardType,omitempty"`
	Formatted  string  `json:"formatted"`
	ID         int     `json:"id"`
	ImageURL   *string `json:"imageUrl,omitempty"`
	Proto      string  `json:"proto"`
	VFXAddress *string `json:"vfxAddress,omitempty"`
}

type Move struct {
	DurationMs       *int       `json:"durationMs,omitempty"`
	EnergyDelta      *int       `json:"energyDelta,omitempty"`
	Fast             *bool      `json:"fast,omitempty"`
	MoveID           int        `json:"moveId"`
	MoveName         string     `json:"moveName"`
	Proto            *string    `json:"proto,omitempty"`
	PVPBuffs         []MoveBuff `json:"pvpBuffs,omitempty"`
	PVPDurationTurns *int       `json:"pvpDurationTurns,omitempty"`
	PVPEnergyDelta   *int       `json:"pvpEnergyDelta,omitempty"`
	PVPPower         *int       `json:"pvpPower,omitempty"`
	Power            *int       `json:"power,omitempty"`
	Type             *int       `json:"type,omitempty"`
}

type MoveBuff struct {
	AttackerAttackStatStageChange  *int    `json:"attackerAttackStatStageChange,omitempty"`
	AttackerDefenseStatStageChange *int    `json:"attackerDefenseStatStageChange,omitempty"`
	BuffActivationChance           float64 `json:"buffActivationChance"`
	TargetAttackStatStageChange    *int    `json:"targetAttackStatStageChange,omitempty"`
	TargetDefenseStatStageChange   *int    `json:"targetDefenseStatStageChange,omitempty"`
}

type MoveReassignment struct {
	ExistingMoves    []int `json:"existingMoves,omitempty"`
	ReplacementMoves []int `json:"replacementMoves,omitempty"`
}

type Pokemon struct {
	Attack              int                        `json:"attack"`
	BuddyDistance       *int                       `json:"buddyDistance,omitempty"`
	BuddyGroupNumber    *int                       `json:"buddyGroupNumber,omitempty"`
	BuddyMegaEnergy     *int                       `json:"buddyMegaEnergy,omitempty"`
	CaptureRate         *float64                   `json:"captureRate,omitempty"`
	ChargedMoves        []int                      `json:"chargedMoves"`
	CostumeOverrideEvos []CostumeOverrideEvolution `json:"costumeOverrideEvos,omitempty"`
	DefaultFormID       int                        `json:"defaultFormId"`
	Defense             int                        `json:"defense"`
	EliteChargedMoves   []int                      `json:"eliteChargedMoves"`
	EliteQuickMoves     []int                      `json:"eliteQuickMoves"`
	Evolutions          []Evolution                `json:"evolutions,omitempty"`
	Family              *int                       `json:"family,omitempty"`
	FleeRate            *float64                   `json:"fleeRate,omitempty"`
	Forms               []int                      `json:"forms"`
	GenID               int                        `json:"genId"`
	Generation          string                     `json:"generation"`
	GymDefenderEligible *bool                      `json:"gymDefenderEligible,omitempty"`
	Height              *float64                   `json:"height,omitempty"`
	Jungle              *bool                      `json:"jungle,omitempty"`
	Legendary           bool                       `json:"legendary"`
	Mythic              bool                       `json:"mythic"`
	PokedexID           int                        `json:"pokedexId"`
	PokemonName         string                     `json:"pokemonName"`
	PurificationCandy   *int                       `json:"purificationCandy,omitempty"`
	PurificationDust    *int                       `json:"purificationDust,omitempty"`
	QuickMoves          []int                      `json:"quickMoves"`
	SizeSettings        []SizeSetting              `json:"sizeSettings"`
	Stamina             int                        `json:"stamina"`
	TempEvolutions      []TempEvolution            `json:"tempEvolutions,omitempty"`
	ThirdMoveCandy      *int                       `json:"thirdMoveCandy,omitempty"`
	ThirdMoveStardust   *int                       `json:"thirdMoveStardust,omitempty"`
	Tradable            *bool                      `json:"tradable,omitempty"`
	Transferable        *bool                      `json:"transferable,omitempty"`
	Types               []int                      `json:"types"`
	UltraBeast          *bool                      `json:"ultraBeast,omitempty"`
	Unreleased          *bool                      `json:"unreleased,omitempty"`
	Weight              *float64                   `json:"weight,omitempty"`
}

type PokemonType struct {
	Immunes         []int   `json:"immunes,omitempty"`
	Resistances     []int   `json:"resistances,omitempty"`
	Strengths       []int   `json:"strengths,omitempty"`
	TypeID          *int    `json:"typeId,omitempty"`
	TypeName        *string `json:"typeName,omitempty"`
	VeryWeakAgainst []int   `json:"veryWeakAgainst,omitempty"`
	WeakAgainst     []int   `json:"weakAgainst,omitempty"`
	Weaknesses      []int   `json:"weaknesses,omitempty"`
}

type QuestCondition struct {
	Formatted string `json:"formatted"`
	Proto     string `json:"proto"`
	QuestID   int    `json:"questId"`
}

type QuestRewardType struct {
	Formatted string `json:"formatted"`
	Proto     string `json:"proto"`
	QuestID   int    `json:"questId"`
}

type QuestType struct {
	Formatted string `json:"formatted"`
	Proto     string `json:"proto"`
	QuestID   int    `json:"questId"`
}

type Raid struct {
	Formatted string `json:"formatted"`
	ID        int    `json:"id"`
	Proto     string `json:"proto"`
}

type RouteType struct {
	Formatted string `json:"formatted"`
	ID        int    `json:"id"`
	Proto     string `json:"proto"`
}

type SizeSetting struct {
	Name  string  `json:"name"`
	Value float64 `json:"value"`
}

type Team struct {
	Formatted string `json:"formatted"`
	ID        int    `json:"id"`
	Proto     string `json:"proto"`
}

type TempEvolution struct {
	Attack               *int           `json:"attack,omitempty"`
	Defense              *int           `json:"defense,omitempty"`
	FirstEnergyCost      *int           `json:"firstEnergyCost,omitempty"`
	Height               *float64       `json:"height,omitempty"`
	Stamina              *int           `json:"stamina,omitempty"`
	SubsequentEnergyCost *int           `json:"subsequentEnergyCost,omitempty"`
	TempEvoID            StringOrNumber `json:"tempEvoId"`
	Types                []int          `json:"types,omitempty"`
	Unreleased           *bool          `json:"unreleased,omitempty"`
	Weight               *float64       `json:"weight,omitempty"`
}

type TranslationCategoryPayload map[string]string

type TranslationLocalePayload map[string]TranslationCategoryPayload

type Weather struct {
	Proto       string `json:"proto"`
	Types       []int  `json:"types"`
	WeatherID   int    `json:"weatherId"`
	WeatherName string `json:"weatherName"`
}

type BreadMoveSlot struct {
	MoveLevel *string `json:"moveLevel,omitempty"`
	MoveType  *string `json:"moveType,omitempty"`
}

type CostumeOverrideEvolution struct {
	CostumeID    int    `json:"costumeId"`
	CostumeName  string `json:"costumeName"`
	CostumeProto string `json:"costumeProto"`
}
