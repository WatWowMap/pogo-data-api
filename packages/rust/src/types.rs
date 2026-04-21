use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum StringOrNumber {
    String(String),
    Number(i64),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseStats {
    pub attack: Option<i64>,
    pub defense: Option<i64>,
    pub height: Option<f64>,
    pub stamina: Option<i64>,
    pub types: Option<Vec<i64>>,
    pub weight: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Costume {
    pub id: i64,
    pub name: String,
    pub no_evolve: bool,
    pub proto: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Evolution {
    pub candy_cost: Option<i64>,
    pub evo_id: Option<i64>,
    pub form_id: Option<i64>,
    pub gender_requirement: Option<StringOrNumber>,
    pub item_requirement: Option<i64>,
    pub must_be_buddy: Option<bool>,
    pub only_daytime: Option<bool>,
    pub only_nighttime: Option<bool>,
    pub quest_requirement: Option<String>,
    pub trade_bonus: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormChangeBonusAttribute {
    pub bread_mode: Option<String>,
    pub clear_bread_mode: Option<bool>,
    pub max_moves: Option<Vec<BreadMoveSlot>>,
    pub target_form: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormChangeBreadMoveRequirement {
    pub move_level: Option<String>,
    pub move_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormChangeComponentLocationCardSettings {
    pub base_pokemon_location_card: Option<i64>,
    pub component_pokemon_location_card: Option<i64>,
    pub fusion_pokemon_location_card: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormChangeComponentPokemonSettings {
    pub component_candy_cost: Option<i64>,
    pub family_id: Option<i64>,
    pub form_change_type: Option<String>,
    pub form_id: Option<i64>,
    pub fusion_move_1: Option<i64>,
    pub fusion_move_2: Option<i64>,
    pub location_card_settings: Option<Vec<FormChangeComponentLocationCardSettings>>,
    pub pokedex_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormChangeLocationCardSettings {
    pub existing_location_card: Option<i64>,
    pub replacement_location_card: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormChangeMoveReassignment {
    pub charged_moves: Option<Vec<MoveReassignment>>,
    pub quick_moves: Option<Vec<MoveReassignment>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormChangeMoveRequirement {
    pub required_moves: Option<Vec<i64>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormChangeQuestRequirement {
    pub description: Option<String>,
    pub quest_requirement: Option<String>,
    pub target: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormChange {
    pub available_forms: Option<Vec<i64>>,
    pub candy_cost: Option<i64>,
    pub component_pokemon_settings: Option<FormChangeComponentPokemonSettings>,
    pub form_change_bonus_attributes: Option<Vec<FormChangeBonusAttribute>>,
    pub item_cost_count: Option<i64>,
    pub item_requirement: Option<i64>,
    pub location_card_settings: Option<Vec<FormChangeLocationCardSettings>>,
    pub move_reassignment: Option<FormChangeMoveReassignment>,
    pub priority: Option<i64>,
    pub quest_requirements: Option<Vec<FormChangeQuestRequirement>>,
    pub required_bread_moves: Option<Vec<FormChangeBreadMoveRequirement>>,
    pub required_charged_moves: Option<Vec<FormChangeMoveRequirement>>,
    pub required_quick_moves: Option<Vec<FormChangeMoveRequirement>>,
    pub stardust_cost: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    pub attack: Option<i64>,
    pub bonus_candy_capture: Option<i64>,
    pub bonus_stardust_capture: Option<i64>,
    pub charged_moves: Option<Vec<i64>>,
    pub costume_override_evos: Option<Vec<CostumeOverrideEvolution>>,
    pub defense: Option<i64>,
    pub elite_charged_moves: Option<Vec<i64>>,
    pub elite_quick_moves: Option<Vec<i64>>,
    pub evolutions: Option<Vec<Evolution>>,
    pub family: Option<i64>,
    pub form_changes: Option<Vec<FormChange>>,
    pub form_id: Option<i64>,
    pub form_name: Option<String>,
    pub gmax_move: Option<i64>,
    pub height: Option<f64>,
    pub is_costume: Option<bool>,
    pub little: Option<bool>,
    pub proto: Option<String>,
    pub purification_candy: Option<i64>,
    pub purification_dust: Option<i64>,
    pub quick_moves: Option<Vec<i64>>,
    pub size_settings: Option<Vec<SizeSetting>>,
    pub stamina: Option<i64>,
    pub temp_evolutions: Option<Vec<TempEvolution>>,
    pub tradable: Option<bool>,
    pub transferable: Option<bool>,
    pub types: Option<Vec<i64>>,
    pub weight: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invasion {
    pub active: bool,
    pub encounters: Option<Vec<InvasionEncounter>>,
    pub first_reward: bool,
    pub gender: StringOrNumber,
    pub grunt: String,
    pub id: i64,
    pub proto: String,
    pub second_reward: bool,
    pub third_reward: bool,
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvasionEncounter {
    pub form_id: Option<i64>,
    pub id: Option<i64>,
    pub position: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub category: String,
    pub item_id: i64,
    pub item_name: String,
    pub min_trainer_level: i64,
    pub proto: String,
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationCard {
    pub card_type: Option<String>,
    pub formatted: String,
    pub id: i64,
    pub image_url: Option<String>,
    pub proto: String,
    pub vfx_address: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Move {
    pub duration_ms: Option<i64>,
    pub energy_delta: Option<i64>,
    pub fast: Option<bool>,
    pub move_id: i64,
    pub move_name: String,
    pub proto: Option<String>,
    pub pvp_buffs: Option<Vec<MoveBuff>>,
    pub pvp_duration_turns: Option<i64>,
    pub pvp_energy_delta: Option<i64>,
    pub pvp_power: Option<i64>,
    pub power: Option<i64>,
    pub r#type: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveBuff {
    pub attacker_attack_stat_stage_change: Option<i64>,
    pub attacker_defense_stat_stage_change: Option<i64>,
    pub buff_activation_chance: f64,
    pub target_attack_stat_stage_change: Option<i64>,
    pub target_defense_stat_stage_change: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveReassignment {
    pub existing_moves: Option<Vec<i64>>,
    pub replacement_moves: Option<Vec<i64>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pokemon {
    pub attack: i64,
    pub buddy_distance: Option<i64>,
    pub buddy_group_number: Option<i64>,
    pub buddy_mega_energy: Option<i64>,
    pub capture_rate: Option<f64>,
    pub charged_moves: Vec<i64>,
    pub costume_override_evos: Option<Vec<CostumeOverrideEvolution>>,
    pub default_form_id: i64,
    pub defense: i64,
    pub elite_charged_moves: Vec<i64>,
    pub elite_quick_moves: Vec<i64>,
    pub evolutions: Option<Vec<Evolution>>,
    pub family: Option<i64>,
    pub flee_rate: Option<f64>,
    pub forms: Vec<i64>,
    pub gen_id: i64,
    pub generation: String,
    pub gym_defender_eligible: Option<bool>,
    pub height: Option<f64>,
    pub jungle: Option<bool>,
    pub legendary: bool,
    pub mythic: bool,
    pub pokedex_id: i64,
    pub pokemon_name: String,
    pub purification_candy: Option<i64>,
    pub purification_dust: Option<i64>,
    pub quick_moves: Vec<i64>,
    pub size_settings: Vec<SizeSetting>,
    pub stamina: i64,
    pub temp_evolutions: Option<Vec<TempEvolution>>,
    pub third_move_candy: Option<i64>,
    pub third_move_stardust: Option<i64>,
    pub tradable: Option<bool>,
    pub transferable: Option<bool>,
    pub types: Vec<i64>,
    pub ultra_beast: Option<bool>,
    pub unreleased: Option<bool>,
    pub weight: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokemonType {
    pub immunes: Option<Vec<i64>>,
    pub resistances: Option<Vec<i64>>,
    pub strengths: Option<Vec<i64>>,
    pub type_id: Option<i64>,
    pub type_name: Option<String>,
    pub very_weak_against: Option<Vec<i64>>,
    pub weak_against: Option<Vec<i64>>,
    pub weaknesses: Option<Vec<i64>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestCondition {
    pub formatted: String,
    pub proto: String,
    pub quest_id: i64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestRewardType {
    pub formatted: String,
    pub proto: String,
    pub quest_id: i64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestType {
    pub formatted: String,
    pub proto: String,
    pub quest_id: i64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Raid {
    pub formatted: String,
    pub id: i64,
    pub proto: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteType {
    pub formatted: String,
    pub id: i64,
    pub proto: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SizeSetting {
    pub name: String,
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub formatted: String,
    pub id: i64,
    pub proto: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TempEvolution {
    pub attack: Option<i64>,
    pub defense: Option<i64>,
    pub first_energy_cost: Option<i64>,
    pub height: Option<f64>,
    pub stamina: Option<i64>,
    pub subsequent_energy_cost: Option<i64>,
    pub temp_evo_id: StringOrNumber,
    pub types: Option<Vec<i64>>,
    pub unreleased: Option<bool>,
    pub weight: Option<f64>,
}

pub type TranslationCategoryPayload = BTreeMap<String, String>;

pub type TranslationLocalePayload = BTreeMap<String, TranslationCategoryPayload>;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weather {
    pub proto: String,
    pub types: Vec<i64>,
    pub weather_id: i64,
    pub weather_name: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BreadMoveSlot {
    pub move_level: Option<String>,
    pub move_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CostumeOverrideEvolution {
    pub costume_id: i64,
    pub costume_name: String,
    pub costume_proto: String,
}
