use std::fs::File;

use megalodon::generator;
use tauri::Manager;

use crate::state::{AppState, AuthState, ClientState};

pub fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
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