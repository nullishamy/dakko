use crate::app;
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    routing::get,
};
use megalodon::entities;
use serde::Deserialize;
use std::{
    net,
    sync::{mpsc, Arc},
};
use tokio::{runtime::Runtime, sync::RwLock};

#[derive(Deserialize)]
pub struct CallbackQuery {
    code: String,
}

pub struct HttpState {
    pub client: Option<Box<dyn megalodon::Megalodon + Sync + Send>>,
    pub app_data: Option<megalodon::oauth::AppData>,
    pub tx: mpsc::Sender<HttpResponse>,
    pub egui_ctx: Option<egui::Context>,
}

impl HttpState {
    fn send_response(&self, res: HttpResponse) {
        self.tx.send(res).unwrap();
        self.egui_ctx.as_ref().unwrap().request_repaint();
    }
}

#[axum::debug_handler]
pub async fn authorize(
    State(handle): State<Arc<RwLock<HttpState>>>,
    query: Query<CallbackQuery>,
) -> impl IntoResponse {
    let code = query.code.trim().to_string();
    let state = handle.write().await;
    let app_data = state.app_data.as_ref().unwrap();
    let client = state.client.as_ref().unwrap();

    let access_token_res = client
        .fetch_access_token(
            app_data.client_id.clone(),
            app_data.client_secret.clone(),
            code,
            megalodon::default::NO_REDIRECT.to_string(),
        )
        .await;
    match access_token_res {
        Ok(token_data) => state.tx.send(HttpResponse::TokenData(token_data)).unwrap(),
        Err(err) => {
            println!("{:#?}", err);
        }
    }

    "authorisation complete"
}

pub async fn run_server(
    handle: Arc<RwLock<HttpState>>,
    addr: net::SocketAddr,
) -> Result<(), axum::Error> {
    let app = axum::Router::new()
        .route("/", get(authorize))
        .with_state(handle);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Started server");
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

pub fn handle(state: &Arc<RwLock<HttpState>>, req: HttpRequest, rt: &Runtime) {
    match req {
        HttpRequest::RequestAuthURL(addr) => {
            let mut state = rt.block_on(state.write());
            let client = state.client.as_ref().unwrap();

            let options = megalodon::megalodon::AppInputOptions {
                scopes: Some(
                    [
                        String::from("read"),
                        String::from("write"),
                        String::from("follow"),
                    ]
                    .to_vec(),
                ),
                redirect_uris: Some(format!("http://{}:{}/", addr.ip(), addr.port())),
                ..Default::default()
            };

            match rt.block_on(client.register_app(String::from("dakko"), &options)) {
                Ok(app_data) => {
                    state.app_data = Some(app_data.clone());
                    // let client_id = app_data.client_id;
                    // let client_secret = app_data.client_secret;
                    eprintln!("Authorization URL is generated {:#?}", app_data.url);
                    // tauri::async_runtime::spawn(async move { run_server(handle).await });
                    state.send_response(HttpResponse::RequestAuthURL(app_data));
                }
                Err(err) => {
                    panic!("could not generate auth url: {}", err)
                }
            };
        }
        HttpRequest::Configure(bootstrap) => {
            let mut state = rt.block_on(state.write());
            state.client = Some(megalodon::generator(
                bootstrap.instance_type,
                bootstrap.base_url,
                None,
                None,
            ));
        }
        HttpRequest::Authenticate(client, auth) => {
            let mut state = rt.block_on(state.write());
            state.client = Some(megalodon::generator(
                client.instance_type,
                client.base_url,
                Some(auth.token.access_token),
                None,
            ));
        }
        HttpRequest::OpenURL(url) => {
            println!("opening {url}");
            open::that(url).unwrap();
        }
        HttpRequest::LaunchServer(addr) => {
            // Must spawn() it so it runs in the bg
            rt.spawn(run_server(Arc::clone(state), addr));
        }
        HttpRequest::RequestSelf => {
            let state = rt.block_on(state.read());
            let client = state.client.as_ref().unwrap();
            let account = rt.block_on(client.verify_account_credentials()).unwrap();
            state.send_response(HttpResponse::RequestSelf(account.json()));
        }
        HttpRequest::RequestTimeline(timeline, from_id) => {
            let state = rt.block_on(state.read());
            let client = state.client.as_ref().unwrap();
            match timeline {
                app::Timeline::Home => {
                    let opts = megalodon::megalodon::GetHomeTimelineInputOptions {
                        limit: Some(25),
                        max_id: from_id,
                        ..Default::default()
                    };

                    let statuses = rt.block_on(client.get_home_timeline(Some(&opts))).unwrap();

                    state.send_response(HttpResponse::RequestTimeline(timeline, statuses.json()));
                }
            }
        }
        HttpRequest::Init(ctx) => {
            let mut state = rt.block_on(state.write());
            state.egui_ctx = Some(ctx);
        }
    }
}

#[derive(Debug)]
pub enum HttpRequest {
    Init(egui::Context),
    RequestAuthURL(net::SocketAddr),
    OpenURL(String),
    LaunchServer(net::SocketAddr),
    Configure(app::BootstrapState),
    Authenticate(app::ClientState, app::AuthState),
    RequestTimeline(app::Timeline, Option<String>),
    RequestSelf,
}

#[derive(Debug)]
pub enum HttpResponse {
    RequestAuthURL(megalodon::oauth::AppData),
    TokenData(megalodon::oauth::TokenData),
    RequestSelf(entities::Account),
    RequestTimeline(app::Timeline, Vec<entities::Status>),
}
