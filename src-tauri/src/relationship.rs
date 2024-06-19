use megalodon::{entities, megalodon::FollowRequestOutput};

use crate::state::AppState;


#[tauri::command]
pub async fn get_follow_requests(
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
pub async fn get_relationships(
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
pub async fn accept_follow_request(
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
pub async fn deny_follow_request(
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
pub async fn follow_user(
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
pub async fn unfollow_user(
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
pub async fn block_user(
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
pub async fn unblock_user(
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
pub async fn mute_user(
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
pub async fn unmute_user(
    id: String,
    state: tauri::State<'_, AppState>,
) -> Result<entities::Relationship, ()> {
    assert!(state.has_logged_in());

    let client = state.client.read();
    let client = client.as_ref().unwrap();

    let res = client.unmute_account(id).await.map_err(|_| ())?;
    Ok(res.json())
}