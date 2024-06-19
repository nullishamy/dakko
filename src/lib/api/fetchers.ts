import * as api from '$lib/api';
import { invoke } from '@tauri-apps/api';
import type { InstanceType } from '../pane/bootstrap';

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

export async function setInstance(instanceURL: string, instanceType: InstanceType): Promise<void> {
  return fetch('set_instance', { url: instanceURL, instanceType });
}
export const fetchLoginURL = makeSimpleFetcher<string>('login');
export const fetchBookmarks = makeSimpleFetcher<api.Status[]>('get_bookmarks');
export const fetchFollowRequests = makeSimpleFetcher<api.FollowRequest[]>('get_follow_requests');
export const fetchCustomEmojis = makeSimpleFetcher<api.CustomEmoji[]>('get_emojis');

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
export const muteUser = makeAccountAction<api.Relationship>('mute_user')
export const unmuteUser = makeAccountAction<api.Relationship>('unumte_user')
export const blockUser = makeAccountAction<api.Relationship>('block_user')
export const unblockUser = makeAccountAction<api.Relationship>('unumte_user')

export const bookmarkStatus = makeStatusAction<void>('bookmark_status')
export const unbookmarkStatus = makeStatusAction<void>('unbookmark_status')
export const boostStatus = makeStatusAction<void>('boost_status')
export const unboostStatus = makeStatusAction<void>('unboost_status')
export const favouriteStatus = makeStatusAction<void>('favourite_status')
export const unfavouriteStatus = makeStatusAction<void>('unfavourite_status')

export interface StatusContent {
  content: string
  cw: string | undefined
  visibility: api.StatusVisibility
  quoting?: string
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

export async function fetchMarker(): Promise<api.Marker> {
  return fetch('get_markers', {
    timelines: ['home', 'notifications']
  })
}

export async function setMarker(lastHomeId: string): Promise<api.Marker> {
  return fetch('save_markers', {
    lastPostInHome: lastHomeId,
    lastNotification: 'garbage'
  })
}

export async function voteForPoll(poll: api.Poll, choices: number[]): Promise<api.Poll> {
  return fetch('vote_for_poll', {
    pollId: poll.id,
    choices
  })
}

export const acceptFollowRequest = makeAccountAction<api.Relationship>('accept_follow_request')
export const denyFollowRequest = makeAccountAction<api.Relationship>('deny_follow_request')