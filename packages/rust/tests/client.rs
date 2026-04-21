use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use pogo_data::{
    configure_pogo_data, pogo_data, PogoDataClient, DATASET_DEFINITIONS, TRANSLATION_CATEGORIES,
    TRANSLATION_LOCALES,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::oneshot,
    task::JoinHandle,
};

static SHARED_CLIENT_TEST_MUTEX: Mutex<()> = Mutex::new(());

#[tokio::test(flavor = "current_thread")]
async fn new_client_provides_typed_dataset_and_translation_methods() {
    let server = TestServer::spawn(vec![
		(
			"/pokemon/1.json",
			ResponseSpec::json(
				200,
				r#"{"pokemonName":"Bulbasaur","pokedexId":1,"defaultFormId":163,"types":[4,12],"quickMoves":[214],"chargedMoves":[90],"eliteQuickMoves":[],"eliteChargedMoves":[],"forms":[163],"sizeSettings":[],"attack":118,"defense":111,"stamina":128,"legendary":false,"mythic":false,"genId":1,"generation":"Kanto"}"#,
			),
		),
		(
			"/pokemon.json",
			ResponseSpec::json(
				200,
				r#"[{"pokemonName":"Bulbasaur","pokedexId":1,"defaultFormId":163,"types":[4,12],"quickMoves":[214],"chargedMoves":[90],"eliteQuickMoves":[],"eliteChargedMoves":[],"forms":[163],"sizeSettings":[],"attack":118,"defense":111,"stamina":128,"legendary":false,"mythic":false,"genId":1,"generation":"Kanto"}]"#,
			),
		),
		(
			"/translations/en.json",
			ResponseSpec::json(200, r#"{"misc":{"hello":"Hello"}}"#),
		),
		(
			"/translations/en/misc.json",
			ResponseSpec::json(200, r#"{"hello":"Hello"}"#),
		),
	])
	.await;

    let client = PogoDataClient::builder()
        .base_url(server.base_url())
        .header("x-test", "present")
        .expect("header should be valid")
        .build();

    let bulbasaur = client
        .pokemon()
        .get(1)
        .await
        .expect("pokemon lookup should succeed");
    assert_eq!(bulbasaur.pokemon_name, "Bulbasaur");

    let all_pokemon = client
        .pokemon()
        .list()
        .await
        .expect("pokemon list lookup should succeed");
    assert_eq!(all_pokemon.len(), 1);
    assert_eq!(all_pokemon[0].pokedex_id, 1);

    let locale_payload = client
        .translations()
        .get_locale("en")
        .await
        .expect("locale lookup should succeed");
    assert_eq!(locale_payload["misc"]["hello"], "Hello");

    let category_payload = client
        .translations()
        .get_category("en", "misc")
        .await
        .expect("category lookup should succeed");
    assert_eq!(category_payload["hello"], "Hello");

    let requests = server.shutdown().await;
    assert_eq!(requests[0].path, "/pokemon/1.json");
    assert_eq!(
        requests[0].headers.get("x-test").map(String::as_str),
        Some("present"),
    );
}

#[tokio::test(flavor = "current_thread")]
async fn configure_pogo_data_updates_the_shared_client() {
    let _guard = SHARED_CLIENT_TEST_MUTEX
        .lock()
        .expect("shared-client test lock should not be poisoned");

    let server = TestServer::spawn(vec![(
        "/moves/13.json",
        ResponseSpec::json(200, r#"{"moveId":13,"moveName":"Wrap"}"#),
    )])
    .await;

    configure_pogo_data(PogoDataClient::builder().base_url(server.base_url()));
    let shared_client = pogo_data();
    let movement = shared_client
        .moves()
        .get(13)
        .await
        .expect("shared move lookup should succeed");
    assert_eq!(movement.move_name, "Wrap");

    configure_pogo_data(PogoDataClient::builder());
    let requests = server.shutdown().await;
    assert_eq!(requests.len(), 1);
}

#[tokio::test(flavor = "current_thread")]
async fn returns_a_typed_error_on_non_ok_responses() {
    let server = TestServer::spawn(vec![(
        "/pokemon/999999.json",
        ResponseSpec::json(404, r#"{"message":"missing"}"#),
    )])
    .await;

    let client = PogoDataClient::builder()
        .base_url(server.base_url())
        .build();
    let error = client
        .pokemon()
        .get(999999)
        .await
        .expect_err("404 responses should fail");

    assert_eq!(error.status, Some(404));
    assert!(error
        .url
        .as_deref()
        .unwrap_or_default()
        .ends_with("/pokemon/999999.json"));

    let _ = server.shutdown().await;
}

#[tokio::test(flavor = "current_thread")]
async fn returns_a_typed_error_on_invalid_json() {
    let server = TestServer::spawn(vec![(
        "/pokemon/1.json",
        ResponseSpec::json(200, r#"{"broken":"#),
    )])
    .await;

    let client = PogoDataClient::builder()
        .base_url(server.base_url())
        .build();
    let error = client
        .pokemon()
        .get(1)
        .await
        .expect_err("invalid json should fail");

    assert_eq!(error.status, Some(200));
    assert!(error
        .url
        .as_deref()
        .unwrap_or_default()
        .ends_with("/pokemon/1.json"));

    let _ = server.shutdown().await;
}

#[test]
fn publishes_discovered_metadata() {
    assert!(!DATASET_DEFINITIONS.is_empty());
    assert!(!TRANSLATION_LOCALES.is_empty());
    assert!(!TRANSLATION_CATEGORIES.is_empty());
    assert!(TRANSLATION_LOCALES.contains(&"en"));
    assert!(TRANSLATION_CATEGORIES.contains(&"misc"));
}

#[derive(Clone)]
struct ResponseSpec {
    status: u16,
    body: &'static str,
    content_type: &'static str,
}

impl ResponseSpec {
    fn json(status: u16, body: &'static str) -> Self {
        Self {
            status,
            body,
            content_type: "application/json",
        }
    }
}

#[derive(Debug, Clone)]
struct RecordedRequest {
    headers: HashMap<String, String>,
    path: String,
}

struct TestServer {
    base_url: String,
    requests: Arc<Mutex<Vec<RecordedRequest>>>,
    shutdown_sender: Option<oneshot::Sender<()>>,
    task: JoinHandle<()>,
}

impl TestServer {
    async fn spawn(routes: Vec<(&'static str, ResponseSpec)>) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("listener should bind");
        let address = listener
            .local_addr()
            .expect("listener should provide a local address");
        let routes = Arc::new(
            routes
                .into_iter()
                .map(|(path, response)| (path.to_owned(), response))
                .collect::<HashMap<_, _>>(),
        );
        let requests = Arc::new(Mutex::new(Vec::new()));
        let requests_for_task = Arc::clone(&requests);
        let routes_for_task = Arc::clone(&routes);
        let (shutdown_sender, mut shutdown_receiver) = oneshot::channel();

        let task = tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = &mut shutdown_receiver => break,
                    accept_result = listener.accept() => {
                        let (mut socket, _) = match accept_result {
                            Ok(value) => value,
                            Err(_) => break,
                        };

                        let mut buffer = Vec::new();
                        loop {
                            let mut chunk = [0_u8; 1024];
                            let read_count = match socket.read(&mut chunk).await {
                                Ok(value) => value,
                                Err(_) => break,
                            };

                            if read_count == 0 {
                                break;
                            }

                            buffer.extend_from_slice(&chunk[..read_count]);
                            if buffer.windows(4).any(|window| window == b"\r\n\r\n") {
                                break;
                            }
                        }

                        let request_text = String::from_utf8_lossy(&buffer);
                        let mut lines = request_text.split("\r\n");
                        let request_line = lines.next().unwrap_or_default();
                        let path = request_line
                            .split_whitespace()
                            .nth(1)
                            .unwrap_or("/")
                            .to_owned();

                        let mut headers = HashMap::new();
                        for line in lines.by_ref() {
                            if line.is_empty() {
                                break;
                            }

                            if let Some((name, value)) = line.split_once(':') {
                                headers.insert(
                                    name.trim().to_ascii_lowercase(),
                                    value.trim().to_owned(),
                                );
                            }
                        }

                        requests_for_task
                            .lock()
                            .expect("requests lock should not be poisoned")
                            .push(RecordedRequest { headers, path: path.clone() });

                        let response = routes_for_task
                            .get(&path)
                            .cloned()
                            .unwrap_or_else(|| ResponseSpec::json(404, r#"{"message":"missing"}"#));
                        let status_text = match response.status {
                            200 => "OK",
                            404 => "Not Found",
                            _ => "Error",
                        };
                        let payload = format!(
                            "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            response.status,
                            status_text,
                            response.content_type,
                            response.body.len(),
                            response.body,
                        );

                        let _ = socket.write_all(payload.as_bytes()).await;
                    }
                }
            }
        });

        Self {
            base_url: format!("http://{}", address),
            requests,
            shutdown_sender: Some(shutdown_sender),
            task,
        }
    }

    fn base_url(&self) -> String {
        self.base_url.clone()
    }

    async fn shutdown(mut self) -> Vec<RecordedRequest> {
        if let Some(shutdown_sender) = self.shutdown_sender.take() {
            let _ = shutdown_sender.send(());
        }
        let _ = self.task.await;
        let requests = self
            .requests
            .lock()
            .expect("requests lock should not be poisoned");

        requests.clone()
    }
}
