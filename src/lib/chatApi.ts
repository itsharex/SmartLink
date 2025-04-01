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

// 检查认证状态的包装函数
async function withAuth<T>(fn: () => Promise<T>): Promise<T> {
  try {
    // 尝试获取当前用户，如果失败会抛出错误
    await getCurrentUser();
    // 如果成功，执行原函数
    return await fn();
  } catch (error) {
    console.error('Authentication error:', error);
    throw new AuthenticationError();
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
    
    return listen<WebSocketEvent>('chat_event', (event) => {
      callback(event.payload);
    }).then(unlisten => {
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
    // 确保参数名称与后端一致
    return invoke<Conversation>('create_conversation', {
      name,
      participants,
      encryption_enabled: encryptionEnabled,
      conversation_type: conversationType
    });
  });
}

export async function getConversation(conversationId: string): Promise<Conversation | null> {
  return withAuth(async () => {
    // 注意参数名称
    return invoke<Conversation | null>('get_conversation', { 
      conversation_id: conversationId 
    });
  });
}

// 消息管理
export async function sendMessage(
  conversationId: string,
  content: string,
  contentType: MessageType = MessageType.Text,
  mediaUrl?: string
): Promise<Message> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    
    // 确保参数名称与后端一致
    return invoke<Message>('send_message', {
      conversation_id: conversationId,
      content,
      sender_id: user.id,
      content_type: contentType,
      media_url: mediaUrl
    });
  });
}

export async function getMessages(
  conversationId: string,
  limit?: number,
  beforeId?: string
): Promise<Message[]> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    
    // 确保参数名称与后端一致
    return invoke<Message[]>('get_messages', {
      conversation_id: conversationId,
      user_id: user.id,
      limit,
      before_id: beforeId
    });
  });
}

export async function updateLocalMessageStatus(
  messageId: string,
  status: MessageStatus
): Promise<void> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    
    // 确保参数名称与后端一致
    return invoke<void>('update_message_status', {
      message_id: messageId,
      user_id: user.id,
      status
    });
  });
}

export async function markMessageRead(messageId: string): Promise<void> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    
    // 确保参数名称与后端一致
    return invoke<void>('mark_message_read', {
      message_id: messageId,
      user_id: user.id
    });
  });
}

export async function markConversationRead(conversationId: string): Promise<number> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    
    // 确保参数名称与后端一致
    return invoke<number>('mark_conversation_read', {
      conversation_id: conversationId,
      user_id: user.id
    });
  });
}

export async function markConversationDelivered(conversationId: string): Promise<number> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    
    // 确保参数名称与后端一致
    return invoke<number>('mark_conversation_delivered', {
      conversation_id: conversationId,
      user_id: user.id
    });
  });
}

// 群组管理
export async function createGroupChat(
  name: string,
  members: string[],
  encryptionEnabled: boolean = false
): Promise<Conversation> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    
    // 确保参数名称与后端一致
    return invoke<Conversation>('create_group_chat', {
      name,
      creator_id: user.id,
      members,
      encryption_enabled: encryptionEnabled
    });
  });
}

export async function addGroupMember(
  conversationId: string,
  memberId: string
): Promise<void> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    
    // 确保参数名称与后端一致
    return invoke<void>('add_group_member', {
      conversation_id: conversationId,
      user_id: user.id,
      member_id: memberId
    });
  });
}

export async function removeGroupMember(
  conversationId: string,
  memberId: string
): Promise<void> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    
    // 确保参数名称与后端一致
    return invoke<void>('remove_group_member', {
      conversation_id: conversationId,
      user_id: user.id,
      member_id: memberId
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

export async function getOnlineParticipants(conversationId: string): Promise<string[]> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    
    // 确保参数名称与后端一致
    return invoke<string[]>('get_online_participants', {
      conversation_id: conversationId,
      user_id: user.id
    });
  });
}

// WebSocket 相关功能
export async function initializeWebSocket(serverUrl?: string): Promise<void> {
  return withAuth(async () => {
    return invoke<void>('initialize_websocket', { 
      server_url: serverUrl 
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
  conversationId: string,
  content: string,
  recipientId?: string,
  messageType: string = MessageType.Text
): Promise<void> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    return invoke<void>('send_chat_message', {
      conversation_id: conversationId,
      recipient_id: recipientId,
      content,
      sender_id: user.id,
      message_type: messageType
    });
  });
}

export async function sendWebRTCSignal(
  recipientId: string,
  signalType: string,
  signalData: any,
  conversationId?: string
): Promise<void> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    return invoke<void>('send_webrtc_signal', {
      recipient_id: recipientId,
      conversation_id: conversationId,
      signal_type: signalType,
      signal_data: signalData,
      sender_id: user.id
    });
  });
}

export async function updateMessageStatus(
  messageId: string,
  conversationId: string,
  originalSenderId: string,
  status: 'read' | 'delivered'
): Promise<void> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    return invoke<void>('update_message_status', {
      message_id: messageId,
      conversation_id: conversationId,
      original_sender_id: originalSenderId,
      status,
      user_id: user.id
    });
  });
}

export async function sendTypingIndicator(
  conversationId: string,
  isTyping: boolean,
  recipients: string[]
): Promise<void> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    return invoke<void>('send_typing_indicator', {
      conversation_id: conversationId,
      is_typing: isTyping,
      user_id: user.id,
      recipients
    });
  });
}

// 添加 ConnectionStatus 枚举
export enum ConnectionStatus {
  Connected = 'Connected',
  Connecting = 'Connecting',
  Disconnected = 'Disconnected',
  Error = 'Error'
}

// 在src/lib/chatApi.ts中添加以下函数和接口

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
    return invoke<User[]>('search_users', { query });
  });
}

// 发送好友请求
export async function sendFriendRequest(recipientId: string): Promise<void> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    return invoke<void>('send_friend_request', {
      sender_id: user.id,
      recipient_id: recipientId
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
export async function acceptFriendRequest(requestId: string): Promise<void> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    return invoke<void>('accept_friend_request', {
      user_id: user.id,
      request_id: requestId
    });
  });
}

// 拒绝好友请求
export async function rejectFriendRequest(requestId: string): Promise<void> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    return invoke<void>('reject_friend_request', {
      user_id: user.id,
      request_id: requestId
    });
  });
}

// 将联系人添加到收藏
export async function addContactToFavorites(contactId: string): Promise<void> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    return invoke<void>('add_contact_to_favorites', {
      user_id: user.id,
      contact_id: contactId
    });
  });
}

// 从收藏中移除联系人
export async function removeContactFromFavorites(contactId: string): Promise<void> {
  return withAuth(async () => {
    const user = await getCurrentUser();
    return invoke<void>('remove_contact_from_favorites', {
      user_id: user.id,
      contact_id: contactId
    });
  });
}