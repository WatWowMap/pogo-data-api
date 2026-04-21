use std::{
    error::Error as StdError,
    fmt::{self, Display},
    marker::PhantomData,
    sync::{Arc, LazyLock, RwLock},
};

use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client, StatusCode,
};
use serde::de::DeserializeOwned;

use crate::types::{
    Costume, Form, Invasion, Item, LocationCard, Move, Pokemon, PokemonType, QuestCondition,
    QuestRewardType, QuestType, Raid, RouteType, Team, TranslationCategoryPayload,
    TranslationLocalePayload, Weather,
};

pub const DEFAULT_BASE_URL: &str =
    "https://raw.githubusercontent.com/WatWowMap/pogo-data-api/refs/heads/main/data/v1";

static SHARED_CLIENT: LazyLock<RwLock<Arc<PogoDataClient>>> =
    LazyLock::new(|| RwLock::new(Arc::new(PogoDataClient::builder().build())));

#[derive(Clone, Debug)]
pub struct PogoDataClient {
    state: Arc<ClientState>,
}

#[derive(Clone, Debug)]
pub struct DatasetClient<T> {
    state: Arc<ClientState>,
    dataset_path: &'static str,
    _marker: PhantomData<T>,
}

#[derive(Clone, Debug)]
pub struct TranslationClient {
    state: Arc<ClientState>,
}

#[derive(Clone, Debug)]
pub struct PogoDataClientBuilder {
    base_url: String,
    headers: HeaderMap,
    http_client: Option<Client>,
}

#[derive(Debug)]
pub struct PogoDataError {
    message: String,
    pub status: Option<u16>,
    pub url: Option<String>,
    source: Option<Box<dyn StdError + Send + Sync>>,
}

#[derive(Debug)]
struct ClientState {
    base_url: String,
    headers: HeaderMap,
    http_client: Client,
}

impl PogoDataClient {
    pub fn builder() -> PogoDataClientBuilder {
        PogoDataClientBuilder::default()
    }

    pub fn pokemon(&self) -> DatasetClient<Pokemon> {
        self.dataset_client("pokemon")
    }

    pub fn moves(&self) -> DatasetClient<Move> {
        self.dataset_client("moves")
    }

    pub fn forms(&self) -> DatasetClient<Form> {
        self.dataset_client("forms")
    }

    pub fn costumes(&self) -> DatasetClient<Costume> {
        self.dataset_client("costumes")
    }

    pub fn invasions(&self) -> DatasetClient<Invasion> {
        self.dataset_client("invasions")
    }

    pub fn items(&self) -> DatasetClient<Item> {
        self.dataset_client("items")
    }

    pub fn location_cards(&self) -> DatasetClient<LocationCard> {
        self.dataset_client("location-cards")
    }

    pub fn quest_conditions(&self) -> DatasetClient<QuestCondition> {
        self.dataset_client("quest-conditions")
    }

    pub fn quest_reward_types(&self) -> DatasetClient<QuestRewardType> {
        self.dataset_client("quest-reward-types")
    }

    pub fn quest_types(&self) -> DatasetClient<QuestType> {
        self.dataset_client("quest-types")
    }

    pub fn raids(&self) -> DatasetClient<Raid> {
        self.dataset_client("raids")
    }

    pub fn route_types(&self) -> DatasetClient<RouteType> {
        self.dataset_client("route-types")
    }

    pub fn teams(&self) -> DatasetClient<Team> {
        self.dataset_client("teams")
    }

    pub fn types(&self) -> DatasetClient<PokemonType> {
        self.dataset_client("types")
    }

    pub fn weather(&self) -> DatasetClient<Weather> {
        self.dataset_client("weather")
    }

    pub fn translations(&self) -> TranslationClient {
        TranslationClient {
            state: Arc::clone(&self.state),
        }
    }

    fn dataset_client<T>(&self, dataset_path: &'static str) -> DatasetClient<T> {
        DatasetClient {
            state: Arc::clone(&self.state),
            dataset_path,
            _marker: PhantomData,
        }
    }
}

impl PogoDataClientBuilder {
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = normalize_base_url(base_url.into());
        self
    }

    pub fn headers(mut self, headers: HeaderMap) -> Self {
        self.headers = headers;
        self
    }

    pub fn header(
        mut self,
        name: impl AsRef<str>,
        value: impl AsRef<str>,
    ) -> Result<Self, PogoDataError> {
        let header_name = HeaderName::try_from(name.as_ref()).map_err(|error| {
            PogoDataError::new(
                format!("Invalid header name {}.", name.as_ref()),
                None,
                None,
                Some(error),
            )
        })?;
        let header_value = HeaderValue::try_from(value.as_ref()).map_err(|error| {
            PogoDataError::new(
                format!("Invalid header value for {}.", name.as_ref()),
                None,
                None,
                Some(error),
            )
        })?;

        self.headers.insert(header_name, header_value);
        Ok(self)
    }

    pub fn http_client(mut self, http_client: Client) -> Self {
        self.http_client = Some(http_client);
        self
    }

    pub fn build(self) -> PogoDataClient {
        PogoDataClient {
            state: Arc::new(ClientState {
                base_url: normalize_base_url(self.base_url),
                headers: self.headers,
                http_client: self.http_client.unwrap_or_default(),
            }),
        }
    }
}

impl Default for PogoDataClientBuilder {
    fn default() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_owned(),
            headers: HeaderMap::new(),
            http_client: None,
        }
    }
}

impl<T> DatasetClient<T>
where
    T: DeserializeOwned,
{
    pub async fn get(&self, id: impl Display) -> Result<T, PogoDataError> {
        self.fetch_json(&format!(
            "{}/{}.json",
            self.dataset_path,
            encode_path_segment(&id.to_string()),
        ))
        .await
    }

    pub async fn list(&self) -> Result<Vec<T>, PogoDataError> {
        self.fetch_json(&format!("{}.json", self.dataset_path))
            .await
    }

    async fn fetch_json<TValue>(&self, path: &str) -> Result<TValue, PogoDataError>
    where
        TValue: DeserializeOwned,
    {
        fetch_json(&self.state, path).await
    }
}

impl TranslationClient {
    pub async fn get_category(
        &self,
        locale: &str,
        category: &str,
    ) -> Result<TranslationCategoryPayload, PogoDataError> {
        fetch_json(
            &self.state,
            &format!(
                "translations/{}/{}.json",
                encode_path_segment(locale),
                encode_path_segment(category),
            ),
        )
        .await
    }

    pub async fn get_locale(
        &self,
        locale: &str,
    ) -> Result<TranslationLocalePayload, PogoDataError> {
        fetch_json(
            &self.state,
            &format!("translations/{}.json", encode_path_segment(locale)),
        )
        .await
    }
}

impl PogoDataError {
    fn new(
        message: String,
        status: Option<StatusCode>,
        url: Option<String>,
        source: Option<impl StdError + Send + Sync + 'static>,
    ) -> Self {
        Self {
            message,
            status: status.map(|status| status.as_u16()),
            url,
            source: source.map(|source| Box::new(source) as _),
        }
    }
}

impl Display for PogoDataError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl StdError for PogoDataError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source.as_deref().map(|source| source as _)
    }
}

pub fn configure_pogo_data(builder: PogoDataClientBuilder) {
    let mut shared_client = SHARED_CLIENT
        .write()
        .expect("shared pogo_data client lock should not be poisoned");
    *shared_client = Arc::new(builder.build());
}

pub fn pogo_data() -> Arc<PogoDataClient> {
    SHARED_CLIENT
        .read()
        .expect("shared pogo_data client lock should not be poisoned")
        .clone()
}

fn normalize_base_url(base_url: String) -> String {
    let trimmed = base_url.trim_end_matches('/');
    if trimmed.is_empty() {
        DEFAULT_BASE_URL.to_owned()
    } else {
        trimmed.to_owned()
    }
}

fn build_url(base_url: &str, path: &str) -> String {
    format!(
        "{}/{}",
        normalize_base_url(base_url.to_owned()),
        path.trim_start_matches('/'),
    )
}

fn encode_path_segment(value: &str) -> String {
    let mut encoded = String::new();

    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                encoded.push(char::from(byte))
            }
            _ => {
                let _ = fmt::Write::write_fmt(&mut encoded, format_args!("%{byte:02X}"));
            }
        }
    }

    encoded
}

async fn fetch_json<T>(state: &ClientState, path: &str) -> Result<T, PogoDataError>
where
    T: DeserializeOwned,
{
    let url = build_url(&state.base_url, path);
    let mut request = state.http_client.get(&url);

    for (header_name, header_value) in &state.headers {
        request = request.header(header_name, header_value);
    }

    let response = request.send().await.map_err(|error| {
        PogoDataError::new(
            format!("Request failed for {url}."),
            None,
            Some(url.clone()),
            Some(error),
        )
    })?;

    let status = response.status();
    if !status.is_success() {
        return Err(PogoDataError::new(
            format!("Request failed for {url} with status {}.", status.as_u16()),
            Some(status),
            Some(url),
            None::<reqwest::Error>,
        ));
    }

    response.json::<T>().await.map_err(|error| {
        PogoDataError::new(
            format!("Failed to parse JSON from {url}."),
            Some(status),
            Some(url),
            Some(error),
        )
    })
}
