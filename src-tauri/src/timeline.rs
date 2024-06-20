use megalodon::entities;

use crate::state::AppState;
use crate::error;

#[tauri::command]
pub async fn get_markers(
    timelines: Vec<String>,
    state: tauri::State<'_, AppState>,
) -> Result<entities::Marker, error::DakkoError> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let res = client.get_markers(timelines).await?;
    Ok(res.json())
}

#[tauri::command]
pub async fn save_markers(
    last_post_in_home: String,
    last_notification: String,
    state: tauri::State<'_, AppState>,
) -> Result<entities::Marker, error::DakkoError> {
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

    let res = client.save_markers(Some(&options)).await?;
    Ok(res.json())
}

#[tauri::command]
pub async fn get_home_catchup(
    since_id: String,
    state: tauri::State<'_, AppState>,
) -> Result<usize, error::DakkoError> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let options = megalodon::megalodon::GetHomeTimelineInputOptions {
        since_id: Some(since_id),
        ..Default::default()
    };

    let res = client.get_home_timeline(Some(&options)).await?;
    Ok(res.json().len())
}

#[tauri::command]
pub async fn get_public_catchup(
    since_id: String,
    state: tauri::State<'_, AppState>,
) -> Result<usize, error::DakkoError> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let options = megalodon::megalodon::GetPublicTimelineInputOptions {
        since_id: Some(since_id),
        ..Default::default()
    };

    let res = client.get_public_timeline(Some(&options)).await?;
    Ok(res.json().len())
}

#[tauri::command]
pub async fn get_home_timeline(
    start_at: Option<String>,
    limit: u32,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<entities::Status>, error::DakkoError> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let options = megalodon::megalodon::GetHomeTimelineInputOptions {
        limit: Some(limit),
        max_id: start_at,
        ..Default::default()
    };

    let res = client.get_home_timeline(Some(&options)).await?;
    Ok(res.json())
}

#[tauri::command]
pub async fn get_public_timeline(
    state: tauri::State<'_, AppState>,
    limit: u32,
) -> Result<Vec<entities::Status>, error::DakkoError> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let options = megalodon::megalodon::GetPublicTimelineInputOptions {
        limit: Some(limit),
        ..Default::default()
    };

    let res = client.get_public_timeline(Some(&options)).await?;
    Ok(res.json())
}

#[tauri::command]
pub async fn get_conversation(
    entry_point: String,
    state: tauri::State<'_, AppState>,
) -> Result<entities::Context, error::DakkoError> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let options = megalodon::megalodon::GetStatusContextInputOptions {
        limit: Some(25),
        ..Default::default()
    };

    let res = client
        .get_status_context(entry_point, Some(&options))
        .await?;
    Ok(res.json())
}

#[tauri::command]
pub async fn get_local_timeline(
    limit: u32,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<entities::Status>, error::DakkoError> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let options = megalodon::megalodon::GetLocalTimelineInputOptions {
        limit: Some(limit),
        ..Default::default()
    };

    let res = client.get_local_timeline(Some(&options)).await?;
    Ok(res.json())
}
