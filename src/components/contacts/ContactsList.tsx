'use client';

import React, { useState } from 'react';
import GlassCard from '@/components/ui/GlassCard';
import Avatar from '@/components/ui/Avatar';
import Button from '@/components/ui/Button';
import { UserPlus, Star, MessageSquare, Phone, Video } from 'lucide-react';
import { useContacts } from '@/context/ContactsContext';
import { useRouter } from 'next/navigation';
import SearchUserModal from './SearchUserModal'; // 导入模态框组件

export interface Contact {
  id: string;
  name: string;
  status: string;
  avatar: string;
  favorite: boolean;
  tags: string[];
}

interface FriendRequest {
  id: string;
  name: string;
  avatar: string;
  timeAgo: string;
}

interface ContactsListProps {
  activeTab: 'all' | 'favorites' | 'groups' | 'friendRequests';
  contacts: Contact[];
  friendRequestsData?: FriendRequest[];
}

const ContactsList: React.FC<ContactsListProps> = ({ activeTab, contacts, friendRequestsData }) => {
  // 使用Router和ContactsContext获取功能
  const router = useRouter();
  const { 
    acceptFriendRequest, 
    rejectFriendRequest,
    createChatWithContact,
    toggleFavorite 
  } = useContacts();
  
  // 添加状态控制搜索用户模态框
  const [isSearchModalOpen, setIsSearchModalOpen] = useState(false);

  let headerTitle = '';
  if (activeTab === 'all') {
    headerTitle = 'All Contacts';
  } else if (activeTab === 'favorites') {
    headerTitle = 'Favorite Contacts';
  } else if (activeTab === 'groups') {
    headerTitle = 'Groups';
  } else if (activeTab === 'friendRequests') {
    headerTitle = 'Friend Requests';
  }

  // 处理接受好友请求
  const handleAcceptRequest = async (requestId: string) => {
    try {
      await acceptFriendRequest(requestId);
      // 可以添加成功提示或刷新页面
    } catch (error) {
      console.error("Failed to accept request:", error);
    }
  };

  // 处理拒绝好友请求
  const handleRejectRequest = async (requestId: string) => {
    try {
      await rejectFriendRequest(requestId);
      // 可以添加成功提示或刷新页面
    } catch (error) {
      console.error("Failed to reject request:", error);
    }
  };

  // 处理创建聊天
  const handleStartChat = async (contactId: string) => {
    try {
      const conversationId = await createChatWithContact(contactId);
      router.push(`/chat?id=${conversationId}`);
    } catch (error) {
      console.error("Failed to start chat:", error);
    }
  };

  // 处理收藏联系人
  const handleToggleFavorite = async (contactId: string, isFavorite: boolean) => {
    try {
      await toggleFavorite(contactId, !isFavorite);
      // 成功后可以添加提示或刷新
    } catch (error) {
      console.error("Failed to toggle favorite:", error);
    }
  };

  // 处理添加联系人
  const handleAddContact = () => {
    setIsSearchModalOpen(true);
  };

  // Render friend requests if active tab is "friendRequests"
  if (activeTab === 'friendRequests') {
    return (
      <>
        <GlassCard className="p-6">
          <div className="flex justify-between items-center mb-6">
            <h2 className="text-xl font-semibold">{headerTitle}</h2>
          </div>
          {friendRequestsData && friendRequestsData.length > 0 ? (
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              {friendRequestsData.map(request => (
                <div 
                  key={request.id} 
                  className="bg-white/5 rounded-lg p-4 flex items-center justify-between hover:bg-accent-primary/5 transition duration-200"
                >
                  <div className="flex items-center gap-3">
                    <Avatar 
                      text={request.name} 
                      src={request.avatar} 
                    />
                    <div>
                      <h3 className="font-semibold text-text-primary">{request.name}</h3>
                      <p className="text-xs text-text-primary">{request.timeAgo}</p>
                    </div>
                  </div>
                  <div className="flex gap-2">
                    <Button 
                      variant="primary" 
                      size="sm"
                      onClick={() => handleAcceptRequest(request.id)}
                    >
                      Accept
                    </Button>
                    <Button 
                      variant="outline" 
                      size="sm"
                      onClick={() => handleRejectRequest(request.id)}
                    >
                      Reject
                    </Button>
                  </div>
                </div>
              ))}
            </div>
          ) : (
            <div className="text-center py-12">
              <p className="text-text-secondary mb-4">No friend requests found</p>
            </div>
          )}
        </GlassCard>
        
        {/* 搜索用户模态框 */}
        <SearchUserModal 
          isOpen={isSearchModalOpen}
          onClose={() => setIsSearchModalOpen(false)}
        />
      </>
    );
  }

  // Render regular contacts list
  return (
    <>
      <GlassCard className="p-6">
        <div className="flex justify-between items-center mb-6">
          <h2 className="text-xl font-semibold">{headerTitle}</h2>
          <Button 
            variant="outline" 
            size="sm"
            onClick={handleAddContact}
          >
            <UserPlus size={16} className="mr-2" />
            Add Contact
          </Button>
        </div>
        
        {contacts.length === 0 ? (
          <div className="text-center py-12">
            <p className="text-text-secondary mb-4">No contacts found</p>
            <Button 
              variant="outline"
              onClick={handleAddContact}
            >
              Add New Contact
            </Button>
          </div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {contacts.map(contact => (
              <div 
                key={contact.id} 
                className="bg-white/5 rounded-lg p-4 flex items-center justify-between hover:bg-accent-primary/5 transition duration-200"
              >
                <div className="flex items-center gap-3">
                  <Avatar 
                    text={contact.name} 
                    src={contact.avatar} 
                    status={contact.status as any} 
                  />
                  <div>
                    <div className="flex items-center gap-2">
                      <h3 className="font-semibold text-text-primary">{contact.name}</h3>
                      {contact.favorite && (
                        <Star size={14} className="text-yellow-400 fill-yellow-400" />
                      )}
                    </div>
                    {contact.tags && contact.tags.length > 0 && (
                      <div className="flex gap-1 mt-1">
                        {contact.tags.map((tag, index) => (
                          <span 
                            key={index} 
                            className="text-xs px-2 py-0.5 rounded bg-accent-primary/10 text-accent-primary"
                          >
                            {tag}
                          </span>
                        ))}
                      </div>
                    )}
                  </div>
                </div>
                <div className="flex gap-2">
                  <button 
                    className="w-8 h-8 flex items-center justify-center rounded-full bg-white/5 text-text-primary hover:bg-accent-primary/10 hover:text-accent-primary transition"
                    onClick={() => handleStartChat(contact.id)}
                  >
                    <MessageSquare size={16} />
                  </button>
                  <button className="w-8 h-8 flex items-center justify-center rounded-full bg-white/5 text-text-primary hover:bg-accent-primary/10 hover:text-accent-primary transition">
                    <Phone size={16} />
                  </button>
                  <button className="w-8 h-8 flex items-center justify-center rounded-full bg-white/5 text-text-primary hover:bg-accent-primary/10 hover:text-accent-primary transition">
                    <Video size={16} />
                  </button>
                  <button 
                    className="w-8 h-8 flex items-center justify-center rounded-full bg-white/5 text-text-primary hover:bg-accent-primary/10 hover:text-accent-primary transition"
                    onClick={() => handleToggleFavorite(contact.id, contact.favorite)}
                  >
                    <Star size={16} className={contact.favorite ? "fill-yellow-400 text-yellow-400" : ""} />
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}
      </GlassCard>
      
      {/* 搜索用户模态框 */}
      <SearchUserModal 
        isOpen={isSearchModalOpen}
        onClose={() => setIsSearchModalOpen(false)}
      />
    </>
  );
};

export default ContactsList;