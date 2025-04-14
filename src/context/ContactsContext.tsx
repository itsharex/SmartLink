'use client';

import React, { createContext, useState, useContext, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useAuth } from '@/hooks/useAuth';
import { createConversation, ConversationType } from '@/lib/chatApi';
import { getCurrentUser } from '@/lib/authApi';

// 定义用户接口
export interface User {
  id: string;
  name: string;
  email?: string;
  avatar_url?: string;
  status?: string;
}

// 扩展的接口，包含搜索用户功能
interface ContactsContextType {
  acceptFriendRequest: (requestId: string) => Promise<void>;
  rejectFriendRequest: (requestId: string) => Promise<void>;
  createChatWithContact: (contactId: string) => Promise<string>;
  toggleFavorite: (contactId: string, isFavorite: boolean) => Promise<void>;
  // 新增的功能
  searchUsers: (query: string) => Promise<User[]>;
  sendFriendRequest: (recipientId: string) => Promise<void>;
  searching: boolean;
  searchResults: User[];
}

const ContactsContext = createContext<ContactsContextType | undefined>(undefined);

// 直接调用Tauri后端APIs
export const ContactsProvider: React.FC<{children: React.ReactNode}> = ({ children }) => {
  const { user } = useAuth();
  console.log("Current user in ContactsProvider:", user);
  const [searchResults, setSearchResults] = useState<User[]>([]);
  const [searching, setSearching] = useState(false);

  // 接受好友请求
  const acceptFriendRequest = async (requestId: string): Promise<void> => {
    const user = await getCurrentUser();
    try {
      if (!user) throw new Error("Not authenticated");
      
      await invoke<void>('accept_friend_request', {
        user_id: user.id,
        request_id: requestId
      });
      
      // 成功后可以添加一些UI反馈
    } catch (err) {
      console.error('Failed to accept friend request:', err);
      throw err;
    }
  };

  // 拒绝好友请求
  const rejectFriendRequest = async (requestId: string): Promise<void> => {
    const user = await getCurrentUser();
    try {
      if (!user) throw new Error("Not authenticated");
      
      await invoke<void>('reject_friend_request', {
        user_id: user.id,
        request_id: requestId
      });
      
      // 成功后可以添加一些UI反馈
    } catch (err) {
      console.error('Failed to reject friend request:', err);
      throw err;
    }
  };

  // 创建与联系人的聊天会话
  const createChatWithContact = async (contactId: string): Promise<string> => {
    console.log("开始获取当前用户...");
    const user = await getCurrentUser();
    console.log("获取到的用户信息:", user);
    
    try {
      if (!user) {
        console.error("用户未认证，无法创建聊天");
        throw new Error("Not authenticated");
      }
      
      console.log("准备创建会话，用户ID:", user.id, "联系人ID:", contactId);
      
      const conversation = await createConversation(
        [contactId],
        undefined,
        false,
        ConversationType.Direct
      );
      
      console.log("成功创建会话:", conversation);
      
      return conversation.id;
    } catch (err) {
      console.error('创建会话失败:', err);
      throw err;
    }
  };

  // 切换联系人收藏状态
  const toggleFavorite = async (contactId: string, isFavorite: boolean): Promise<void> => {
    const user = await getCurrentUser();
    try {
      if (!user) throw new Error("Not authenticated");
      
      if (isFavorite) {
        await invoke<void>('add_contact_to_favorites', {
          userId: user.id,
          contactId: contactId
        });
      } else {
        await invoke<void>('remove_contact_from_favorites', {
          userId: user.id,
          contactId: contactId
        });
      }
      
      // 成功后可以添加一些UI反馈
    } catch (err) {
      console.error('Failed to update favorite status:', err);
      throw err;
    }
  };

    // 新增：搜索用户
    const searchUsers = async (query: string): Promise<User[]> => {
        try {
            if (!query.trim()) {
                setSearchResults([]);
                return [];
            }
            
            console.log("Searching for:", query);
            setSearching(true);
            
            const results = await invoke<User[]>('search_users', { query });
            
            console.log("Search results:", results);
            setSearchResults(results || []);
            setSearching(false);
            return results || [];
        } catch (err) {
            console.error('Failed to search users:', err);
            setSearching(false);
            setSearchResults([]);
            return [];
        }
    };

    // 发送好友请求
    const sendFriendRequest = async (recipientId: string): Promise<void> => {
      const user = await getCurrentUser();
      try {
          if (!user) throw new Error("Not authenticated");
          console.log("Sending friend request from:", user.id, "to:", recipientId);
          
          // Ensure that you're passing `sender_id` as `senderId` to match what Rust expects
          await invoke<void>('send_friend_request', {
              senderId: user.id, // Use `senderId` here to match the backend
              recipientId: recipientId // Use `recipientId` here to match the backend
          });
      } catch (err) {
          console.error('Failed to send friend request:', err);
          throw err;
      }
  };
  

  const value = {
    acceptFriendRequest,
    rejectFriendRequest,
    createChatWithContact,
    toggleFavorite,
    searchUsers,
    sendFriendRequest,
    searching,
    searchResults
  };

  return <ContactsContext.Provider value={value}>{children}</ContactsContext.Provider>;
};

export const useContacts = () => {
  const context = useContext(ContactsContext);
  if (context === undefined) {
    throw new Error('useContacts must be used within a ContactsProvider');
  }
  return context;
};