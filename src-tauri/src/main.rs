// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    fs::{self, File},
    io::Write,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
    time::{Duration, Instant},
};

use axum::{extract::Query, response::IntoResponse, routing::get, Extension};
use megalodon::{entities, generator, megalodon::FollowRequestOutput, oauth::TokenData, Megalodon};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use tauri::Manager;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ClientState {
    client_id: String,
    client_secret: String,
    base_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AuthState {
    token: TokenData,
}

struct AppState {
    client_state: RwLock<Option<ClientState>>,
    auth_state: RwLock<Option<AuthState>>,
    client: RwLock<Option<Box<dyn Megalodon + Sync + Send>>>,

    redirect_addr: SocketAddr,
    config_dir: RwLock<Option<PathBuf>>,
}

impl AppState {
    fn has_logged_in(&self) -> bool {
        let auth_lock = self
            .auth_state
            .try_read_until(Instant::now().checked_add(Duration::from_secs(10)).unwrap());
        let client_lock = self
            .client
            .try_read_until(Instant::now().checked_add(Duration::from_secs(10)).unwrap());

        if auth_lock.is_none() {
            panic!("could not acquire auth lock in time")
        }

        if client_lock.is_none() {
            panic!("could not acquire client lock in time")
        }

        auth_lock.unwrap().is_some() && client_lock.unwrap().is_some()
    }
}

fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = app.handle().path_resolver().app_config_dir().unwrap();
    let state = app.state::<AppState>();

    {
        let path = config_dir.join("client.json");
        let file = File::open(path);
        if let Ok(file) = file {
            let client_state = serde_json::from_reader::<_, ClientState>(file).unwrap();
            let url = client_state.base_url.clone();
            *state.client_state.write() = Some(client_state);

            let mut client = generator(megalodon::SNS::Pleroma, url.clone(), None, None);

            {
                let path = config_dir.join("auth.json");
                let file = File::open(path);
                if let Ok(file) = file {
                    let auth_state = serde_json::from_reader::<_, AuthState>(file).unwrap();
                    client = generator(
                        megalodon::SNS::Pleroma,
                        url,
                        Some(auth_state.token.access_token.clone()),
                        None,
                    );

                    // TODO: Use this to refresh the tokens once we want to do that
                    // tauri::async_runtime::block_on(async {
                    // });

                    *state.auth_state.write() = Some(auth_state);
                }
            }

            *state.client.write() = Some(client);
        } else {
            eprintln!("No client.json found, cannot login");
        }
    }

    *state.config_dir.write() = Some(config_dir);
    Ok(())
}

fn save_state(state: &tauri::State<'_, AppState>) {
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

fn main() {
    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9119);

    tauri::Builder::default()
        .setup(setup)
        .invoke_handler(tauri::generate_handler![
            get_instance,
            get_statuses,
            get_user,
            get_home_timeline,
            get_public_timeline,
            get_conversation,
            post_reply,
            boost_status,
            post_status,
            favourite_status,
            get_local_timeline,
            get_notifications,
            login,
            login_state,
            set_instance,
            get_relationships,
            block_user,
            unblock_user,
            mute_user,
            unmute_user,
            follow_user,
            unfollow_user,
            get_follow_requests,
            get_markers,
            save_markers,
            get_status,
            bookmark_status,
            unbookmark_status,
            get_bookmarks,
            get_emojis,
            get_home_catchup,
            accept_follow_request,
            deny_follow_request
        ])
        .manage(AppState {
            client: RwLock::new(None),
            client_state: RwLock::new(None),
            auth_state: RwLock::new(None),
            config_dir: RwLock::new(None),
            redirect_addr: socket_addr,
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Content {
    content: String,
    cw: Option<String>,
    visibility: entities::status::StatusVisibility,
    quoting: Option<String>,
}

#[tauri::command]
async fn post_reply(
    post_id: String,
    reply: Content,
    state: tauri::State<'_, AppState>,
) -> Result<entities::Status, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let sensitive = reply.cw.is_some();
    let options = megalodon::megalodon::PostStatusInputOptions {
        in_reply_to_id: Some(post_id),
        sensitive: Some(sensitive),
        spoiler_text: reply.cw,
        visibility: Some(reply.visibility),
        ..Default::default()
    };

    let res = client
        .post_status(reply.content, Some(&options))
        .await
        .map_err(|_| ())?;
    let output = res.json();
    match output {
        megalodon::megalodon::PostStatusOutput::Status(status) => Ok(status),
        megalodon::megalodon::PostStatusOutput::ScheduledStatus(_) => todo!(),
    }
}

#[tauri::command]
async fn post_status(
    status: Content,
    state: tauri::State<'_, AppState>,
) -> Result<entities::Status, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let sensitive = status.cw.is_some();
    let options = megalodon::megalodon::PostStatusInputOptions {
        sensitive: Some(sensitive),
        spoiler_text: status.cw,
        visibility: Some(status.visibility),
        quote_id: status.quoting,
        ..Default::default()
    };

    let res = client
        .post_status(status.content, Some(&options))
        .await
        .map_err(|_| ())?;
    let output = res.json();
    match output {
        megalodon::megalodon::PostStatusOutput::Status(status) => Ok(status),
        megalodon::megalodon::PostStatusOutput::ScheduledStatus(_) => todo!(),
    }
}

#[tauri::command]
async fn favourite_status(
    id: String,
    state: tauri::State<'_, AppState>,
) -> Result<entities::Status, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let res = client.favourite_status(id).await.map_err(|_| ())?;
    Ok(res.json())
}

#[tauri::command]
async fn get_bookmarks(state: tauri::State<'_, AppState>) -> Result<Vec<entities::Status>, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let res = client.get_bookmarks(None).await.map_err(|_| ())?;
    Ok(res.json())
}

#[tauri::command]
async fn get_emojis(state: tauri::State<'_, AppState>) -> Result<Vec<entities::Emoji>, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let res = client.get_instance_custom_emojis().await.map_err(|_| ())?;
    Ok(res.json())
}

#[tauri::command]
async fn get_follow_requests(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<entities::Account>, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let res = client.get_follow_requests(None).await.map_err(|_| ())?;
    let requests = res.json();
    let requests = requests
        .into_iter()
        .map(|r| match r {
            FollowRequestOutput::Account(d) => d,
            FollowRequestOutput::FollowRequest(_) => todo!("implement this type"),
        })
        .collect();

    Ok(requests)
}

#[tauri::command]
async fn get_statuses(
    id: String,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<entities::Status>, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let options = megalodon::megalodon::GetAccountStatusesInputOptions {
        limit: Some(25),
        ..Default::default()
    };

    let res = client
        .get_account_statuses(id, Some(&options))
        .await
        .map_err(|_| ())?;
    Ok(res.json())
}

#[tauri::command]
async fn get_markers(
    timelines: Vec<String>,
    state: tauri::State<'_, AppState>,
) -> Result<entities::Marker, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let res = client.get_markers(timelines).await.map_err(|_| ())?;
    Ok(res.json())
}

#[tauri::command]
async fn get_status(id: String, state: tauri::State<'_, AppState>) -> Result<entities::Status, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let res = client.get_status(id).await.map_err(|_| ())?;
    Ok(res.json())
}

#[tauri::command]
async fn save_markers(
    last_post_in_home: String,
    last_notification: String,
    state: tauri::State<'_, AppState>,
) -> Result<entities::Marker, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let options = megalodon::megalodon::SaveMarkersInputOptions {
        home: Some(megalodon::megalodon::Marker {
            last_reading_id: last_post_in_home,
        }),
        notifications: Some(megalodon::megalodon::Marker {
            last_reading_id: last_notification,
        }),
    };

    let res = client.save_markers(Some(&options)).await.unwrap();
    Ok(res.json())
}

#[tauri::command]
async fn get_relationships(
    account_ids: Vec<String>,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<entities::Relationship>, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let res = client
        .get_relationships(account_ids)
        .await
        .map_err(|_| ())?;
    Ok(res.json())
}

#[tauri::command]
async fn boost_status(
    id: String,
    state: tauri::State<'_, AppState>,
) -> Result<entities::Status, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let res = client.reblog_status(id).await.unwrap();
    Ok(res.json())
}

#[tauri::command]
async fn get_instance(state: tauri::State<'_, AppState>) -> Result<entities::Instance, ()> {
    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let res = client.get_instance().await.map_err(|_| ())?;
    Ok(res.json())
}

#[tauri::command]
async fn get_user(state: tauri::State<'_, AppState>) -> Result<entities::Account, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let res = client.verify_account_credentials().await.map_err(|_| ())?;
    Ok(res.json())
}

#[tauri::command]
async fn bookmark_status(
    id: String,
    state: tauri::State<'_, AppState>,
) -> Result<entities::Status, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let res = client.bookmark_status(id).await.unwrap();
    Ok(res.json())
}

#[tauri::command]
async fn unbookmark_status(
    id: String,
    state: tauri::State<'_, AppState>,
) -> Result<entities::Status, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let res = client.unbookmark_status(id).await.unwrap();
    Ok(res.json())
}

#[tauri::command]
async fn accept_follow_request(
    id: String,
    state: tauri::State<'_, AppState>,
) -> Result<entities::Relationship, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let res = client.accept_follow_request(id).await.map_err(|_| ())?;
    Ok(res.json())
}

#[tauri::command]
async fn deny_follow_request(
    id: String,
    state: tauri::State<'_, AppState>,
) -> Result<entities::Relationship, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let res = client.reject_follow_request(id).await.map_err(|_| ())?;
    Ok(res.json())
}

#[tauri::command]
async fn follow_user(
    id: String,
    state: tauri::State<'_, AppState>,
) -> Result<entities::Relationship, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let res = client.follow_account(id, None).await.map_err(|_| ())?;
    Ok(res.json())
}

#[tauri::command]
async fn unfollow_user(
    id: String,
    state: tauri::State<'_, AppState>,
) -> Result<entities::Relationship, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let res = client.unfollow_account(id).await.map_err(|_| ())?;
    Ok(res.json())
}

#[tauri::command]
async fn block_user(
    id: String,
    state: tauri::State<'_, AppState>,
) -> Result<entities::Relationship, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let res = client.block_account(id).await.map_err(|_| ())?;
    Ok(res.json())
}

#[tauri::command]
async fn unblock_user(
    id: String,
    state: tauri::State<'_, AppState>,
) -> Result<entities::Relationship, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let res = client.unblock_account(id).await.map_err(|_| ())?;
    Ok(res.json())
}

#[tauri::command]
async fn mute_user(
    id: String,
    state: tauri::State<'_, AppState>,
) -> Result<entities::Relationship, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let res = client.mute_account(id, false).await.map_err(|_| ())?;
    Ok(res.json())
}

#[tauri::command]
async fn unmute_user(
    id: String,
    state: tauri::State<'_, AppState>,
) -> Result<entities::Relationship, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let res = client.unmute_account(id).await.map_err(|_| ())?;
    Ok(res.json())
}

#[tauri::command]
async fn get_notifications(
    since: Option<String>,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<entities::Notification>, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let options = megalodon::megalodon::GetNotificationsInputOptions {
        limit: Some(25),
        since_id: since,
        ..Default::default()
    };

    let res = client
        .get_notifications(Some(&options))
        .await
        .map_err(|_| ())?;
    Ok(res.json())
}

#[tauri::command]
async fn get_home_catchup(
    since_id: String,
    state: tauri::State<'_, AppState>,
) -> Result<usize, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let options = megalodon::megalodon::GetHomeTimelineInputOptions {
        since_id: Some(since_id),
        ..Default::default()
    };

    let res = client
        .get_home_timeline(Some(&options))
        .await
        .map_err(|_| ())?;
    Ok(res.json().len())
}

#[tauri::command]
async fn get_home_timeline(
    start_at: Option<String>,
    limit: u32,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<entities::Status>, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let options = megalodon::megalodon::GetHomeTimelineInputOptions {
        limit: Some(limit),
        max_id: start_at,
        ..Default::default()
    };

    let res = client
        .get_home_timeline(Some(&options))
        .await
        .map_err(|_| ())?;
    Ok(res.json())
}

#[tauri::command]
async fn get_public_timeline(
    state: tauri::State<'_, AppState>,
    limit: u32,
) -> Result<Vec<entities::Status>, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let options = megalodon::megalodon::GetPublicTimelineInputOptions {
        limit: Some(limit),
        ..Default::default()
    };

    let res = client
        .get_public_timeline(Some(&options))
        .await
        .map_err(|_| ())?;
    Ok(res.json())
}

#[tauri::command]
async fn get_conversation(
    entry_point: String,
    state: tauri::State<'_, AppState>,
) -> Result<entities::Context, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let options = megalodon::megalodon::GetStatusContextInputOptions {
        limit: Some(25),
        ..Default::default()
    };

    let res = client
        .get_status_context(entry_point, Some(&options))
        .await
        .map_err(|_| ())?;
    Ok(res.json())
}

#[tauri::command]
async fn get_local_timeline(
    limit: u32,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<entities::Status>, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let options = megalodon::megalodon::GetLocalTimelineInputOptions {
        limit: Some(limit),
        ..Default::default()
    };

    let res = client
        .get_local_timeline(Some(&options))
        .await
        .map_err(|_| ())?;
    Ok(res.json())
}

#[tauri::command]
async fn set_instance(url: String, state: tauri::State<'_, AppState>) -> Result<(), ()> {
    let mut client_state = state.client_state.write();
    let mut client = state.client.write();

    *client_state = Some(ClientState {
        base_url: url.clone(),
        client_id: "".to_string(),
        client_secret: "".to_string(),
    });

    *client = Some(generator(megalodon::SNS::Pleroma, url, None, None));

    Ok(())
}

#[derive(Serialize)]
enum LoginState {
    LoggedIn,
    InstanceUnknown,
    LoginExpired,
}

#[tauri::command]
async fn login_state(state: tauri::State<'_, AppState>) -> Result<LoginState, ()> {
    // let lock = state.client.try_lock();
    if state.has_logged_in() {
        Ok(LoginState::LoggedIn)
    // } else if lock.is_some() && lock.unwrap().is_none() {
    //     Ok(LoginState::InstanceUnknown)
    } else {
        Ok(LoginState::LoginExpired)
    }
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
struct CallbackQuery {
    code: String,
}

async fn authorize(
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

    "authorised; reload dakko to boot up"
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
