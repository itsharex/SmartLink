'use client';

import React, { useState, useEffect } from 'react';
import SideNav from '@/components/layout/SideNav';
import ContactsSidebar from '@/components/contacts/ContactsSidebar';
import ContactsList, { Contact } from '@/components/contacts/ContactsList';
import { useContacts } from '@/context/ContactsContext';
import { invoke } from '@tauri-apps/api/core';
import { useAuth } from '@/hooks/useAuth';

// 添加好友请求接口
interface FriendRequest {
  id: string;
  name: string;
  avatar: string;
  timeAgo: string;
}

export default function ContactsPage() {
  const [activeTab, setActiveTab] = useState<'all' | 'favorites' | 'groups' | 'friendRequests'>('all');
  const [searchTerm, setSearchTerm] = useState('');
  const [contacts, setContacts] = useState<Contact[]>([]);
  const [favoriteContacts, setFavoriteContacts] = useState<Contact[]>([]);
  const [friendRequests, setFriendRequests] = useState<FriendRequest[]>([]);
  const [loading, setLoading] = useState(false);
  const { user } = useAuth();
  const contactsContext = useContacts();
  
  // 从数据库加载联系人
  const loadContacts = async () => {
    try {
      setLoading(true);
      if (!user) return;
      
      // 获取所有联系人
      const allContacts = await invoke<any[]>('get_contacts', { userId: user.id });
      
      // 获取收藏的联系人
      const favorites = await invoke<any[]>('get_favorite_contacts', { userId: user.id });
      
      // 转换数据格式
      const contactsList: Contact[] = allContacts.map(contact => ({
        id: contact.id,
        name: contact.name,
        status: contact.status || 'offline',
        avatar: contact.avatar_url || '',
        favorite: favorites.some(fav => fav.id === contact.id),
        tags: [] // 你可能需要从其他地方获取标签
      }));
      
      const favoritesList: Contact[] = favorites.map(contact => ({
        id: contact.id,
        name: contact.name,
        status: contact.status || 'offline',
        avatar: contact.avatar_url || '',
        favorite: true,
        tags: []
      }));
      
      setContacts(contactsList);
      setFavoriteContacts(favoritesList);
    } catch (error) {
      console.error('Failed to load contacts:', error);
    } finally {
      setLoading(false);
    }
  };
  
  // 加载好友请求
  const loadFriendRequests = async () => {
    try {
      setLoading(true);
      if (!user) return;
      
      // 获取好友请求
      const requests = await invoke<any[]>('get_friend_requests', { userId: user.id });
      
      // 转换数据格式
      const requestsList: FriendRequest[] = requests.map(req => ({
        id: req.id,
        name: req.sender?.name || 'Unknown User',
        avatar: req.sender?.avatar_url || '',
        timeAgo: calculateTimeAgo(req.created_at)
      }));
      
      setFriendRequests(requestsList);
    } catch (error) {
      console.error('Failed to load friend requests:', error);
    } finally {
      setLoading(false);
    }
  };
  
  // 计算时间差的辅助函数
  const calculateTimeAgo = (timestamp: string): string => {
    const date = new Date(timestamp);
    const now = new Date();
    const diffInSeconds = Math.floor((now.getTime() - date.getTime()) / 1000);
    
    if (diffInSeconds < 60) return 'just now';
    if (diffInSeconds < 3600) return `${Math.floor(diffInSeconds / 60)} minutes ago`;
    if (diffInSeconds < 86400) return `${Math.floor(diffInSeconds / 3600)} hours ago`;
    if (diffInSeconds < 604800) return `${Math.floor(diffInSeconds / 86400)} days ago`;
    if (diffInSeconds < 2592000) return `${Math.floor(diffInSeconds / 604800)} weeks ago`;
    
    return `${Math.floor(diffInSeconds / 2592000)} months ago`;
  };
  
  // 根据标签和搜索过滤联系人
  const getFilteredContacts = () => {
    let filteredList = activeTab === 'favorites' ? favoriteContacts : contacts;
    
    if (searchTerm) {
      filteredList = filteredList.filter(contact => 
        contact.name.toLowerCase().includes(searchTerm.toLowerCase())
      );
    }
    
    return filteredList;
  };
  
  // 初始加载和标签切换时加载数据
  useEffect(() => {
    if (activeTab === 'friendRequests') {
      loadFriendRequests();
    } else {
      loadContacts();
    }
  }, [activeTab, user]);
  
  return (
    <div className="flex h-screen overflow-hidden">
      <SideNav userName={user?.name || ''} />
      
      <div className="flex-1 bg-bg-primary p-6 overflow-auto">
        <div className="max-w-6xl mx-auto">
          <header className="mb-8">
            <h1 className="text-3xl font-bold font-orbitron mb-2 bg-gradient-to-r from-accent-primary to-accent-secondary bg-clip-text text-transparent">
              Contacts
            </h1>
            <p className="text-text-primary">Manage your contacts and groups</p>
          </header>
          
          <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
            {/* Sidebar */}
            <div className="md:col-span-1">
              <ContactsSidebar
                activeTab={activeTab}
                setActiveTab={setActiveTab}
                searchTerm={searchTerm}
                setSearchTerm={setSearchTerm}
                friendRequestCount={friendRequests.length}
              />
            </div>
            
            {/* Contacts List */}
            <div className="md:col-span-3">
              {loading ? (
                <div className="flex items-center justify-center h-64">
                  <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-accent-primary"></div>
                </div>
              ) : activeTab === 'friendRequests' ? (
                <ContactsList
                  activeTab={activeTab}
                  friendRequestsData={friendRequests}
                  contacts={[]}
                />
              ) : (
                <ContactsList
                  activeTab={activeTab}
                  contacts={getFilteredContacts()}
                />
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}