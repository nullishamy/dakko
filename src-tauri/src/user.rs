use megalodon::entities;

use crate::state::AppState;


#[tauri::command]
pub async fn get_bookmarks(state: tauri::State<'_, AppState>) -> Result<Vec<entities::Status>, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let res = client.get_bookmarks(None).await.map_err(|_| ())?;
    Ok(res.json())
}

#[tauri::command]
pub async fn get_emojis(state: tauri::State<'_, AppState>) -> Result<Vec<entities::Emoji>, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let res = client.get_instance_custom_emojis().await.map_err(|_| ())?;
    Ok(res.json())
}

#[tauri::command]
pub async fn get_statuses(
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
pub async fn get_instance(state: tauri::State<'_, AppState>) -> Result<entities::Instance, ()> {
    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let res = client.get_instance().await.map_err(|_| ())?;
    Ok(res.json())
}

#[tauri::command]
pub async fn get_user(state: tauri::State<'_, AppState>) -> Result<entities::Account, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let res = client.verify_account_credentials().await.map_err(|_| ())?;
    Ok(res.json())
}

#[tauri::command]
pub async fn get_notifications(
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