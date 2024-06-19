use megalodon::entities;

use crate::error;
use crate::state::AppState;

#[tauri::command]
pub async fn get_bookmarks(state: tauri::State<'_, AppState>) -> Result<Vec<entities::Status>, error::DakkoError> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let res = client.get_bookmarks(None).await?;
    Ok(res.json())
}

#[tauri::command]
pub async fn get_emojis(state: tauri::State<'_, AppState>) -> Result<Vec<entities::Emoji>, error::DakkoError> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let res = client.get_instance_custom_emojis().await?;
    Ok(res.json())
}

#[tauri::command]
pub async fn get_statuses(
    id: String,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<entities::Status>, error::DakkoError> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let options = megalodon::megalodon::GetAccountStatusesInputOptions {
        limit: Some(25),
        ..Default::default()
    };

    let res = client.get_account_statuses(id, Some(&options)).await?;
    Ok(res.json())
}

#[tauri::command]
pub async fn get_instance(
    state: tauri::State<'_, AppState>,
) -> Result<entities::Instance, error::DakkoError> {
    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let res = client.get_instance().await?;
    Ok(res.json())
}

#[tauri::command]
pub async fn get_user(
    state: tauri::State<'_, AppState>,
) -> Result<entities::Account, error::DakkoError> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let res = client.verify_account_credentials().await?;
    Ok(res.json())
}

#[tauri::command]
pub async fn get_notifications(
    since: Option<String>,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<entities::Notification>, error::DakkoError> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let options = megalodon::megalodon::GetNotificationsInputOptions {
        limit: Some(25),
        since_id: since,
        ..Default::default()
    };

    let res = client.get_notifications(Some(&options)).await?;
    Ok(res.json())
}
