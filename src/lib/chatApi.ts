// src/lib/chatApi.ts
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { getCurrentUser } from './authApi';

// 类型定义
export enum MessageType {
  Text = 'Text',
  Image = 'Image',
  File = 'File',
  Voice = 'Voice',
  Video = 'Video',
  Location = 'Location'
}

export enum ConversationType {
  Direct = 'Direct',
  Group = 'Group'
}

export enum MessageStatus {
  Sent = 'Sent',
  Delivered = 'Delivered',
  Read = 'Read'
}

export interface ReadStatus {
  read_by: Record<string, string>; // 用户ID -> 时间戳
}

export interface LastMessagePreview {
  id: string;
  sender_id: string;
  content: string;
  content_type: MessageType;
  timestamp: string;
  read_by_all: boolean;
}

export interface Message {
  id: string;
  conversation_id: string;
  sender_id: string;
  content: string;
  content_type: MessageType;
  timestamp: string;
  read_status: ReadStatus;
  encrypted: boolean;
  encryption_info?: {
    algorithm: string;
    key_id: string;
  };
  media_url?: string;
}

export interface Conversation {
  id: string;
  conversation_type: ConversationType;
  participants: string[];
  created_at: string;
  updated_at: string;
  last_message?: LastMessagePreview;
  encryption_key?: string;
  name?: string;
  avatar_url?: string;
}

export interface WebSocketEvent {
  id: string;
  event_type: string;
  conversation_id: string;
  data: any;
  timestamp: string;
}

// 自定义错误类型
export class AuthenticationError extends Error {
  constructor(message = 'You must be logged in to perform this action') {
    super(message);
    this.name = 'AuthenticationError';
  }
}

async function withAuth<T>(fn: () => Promise<T>): Promise<T> {
  try {
    await getCurrentUser();
    return await fn();
  } catch (error) {
    console.error('Authentication error details:', error); // 打印详细错误
    throw new AuthenticationError(error instanceof Error ? error.message : 'Unknown authentication error');
  }
}

// 实时通信监听器
let chatEventListenerInitialized = false;

export function initChatEventListener(callback: (event: WebSocketEvent) => void): Promise<() => void> {
  return withAuth(async () => {
    if (chatEventListenerInitialized) {
      throw new Error('Chat event listener already initialized');
    }
    
    chatEventListenerInitialized = true;
    
    return listen<WebSocketEvent>('chat_event', (event: { payload: WebSocketEvent; }) => {
      callback(event.payload);
    }).then((unlisten: () => void) => {
      return () => {
        chatEventListenerInitialized = false;
        unlisten();
      };
    });
  });
}

// 会话管理
export async function getConversations(): Promise<Conversation[]> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    // 修改这里：user_id 而不是 userId
    return invoke<Conversation[]>('get_conversations', { 
      user_id: user.id 
    });
  });
}

export async function createConversation(
  participants: string[],
  name?: string,
  encryptionEnabled: boolean = false, 
  conversationType: ConversationType = ConversationType.Direct
): Promise<Conversation> {
  return withAuth(async () => {
    const payload = { name, participants, encryptionEnabled, conversationType };
    console.log("Sending to invoke:", JSON.stringify(payload));
    return invoke<Conversation>('create_conversation', payload);
  });
}

export async function getConversation(conversation_id: string): Promise<Conversation | null> {
  return withAuth(async () => {
    // 注意参数名称
    return invoke<Conversation | null>('get_conversation', { 
      conversation_id
    });
  });
}

// 消息管理
export async function sendMessage(
  conversation_id: string,
  content: string,
  content_type: MessageType = MessageType.Text,
  media_url?: string
): Promise<Message> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    
    // 确保参数名称与后端一致
    return invoke<Message>('send_message', {
      conversation_id,
      content,
      sender_id: user.id,
      content_type,
      media_url
    });
  });
}

export async function getMessages(
  conversation_id: string,
  limit?: number,
  before_id?: string
): Promise<Message[]> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    
    // 确保参数名称与后端一致
    return invoke<Message[]>('get_messages', {
      conversation_id,
      user_id: user.id,
      limit,
      before_id,
    });
  });
}

export async function updateLocalMessageStatus(
  message_id: string,
  status: MessageStatus
): Promise<void> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    
    // 确保参数名称与后端一致
    return invoke<void>('update_message_status', {
      message_id,
      user_id: user.id,
      status
    });
  });
}

export async function markMessageRead(message_id: string): Promise<void> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    
    // 确保参数名称与后端一致
    return invoke<void>('mark_message_read', {
      message_id,
      user_id: user.id
    });
  });
}

export async function markConversationRead(conversation_id: string): Promise<number> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    
    // 确保参数名称与后端一致
    return invoke<number>('mark_conversation_read', {
      conversation_id,
      user_id: user.id
    });
  });
}

export async function markConversationDelivered(conversation_id: string): Promise<number> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    
    // 确保参数名称与后端一致
    return invoke<number>('mark_conversation_delivered', {
      conversation_id,
      user_id: user.id
    });
  });
}

// 群组管理
export async function createGroupChat(
  name: string,
  members: string[],
  encryptionEnabled: boolean = false // 改为驼峰格式
): Promise<Conversation> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    return invoke<Conversation>('create_group_chat', {
      name,
      creatorId: user.id,
      members,
      encryptionEnabled
    });
  });
}

export async function addGroupMember(
  conversation_id: string,
  member_id: string
): Promise<void> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    
    // 确保参数名称与后端一致
    return invoke<void>('add_group_member', {
      conversation_id,
      user_id: user.id,
      member_id
    });
  });
}

export async function removeGroupMember(
  conversation_id: string,
  member_id: string
): Promise<void> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    
    // 确保参数名称与后端一致
    return invoke<void>('remove_group_member', {
      conversation_id,
      user_id: user.id,
      member_id
    });
  });
}

// 状态查询
export async function getUnreadCount(): Promise<number> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    
    // 确保参数名称与后端一致
    return invoke<number>('get_unread_count', {
      user_id: user.id
    });
  });
}

export async function getOnlineParticipants(conversation_id: string): Promise<string[]> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    
    // 确保参数名称与后端一致
    return invoke<string[]>('get_online_participants', {
      conversation_id,
      user_id: user.id
    });
  });
}

// WebSocket 相关功能
export async function initializeWebSocket(server_url?: string): Promise<void> {
  return withAuth(async () => {
    return invoke<void>('initialize_websocket', { 
      server_url
    });
  });
}

export async function connectWebSocket(): Promise<void> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    return invoke<void>('connect_websocket', { 
      user_id: user.id 
    });
  });
}

export async function disconnectWebSocket(): Promise<void> {
  return withAuth(async () => {
    return invoke<void>('disconnect_websocket');
  });
}

export async function getWebSocketStatus(): Promise<ConnectionStatus> {
  return withAuth(async () => {
    return invoke<ConnectionStatus>('get_websocket_status');
  });
}

export async function sendWebSocketMessage(message: string): Promise<void> {
  return withAuth(async () => {
    return invoke<void>('send_websocket_message', { message });
  });
}

export async function sendChatMessage(
  conversation_id: string,
  content: string,
  recipient_id?: string,
  message_type: string = MessageType.Text
): Promise<void> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    return invoke<void>('send_chat_message', {
      conversation_id,
      recipient_id,
      content,
      sender_id: user.id,
      message_type,
    });
  });
}

export async function sendWebRTCSignal(
  recipient_id: string,
  signal_type: string,
  signal_data: any,
  conversation_id?: string
): Promise<void> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    return invoke<void>('send_webrtc_signal', {
      recipient_id,
      conversation_id,
      signal_type,
      signal_data,
      sender_id: user.id
    });
  });
}

export async function updateMessageStatus(
  message_id: string,
  conversation_id: string,
  original_sender_id: string,
  status: 'read' | 'delivered'
): Promise<void> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    return invoke<void>('update_message_status', {
      message_id,
      conversation_id,
      original_sender_id,
      status,
      user_id: user.id
    });
  });
}

export async function sendTypingIndicator(
  conversation_id: string,
  is_typing: boolean,
  recipients: string[]
): Promise<void> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    return invoke<void>('send_typing_indicator', {
      conversation_id,
      is_typing,
      user_id: user.id,
      recipients
    });
  });
}

// 添加 ConnectionStatus 枚举（无需修改）
export enum ConnectionStatus {
  Connected = 'Connected',
  Connecting = 'Connecting',
  Disconnected = 'Disconnected',
  Error = 'Error'
}

// 接口定义（无需修改参数名，仅用于类型）
export interface User {
  id: string;
  name: string;
  email?: string;
  avatar_url?: string;
  status?: 'online' | 'offline' | 'away' | 'busy';
}

export interface FriendRequest {
  id: string;
  sender_id: string;
  recipient_id: string;
  status: 'pending' | 'accepted' | 'rejected';
  created_at: string;
  sender?: User;
}

// 获取所有联系人
export async function getContacts(): Promise<User[]> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    return invoke<User[]>('get_contacts', { 
      user_id: user.id 
    });
  });
}

// 获取收藏的联系人
export async function getFavoriteContacts(): Promise<User[]> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    return invoke<User[]>('get_favorite_contacts', { 
      user_id: user.id 
    });
  });
}

// 搜索用户
export async function searchUsers(query: string): Promise<User[]> {
  return withAuth(async () => {
    return invoke<User[]>('search_users', { 
      query 
    });
  });
}

// 发送好友请求
export async function sendFriendRequest(recipient_id: string): Promise<void> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    return invoke<void>('send_friend_request', {
      sender_id: user.id,
      recipient_id
    });
  });
}

// 获取好友请求列表
export async function getFriendRequests(): Promise<FriendRequest[]> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    return invoke<FriendRequest[]>('get_friend_requests', {
      user_id: user.id
    });
  });
}

// 接受好友请求
export async function acceptFriendRequest(request_id: string): Promise<void> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    return invoke<void>('accept_friend_request', {
      user_id: user.id,
      request_id
    });
  });
}

// 拒绝好友请求
export async function rejectFriendRequest(request_id: string): Promise<void> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    return invoke<void>('reject_friend_request', {
      user_id: user.id,
      request_id
    });
  });
}

// 将联系人添加到收藏
export async function addContactToFavorites(contact_id: string): Promise<void> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    return invoke<void>('add_contact_to_favorites', {
      user_id: user.id,
      contact_id
    });
  });
}

// 从收藏中移除联系人
export async function removeContactFromFavorites(contact_id: string): Promise<void> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    return invoke<void>('remove_contact_from_favorites', {
      user_id: user.id,
      contact_id
    });
  });
}