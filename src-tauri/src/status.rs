use megalodon::entities;
use serde::{Deserialize, Serialize};

use crate::state::AppState;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Content {
    content: String,
    cw: Option<String>,
    visibility: entities::status::StatusVisibility,
    quoting: Option<String>,
}

#[tauri::command]
pub async fn post_reply(
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
pub async fn post_status(
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
pub async fn favourite_status(
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
pub async fn get_status(id: String, state: tauri::State<'_, AppState>) -> Result<entities::Status, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let res = client.get_status(id).await.map_err(|_| ())?;
    Ok(res.json())
}

#[tauri::command]
pub async fn boost_status(
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
pub async fn bookmark_status(
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
pub async fn unbookmark_status(
    id: String,
    state: tauri::State<'_, AppState>,
) -> Result<entities::Status, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let res = client.unbookmark_status(id).await.unwrap();
    Ok(res.json())
}
