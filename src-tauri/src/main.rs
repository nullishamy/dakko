// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    fs::{self, File},
    io::Write,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};

use axum::{extract::Query, response::IntoResponse, routing::get, Extension};
use megalodon::{entities, generator, oauth::TokenData, Megalodon};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use tauri::Manager;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ClientState {
    client_id: String,
    client_secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AuthState {
    token: TokenData,
}

struct AppState {
    client: Mutex<Box<dyn Megalodon + Sync + Send>>,
    client_state: Mutex<Option<ClientState>>,
    auth_state: Mutex<Option<AuthState>>,

    base_url: String,
    redirect_addr: SocketAddr,
    config_dir: Mutex<Option<PathBuf>>,
}

impl AppState {
    fn has_logged_in(&self) -> bool {
        self.auth_state.lock().is_some()
    }
}

fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = app.handle().path_resolver().app_config_dir().unwrap();
    let state = app.state::<AppState>();

    {
        let path = config_dir.join("auth.json");
        let file = File::open(path);
        if let Ok(file) = file {
            let auth_state = serde_json::from_reader::<_, AuthState>(file).unwrap();
            let client = generator(
                megalodon::SNS::Pleroma,
                state.base_url.clone(),
                Some(auth_state.token.access_token.clone()),
                None,
            );

            // TODO: Use this to refresh the tokens once we want to do that
            // tauri::async_runtime::block_on(async {
            // });

            *state.client.lock() = client;
            *state.auth_state.lock() = Some(auth_state);

        }
    }

    {
        let path = config_dir.join("client.json");
        let file = File::open(path);
        if let Ok(file) = file {
            *state.client_state.lock() = Some(serde_json::from_reader(file).unwrap());
        }
    }

    *state.config_dir.lock() = Some(config_dir);
    Ok(())
}

fn save_state(state: &tauri::State<'_, AppState>) {
    let config_dir = state.config_dir.lock();
    let config_dir = config_dir.as_ref().unwrap();
    fs::create_dir_all(&config_dir).unwrap();

    {
        let path = config_dir.join("auth.json");
        let content =
            serde_json::to_string_pretty(&state.auth_state.lock().as_ref().unwrap().clone())
                .unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    {
        let path = config_dir.join("client.json");
        let content =
            serde_json::to_string_pretty(&state.client_state.lock().as_ref().unwrap().clone())
                .unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }
}

fn main() {
    let url = "https://labyrinth.zone/";
    let client = generator(megalodon::SNS::Pleroma, url.to_string(), None, None);

    let context = tauri::generate_context!();
    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9119);

    tauri::Builder::default()
        .setup(setup)
        .invoke_handler(tauri::generate_handler![
            get_instance,
            get_user,
            get_home_timeline,
            get_public_timeline,
            get_conversation,
            post_reply,
            post_status,
            favourite_status,
            get_local_timeline,
            get_notifications,
            login,
            is_logged_in
        ])
        .manage(AppState {
            client: Mutex::new(client),
            client_state: Mutex::new(None),
            auth_state: Mutex::new(None),
            config_dir: Mutex::new(None),
            base_url: url.to_string(),
            redirect_addr: socket_addr,
        })
        .run(context)
        .expect("error while running tauri application");
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Content {
    content: String,
    cw: Option<String>
}

#[tauri::command]
async fn post_reply(post_id: String, reply: Content, state: tauri::State<'_, AppState>) -> Result<entities::Status, ()> {
    assert!(state.has_logged_in());

    let client = state.client.lock();
    let sensitive = reply.cw.is_some();
    let options = megalodon::megalodon::PostStatusInputOptions {
        in_reply_to_id: Some(post_id),
        sensitive: Some(sensitive),
        spoiler_text: reply.cw,
        ..Default::default()
    };

    let res = client.post_status(reply.content, Some(&options)).await.unwrap();
    let output = res.json();
    match output {
        megalodon::megalodon::PostStatusOutput::Status(status) => Ok(status),
        megalodon::megalodon::PostStatusOutput::ScheduledStatus(_) => todo!(),
    }
}

#[tauri::command]
async fn post_status(status: Content, state: tauri::State<'_, AppState>) -> Result<entities::Status, ()> {
    assert!(state.has_logged_in());

    let client = state.client.lock();
    let sensitive = status.cw.is_some();
    let options = megalodon::megalodon::PostStatusInputOptions {
        sensitive: Some(sensitive),
        spoiler_text: status.cw,
        ..Default::default()
    };

    let res = client.post_status(status.content, Some(&options)).await.unwrap();
    let output = res.json();
    match output {
        megalodon::megalodon::PostStatusOutput::Status(status) => Ok(status),
        megalodon::megalodon::PostStatusOutput::ScheduledStatus(_) => todo!(),
    }
}

#[tauri::command]
async fn favourite_status(status_id: String, state: tauri::State<'_, AppState>) -> Result<entities::Status, ()> {
    assert!(state.has_logged_in());

    let client = state.client.lock();
    let res = client.favourite_status(status_id).await.unwrap();
    Ok(res.json())
}

#[tauri::command]
async fn get_instance(state: tauri::State<'_, AppState>) -> Result<entities::Instance, ()> {
    let client = state.client.lock();
    let res = client.get_instance().await.unwrap();
    Ok(res.json())
}

#[tauri::command]
async fn get_user(state: tauri::State<'_, AppState>) -> Result<entities::Account, ()> {
    assert!(state.has_logged_in());

    let client = state.client.lock();
    let res = client.verify_account_credentials().await.unwrap();
    Ok(res.json())
}

#[tauri::command]
async fn get_notifications(state: tauri::State<'_, AppState>) -> Result<Vec<entities::Notification>, ()> {
    assert!(state.has_logged_in());

    let client = state.client.lock();
    let options = megalodon::megalodon::GetNotificationsInputOptions {
        limit: Some(25),
        ..Default::default()
    };

    let res = client.get_notifications(Some(&options)).await.unwrap();
    Ok(res.json())
}

#[tauri::command]
async fn get_home_timeline(start_at: Option<String>, state: tauri::State<'_, AppState>) -> Result<Vec<entities::Status>, ()> {
    assert!(state.has_logged_in());

    let client = state.client.lock();
    let options = megalodon::megalodon::GetHomeTimelineInputOptions {
        limit: Some(10),
        max_id: start_at,
        ..Default::default()
    };

    let res = client.get_home_timeline(Some(&options)).await.unwrap();
    Ok(res.json())
}

#[tauri::command]
async fn get_public_timeline(state: tauri::State<'_, AppState>) -> Result<Vec<entities::Status>, ()> {
    assert!(state.has_logged_in());
    let client = state.client.lock();
    let options = megalodon::megalodon::GetPublicTimelineInputOptions {
        limit: Some(10),
        ..Default::default()
    };

    let res = client.get_public_timeline(Some(&options)).await.unwrap();
    Ok(res.json())
}

#[tauri::command]
async fn get_conversation(entry_point: String, state: tauri::State<'_, AppState>) -> Result<entities::Context, ()> {
    assert!(state.has_logged_in());
    let client = state.client.lock();

    let options = megalodon::megalodon::GetStatusContextInputOptions {
        limit: Some(10),
        ..Default::default()
    };

    let res = client.get_status_context(entry_point, Some(&options)).await.unwrap();
    Ok(res.json())
}

#[tauri::command]
async fn get_local_timeline(state: tauri::State<'_, AppState>) -> Result<Vec<entities::Status>, ()> {
    assert!(state.has_logged_in());
    let client = state.client.lock();
    let options = megalodon::megalodon::GetLocalTimelineInputOptions {
        limit: Some(10),
        ..Default::default()
    };

    let res = client.get_local_timeline(Some(&options)).await.unwrap();
    Ok(res.json())
}

#[tauri::command]
async fn is_logged_in(state: tauri::State<'_, AppState>) -> Result<bool, ()> {
    Ok(state.has_logged_in())
}

#[derive(Serialize)]
struct AuthorizationURL(String);

#[tauri::command]
async fn login(
    state: tauri::State<'_, AppState>,
    handle: tauri::AppHandle,
) -> Result<AuthorizationURL, ()> {
    let options = megalodon::megalodon::AppInputOptions {
        scopes: Some(
            [
                String::from("read"),
                String::from("write"),
                String::from("follow"),
            ]
            .to_vec(),
        ),
        redirect_uris: Some(format!(
            "http://{}:{}/",
            state.redirect_addr.ip(),
            state.redirect_addr.port()
        )),
        ..Default::default()
    };

    let client = state.client.lock();
    match client.register_app(String::from("dakko"), &options).await {
        Ok(app_data) => {
            let client_id = app_data.client_id;
            let client_secret = app_data.client_secret;
            println!("Authorization URL is generated");
            *state.client_state.lock() = Some(ClientState {
                client_id,
                client_secret,
            });

            tauri::async_runtime::spawn(async move { run_server(handle).await });

            return Ok(AuthorizationURL(app_data.url.expect("url to be set")));
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }

    Err(())
}

#[derive(Deserialize)]
struct CallbackQuery {
    code: String,
}

async fn authorize(
    handle: Extension<tauri::AppHandle>,
    query: Query<CallbackQuery>,
) -> impl IntoResponse {
    let state = handle.state::<AppState>();
    let mut client = state.client.lock();
    let client_state_lock = state.client_state.lock();
    let client_state = client_state_lock.as_ref().unwrap();

    let client_id = client_state.client_id.clone();
    let client_secret = client_state.client_secret.clone();

    let code = query.code.trim().to_string();

    match client
        .fetch_access_token(
            client_id,
            client_secret,
            code,
            megalodon::default::NO_REDIRECT.to_string(),
        )
        .await
    {
        Ok(token_data) => {
            *client = generator(
                megalodon::SNS::Pleroma,
                state.base_url.clone(),
                Some(token_data.access_token.clone()),
                None,
            );
            *state.auth_state.lock() = Some(AuthState { token: token_data });
            save_state(&state);
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }

    "authorised"
}

async fn run_server(handle: tauri::AppHandle) -> Result<(), axum::Error> {
    let app = axum::Router::new()
        .route("/", get(authorize))
        .layer(axum::Extension(handle.clone()));

    let listener = tokio::net::TcpListener::bind(&handle.state::<AppState>().redirect_addr.clone())
        .await
        .unwrap();

    println!("Started server");
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
