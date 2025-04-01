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

export async function updateMessageStatus(
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