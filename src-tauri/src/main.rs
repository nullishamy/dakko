// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use megalodon::generator;
use parking_lot::RwLock;
use state::{AppState, ClientState};

mod auth;
mod state;
mod relationship;
mod status;
mod timeline;
mod user;
mod init;


fn main() {
    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9119);

    tauri::Builder::default()
        .setup(init::setup)
        .invoke_handler(tauri::generate_handler![
            set_instance,
            auth::login,
            auth::login_state,
            user::get_instance,
            user::get_statuses,
            user::get_user,
            user::get_notifications,
            user::get_bookmarks,
            user::get_emojis,
            status::post_reply,
            status::boost_status,
            status::post_status,
            status::favourite_status,
            status::get_status,
            status::bookmark_status,
            status::unbookmark_status,
            status::vote_for_poll,
            timeline::get_home_timeline,
            timeline::get_public_timeline,
            timeline::get_conversation,
            timeline::get_local_timeline,
            timeline::get_markers,
            timeline::save_markers,
            timeline::get_home_catchup,
            relationship::get_relationships,
            relationship::block_user,
            relationship::unblock_user,
            relationship::mute_user,
            relationship::unmute_user,
            relationship::follow_user,
            relationship::unfollow_user,
            relationship::get_follow_requests,
            relationship::accept_follow_request,
            relationship::deny_follow_request,
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
