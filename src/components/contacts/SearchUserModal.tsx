// src/components/contacts/SearchUserModal.tsx
'use client';

import React, { useState } from 'react';
import { Search, X, UserPlus, Check } from 'lucide-react';
import { useContacts, User } from '@/context/ContactsContext';
import Avatar from '../ui/Avatar';
import Button from '../ui/Button';

interface SearchUserModalProps {
  isOpen: boolean;
  onClose: () => void;
}

const SearchUserModal: React.FC<SearchUserModalProps> = ({ isOpen, onClose }) => {
  const [query, setQuery] = useState('');
  const { searchUsers, sendFriendRequest, searching, searchResults } = useContacts();
  
  // 处理搜索
  const handleSearch = async () => {
    if (query.trim()) {
      await searchUsers(query);
    }
  };
  
  // 处理按键事件
  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleSearch();
    }
  };
  
  // 发送好友请求
  const handleSendRequest = async (userId: string) => {
    try {
      await sendFriendRequest(userId);
      // 可以添加成功提示
      alert("Friend request sent successfully!");
    } catch (error) {
      console.error("Failed to send friend request:", error);
    }
  };
  
  if (!isOpen) return null;
  
  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-bg-primary w-full max-w-md rounded-lg shadow-lg overflow-hidden">
        {/* Header */}
        <div className="flex items-center justify-between p-4 border-b border-text-primary-5">
          <h2 className="text-lg font-semibold text-text-primary">Find New Friends</h2>
          <button
            className="text-text-primary-30 hover:text-text-primary"
            onClick={onClose}
          >
            <X size={20} />
          </button>
        </div>
        
        {/* Search Input */}
        <div className="p-4">
          <div className="flex items-center gap-2 bg-bg-tertiary-80 rounded-lg px-3 py-3 mb-4">
            <Search size={20} className="text-text-primary-30" />
            <input 
              type="text" 
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              onKeyDown={handleKeyDown}
              placeholder="Search by name or email"
              className="bg-transparent w-full outline-none border-none text-text-primary placeholder:text-text-primary-30" 
            />
            <Button
              variant="primary"
              size="sm"
              onClick={handleSearch}
              disabled={!query.trim() || searching}
            >
              Search
            </Button>
          </div>
        </div>
        
        {/* Results */}
        <div className="max-h-80 overflow-y-auto p-4">
          {searching ? (
            <div className="text-center py-8">
              <p>Searching...</p>
            </div>
          ) : searchResults.length === 0 ? (
            <div className="text-center py-8">
              <p className="text-text-secondary">No users found</p>
              <p className="text-sm text-text-primary-30 mt-2">Try searching with a different name</p>
            </div>
          ) : (
            <div className="space-y-4">
              {searchResults.map(user => (
                <div 
                  key={user.id} 
                  className="bg-white/5 rounded-lg p-4 flex items-center justify-between"
                >
                  <div className="flex items-center gap-3">
                    <Avatar 
                      text={user.name} 
                      src={user.avatar_url || ''} 
                      status={user.status as any || 'offline'} 
                    />
                    <div>
                      <h3 className="font-semibold text-text-primary">{user.name}</h3>
                      {user.email && (
                        <p className="text-xs text-text-primary-30">{user.email}</p>
                      )}
                    </div>
                  </div>
                  <div>
                    <Button 
                      variant="primary" 
                      size="sm"
                      onClick={() => handleSendRequest(user.id)}
                    >
                      <UserPlus size={16} className="mr-1" />
                      Add
                    </Button>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default SearchUserModal;