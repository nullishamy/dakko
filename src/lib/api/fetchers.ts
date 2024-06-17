import * as api from '$lib/api';
import { invoke } from '@tauri-apps/api';

function makeSimpleFetcher<T>(key: string): () => Promise<T> {
  return () => fetch(key);
}

function makeAccountAction<R>(key: string): (targetAccountId: string) => Promise<R> {
  return (targetAccountId: string) => fetch(key, { id: targetAccountId })
}

function makeStatusAction<R>(key: string): (targetStatusId: string) => Promise<R> {
  return (targetStatusId: string) => fetch(key, { id: targetStatusId })
}

async function fetch<R>(...params: Parameters<typeof invoke>): Promise<R> {
  return invoke(...params) as Promise<R>;
}

export const fetchLoginState = makeSimpleFetcher<api.LoginStatus>('login_state');
export const fetchInstance = makeSimpleFetcher<api.Instance>('get_instance');
export const fetchSelf = makeSimpleFetcher<api.Account>('get_user');

export async function setInstance(instanceURL: string): Promise<void> {
  return fetch('set_instance', { url: instanceURL });
}
export const fetchLoginURL = makeSimpleFetcher<string>('login');
export const fetchBookmarks = makeSimpleFetcher<api.Status[]>('get_bookmarks');
export const fetchFollowRequests = makeSimpleFetcher<api.FollowRequest[]>('get_follow_requests');

export async function fetchNotifications(sinceId?: string): Promise<api.Notification[]> {
  return fetch('get_notifications', { since: sinceId })
}

export async function fetchStatusContext(id: string): Promise<api.StatusContext> {
  return fetch('get_conversation', { entryPoint: id });
}

export async function fetchRelationships(accountId: string): Promise<api.Relationship[]> {
  return fetch('get_relationships', { accountIds: [accountId] });
}
export const fetchStatuses = makeAccountAction<api.Status[]>('get_statuses')
export const followUser = makeAccountAction<api.Relationship>('follow_user')
export const unfollowUser = makeAccountAction<api.Relationship>('unfollow_user')

export const bookmarkStatus = makeStatusAction<void>('bookmark_status')
export const unbookmarkStatus = makeStatusAction<void>('unbookmark_status')
export const boostStatus = makeStatusAction<void>('boost_status')
export const unboostStatus = makeStatusAction<void>('unboost_status')
export const favouriteStatus = makeStatusAction<void>('favourite_status')
export const unfavouriteStatus = makeStatusAction<void>('unfavourite_status')

export interface StatusContent {
  content: string
  cw: string | undefined
}
export async function replyToStatus(statusId: string, reply: StatusContent): Promise<void> {
  return fetch('post_reply', {
    postId: statusId,
    reply
  })
}

export async function postStatus(status: StatusContent): Promise<api.Status> {
  return fetch('post_status', {
    status 
  })
}
