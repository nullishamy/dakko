export enum ValidTimeline {
  HOME = 'home',
  PUBLIC = 'public',
  BUBBLE = 'bubble',
  KNOWN = 'known'
}

export interface Instance {
  uri: string;
  title: string;
  description: string;
  email: string;
  version: string;
  thumbnail: string | undefined;
  urls: object | undefined;
  stats: {
    user_count: number;
    status_count: number;
    domain_count: number;
  };
  languages: string[];
  registrations: boolean;
  approval_required: boolean;
  invites_enabled: boolean | undefined;
  configuration: InstanceConfig;
  contact_account: object | undefined;
  rules: object[] | undefined;
}

export interface InstanceConfig {
  statuses: StatusConfig;
  polls: PollConfig | undefined;
}

export interface StatusConfig {
  max_characters: number;
  max_media_attachments: number;
  characters_reserved_per_url: number;
}

export interface PollConfig {
  max_options: number;
  max_characters_per_option: number;
  min_expiration: number;
  max_expiration: number;
}

export interface Account {
  id: string;
  username: string;
  acct: string;
  display_name: string;
  locked: boolean;
  discoverable: boolean | undefined;
  group: boolean | undefined;
  noindex: boolean | undefined;
  moved: Account | undefined;
  suspended: boolean | undefined;
  limited: boolean | undefined;
  created_at: string;
  followers_count: number;
  following_count: number;
  statuses_count: number;
  note: string;
  url: string;
  avatar: string;
  avatar_static: string;
  header: string;
  header_static: string;
  emojis: Emoji[];
  fields: AccountField[];
  bot: boolean;
  source: unknown;
  role: unknown;
  mute_expires_at: string | undefined;
}
export interface AccountField {
  name: string;
  value: string;
  verified_at: string | undefined;
  verified: boolean | undefined;
}

export enum StatusVisibility {
  PUBLIC = 'public',
  UNLISTED = 'unlisted',
  PRIVATE = 'private',
  DIRECT = 'direct'
}

export enum AttachmentType {
  IMAGE = 'image',
  GIFV = 'gifv',
  VIDEO = 'video',
  AUDIO = 'audio',
  UNKNOWN = 'unknown'
}

export interface Attachment {
  id: string;
  type: AttachmentType;
  url: string;
  remote_url: string | undefined;
  preview_url: string | undefined;
  text_url: string | undefined;
  meta: unknown;
  description: string | undefined;
  blurhash: string | undefined;
}

export interface Emoji {
  shortcode: string;
  static_url: string;
  url: string;
  visible_in_picker: boolean;
  category: string | undefined;
}

export interface Status {
  id: string;
  uri: string;
  url: string | undefined;
  account: Account;
  in_reply_to_id: string | undefined;
  in_reply_to_account_id: string | undefined;
  reblog: Status | undefined;
  content: string;
  plain_content: string | undefined;
  created_at: string;
  edited_at: string;
  emojis: Emoji[];
  replies_count: number;
  reblogs_count: number;
  favourites_count: number;
  reblogged: boolean | undefined;
  favourited: boolean | undefined;
  muted: boolean | undefined;
  sensitive: boolean;
  spoiler_text: string;
  visibility: StatusVisibility;
  media_attachments: Attachment[];
  mentions: unknown[];
  tags: unknown[];
  card: unknown;
  poll: unknown;
  application: unknown;
  language: string | undefined;
  pinned: boolean | undefined;
  emoji_reactions: unknown[] | undefined;
  quote: boolean;
  bookmarked: boolean | undefined;
}

export enum NotificationType {
  FOLLOW = 'follow',
  FOLLOW_REQUEST = 'follow_request',
  MENTION = 'mention',
  REBLOG = 'reblog',
  FAVOURITE = 'favourite',
  POLl_VOTE = 'poll_vote',
  POLL_EXPIRED = 'poll_expired',
  STATUS = 'status',
  REACTION = 'reaction',
  UPDATE = 'update',
  MOVE = 'move',
  ADMIN_SIGNUP = 'admin_signup',
  ADMIN_REPORT = 'admin_report',
  GROUP_INVITED = 'group_invited',
  APP = 'app',
  UNKNOWN = 'unknown'
}

export enum LoginStatus {
  LOGGED_IN = 'LoggedIn',
  INSTANCE_UNKNOWN = 'InstanceUnknown',
  LOGIN_EXPIRED = 'LoginExpired'
}

export interface Notification {
  account: Account | undefined;
  created_at: string;
  id: string;
  status: Status | undefined;
  reaction: unknown;
  target: Account | undefined;
  type: NotificationType;
}

export interface StatusContext {
  ancestors: Status[];
  descendants: Status[];
}
