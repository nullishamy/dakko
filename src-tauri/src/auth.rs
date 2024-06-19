use std::{fs::{self, File}, io::Write};

use axum::{extract::Query, response::IntoResponse, routing::get, Extension};
use megalodon::generator;
use serde::{Deserialize, Serialize};
use tauri::Manager;

use crate::state::{AppState, AuthState, ClientState};

#[derive(Serialize)]
pub enum LoginState {
    LoggedIn,
    LoggedOut,
}

pub fn save_state(state: &tauri::State<'_, AppState>) {
    let config_dir = state.config_dir.read();
    let config_dir = config_dir.as_ref().unwrap();
    fs::create_dir_all(&config_dir).unwrap();

    {
        let path = config_dir.join("auth.json");
        let content =
            serde_json::to_string_pretty(&state.auth_state.read().as_ref().unwrap().clone())
                .unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    {
        let path = config_dir.join("client.json");
        let content =
            serde_json::to_string_pretty(&state.client_state.read().as_ref().unwrap().clone())
                .unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }
}

#[tauri::command]
pub async fn login_state(state: tauri::State<'_, AppState>) -> Result<LoginState, ()> {
    if state.has_logged_in() {
        Ok(LoginState::LoggedIn)
    } else {
        Ok(LoginState::LoggedOut)
    }
}

#[derive(Serialize)]
pub struct AuthorizationURL(String);

#[tauri::command]
pub async fn login(
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

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let mut client_state = state.client_state.write();
    let client_state = client_state.as_mut().unwrap();

    match client.register_app(String::from("dakko"), &options).await {
        Ok(app_data) => {
            let client_id = app_data.client_id;
            let client_secret = app_data.client_secret;
            eprintln!("Authorization URL is generated {:#?}", app_data.url);
            *client_state = ClientState {
                client_id,
                client_secret,
                base_url: client_state.base_url.clone(),
            };

            tauri::async_runtime::spawn(async move { run_server(handle).await });

            return Ok(AuthorizationURL(app_data.url.expect("url to be set")));
        }
        Err(err) => {
            eprintln!("{:#?}", err);
        }
    }

    Err(())
}

#[derive(Deserialize)]
pub struct CallbackQuery {
    code: String,
}

pub async fn authorize(
    handle: Extension<tauri::AppHandle>,
    query: Query<CallbackQuery>,
) -> impl IntoResponse {
    let state = handle.state::<AppState>();

    let mut client_lock = state.client.write();
    let client = client_lock.as_mut().unwrap();

    let client_state_lock = state.client_state.read();
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
                client_state.base_url.clone(),
                Some(token_data.access_token.clone()),
                None,
            );
            *state.auth_state.write() = Some(AuthState { token: token_data });

            // Drop our locks before save_state acquires them
            drop(client_lock);
            drop(client_state_lock);

            save_state(&state);
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }

    handle.emit_all("auth-complete", ()).unwrap();

    "authorisation complete"
}

pub async fn run_server(handle: tauri::AppHandle) -> Result<(), axum::Error> {
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
