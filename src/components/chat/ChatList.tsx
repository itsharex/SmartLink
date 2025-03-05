import React, { useState } from 'react';
import Avatar from '../ui/Avatar';
import { Search, Plus, Settings, ChevronDown, Pin, Filter } from 'lucide-react';

type ChatGroup = {
  id: string;
  name: string;
  collapsed?: boolean;
};

type Chat = {
  id: string;
  name: string;
  avatar?: string;
  lastMessage: string;
  time: string;
  unread: number;
  online: boolean;
  pinned?: boolean;
  muted?: boolean;
  groupId?: string; // 关联的分组ID
};

type ChatListProps = {
  chats: Chat[];
  groups?: ChatGroup[];
  onSelectChat: (chatId: string) => void;
  selectedChatId?: string;
};

const ChatList: React.FC<ChatListProps> = ({
  chats,
  groups = [],
  onSelectChat,
  selectedChatId
}) => {
  const [searchTerm, setSearchTerm] = useState('');
  const [groupCollapse, setGroupCollapse] = useState<Record<string, boolean>>({});
  const [showFilterMenu, setShowFilterMenu] = useState(false);
  const [filter, setFilter] = useState<'all' | 'unread' | 'pinned'>('all');
  
  // 按分组整理聊天
  const getChatsInGroup = (groupId?: string) => {
    return chats.filter(chat => 
      (!groupId && !chat.groupId) || chat.groupId === groupId
    );
  };
  
  // 应用筛选器和搜索
  const filterChats = (chats: Chat[]) => {
    return chats.filter(chat => {
      // 先应用搜索
      if (searchTerm && !chat.name.toLowerCase().includes(searchTerm.toLowerCase()) && 
          !chat.lastMessage.toLowerCase().includes(searchTerm.toLowerCase())) {
        return false;
      }
      
      // 再应用筛选器
      if (filter === 'unread' && chat.unread === 0) return false;
      if (filter === 'pinned' && !chat.pinned) return false;
      
      return true;
    });
  };
  
  // 切换分组折叠状态
  const toggleGroupCollapse = (groupId: string) => {
    setGroupCollapse(prev => ({
      ...prev,
      [groupId]: !prev[groupId]
    }));
  };

  // 获取置顶的聊天
  const pinnedChats = filterChats(chats.filter(chat => chat.pinned));
  
  return (
    <div className="w-80 border-r border-white/5 h-full bg-bg-secondary flex flex-col">
      {/* Header */}
      <div className="p-4 border-b border-white/5 flex justify-between items-center">
        <h2 className="text-lg font-semibold text-text-primary">消息</h2>
        <div className="flex space-x-2">
          <button 
            className="w-9 h-9 flex items-center justify-center rounded-full bg-white/5 text-text-primary hover:bg-accent-primary/10 hover:text-accent-primary transition"
            onClick={() => setShowFilterMenu(!showFilterMenu)}
          >
            <Filter size={18} />
          </button>
          <button className="w-9 h-9 flex items-center justify-center rounded-full bg-white/5 text-text-primary hover:bg-accent-primary/10 hover:text-accent-primary transition">
            <Plus size={18} />
          </button>
          <button className="w-9 h-9 flex items-center justify-center rounded-full bg-white/5 text-text-primary hover:bg-accent-primary/10 hover:text-accent-primary transition">
            <Settings size={18} />
          </button>
        </div>
        
        {/* Filter Menu */}
        {showFilterMenu && (
          <div className="absolute top-16 right-4 w-40 bg-bg-tertiary/90 backdrop-blur-md rounded-lg shadow-lg border border-white/5 z-50">
            <div className="py-2">
              <button
                className={`flex items-center w-full px-4 py-2 hover:bg-white/5 text-left ${filter === 'all' ? 'text-accent-primary' : 'text-text-primary'}`}
                onClick={() => {
                  setFilter('all');
                  setShowFilterMenu(false);
                }}
              >
                所有消息
              </button>
              <button
                className={`flex items-center w-full px-4 py-2 hover:bg-white/5 text-left ${filter === 'unread' ? 'text-accent-primary' : 'text-text-primary'}`}
                onClick={() => {
                  setFilter('unread');
                  setShowFilterMenu(false);
                }}
              >
                未读消息
              </button>
              <button
                className={`flex items-center w-full px-4 py-2 hover:bg-white/5 text-left ${filter === 'pinned' ? 'text-accent-primary' : 'text-text-primary'}`}
                onClick={() => {
                  setFilter('pinned');
                  setShowFilterMenu(false);
                }}
              >
                置顶消息
              </button>
            </div>
          </div>
        )}
      </div>
      
      {/* Search */}
      <div className="p-4 border-b border-white/5">
        <div className="flex items-center gap-2 bg-white/5 rounded-lg px-4 py-2 focus-within:bg-accent-primary/10 focus-within:shadow-glow-sm transition duration-300">
          <Search size={18} className="text-text-secondary" />
          <input 
            type="text" 
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            placeholder="搜索" 
            className="bg-transparent w-full outline-none border-none text-text-primary placeholder:text-text-secondary" 
          />
        </div>
      </div>
      
      {/* Filter indicator */}
      {filter !== 'all' && (
        <div className="px-4 py-2 bg-accent-primary/5">
          <div className="flex items-center justify-between">
            <span className="text-sm text-accent-primary">
              {filter === 'unread' ? '仅显示未读消息' : '仅显示置顶消息'}
            </span>
            <button 
              className="text-accent-primary text-sm"
              onClick={() => setFilter('all')}
            >
              清除
            </button>
          </div>
        </div>
      )}
      
      {/* Chat List */}
      <div className="flex-1 overflow-y-auto py-2 scrollbar-thin scrollbar-thumb-accent-primary/30 scrollbar-track-transparent">
        {/* No results message */}
        {filterChats(chats).length === 0 && (
          <div className="flex flex-col items-center justify-center h-40 text-text-secondary p-4 text-center">
            <span className="text-lg mb-2">没有找到聊天</span>
            <span className="text-sm">尝试不同的搜索词或开始新的对话</span>
          </div>
        )}
        
        {/* Pinned chats */}
        {pinnedChats.length > 0 && (
          <div className="mb-2">
            <div className="px-4 py-2 flex items-center">
              <Pin size={14} className="text-accent-primary mr-2" />
              <span className="text-xs text-text-secondary font-medium">置顶会话</span>
            </div>
            
            {pinnedChats.map((chat) => (
              <ChatItem 
                key={chat.id}
                chat={chat}
                isSelected={selectedChatId === chat.id}
                onClick={() => onSelectChat(chat.id)}
              />
            ))}
          </div>
        )}
        
        {/* Chats without groups */}
        {!searchTerm && filter === 'all' && (
          <>
            {getChatsInGroup().length > 0 && (
              <div className="mb-2">
                {filterChats(getChatsInGroup()).map((chat) => (
                  <ChatItem 
                    key={chat.id}
                    chat={chat}
                    isSelected={selectedChatId === chat.id}
                    onClick={() => onSelectChat(chat.id)}
                  />
                ))}
              </div>
            )}
            
            {/* Grouped chats */}
            {groups.map((group) => {
              const groupChats = filterChats(getChatsInGroup(group.id));
              if (groupChats.length === 0) return null;
              
              return (
                <div key={group.id} className="mb-2">
                  <button 
                    className="w-full px-4 py-2 flex items-center justify-between text-text-secondary hover:text-text-primary transition-colors"
                    onClick={() => toggleGroupCollapse(group.id)}
                  >
                    <span className="text-xs font-medium">{group.name}</span>
                    <ChevronDown 
                      size={14} 
                      className={`transform transition-transform ${groupCollapse[group.id] ? 'rotate-180' : ''}`} 
                    />
                  </button>
                  
                  {!groupCollapse[group.id] && groupChats.map((chat) => (
                    <ChatItem 
                      key={chat.id}
                      chat={chat}
                      isSelected={selectedChatId === chat.id}
                      onClick={() => onSelectChat(chat.id)}
                    />
                  ))}
                </div>
              );
            })}
          </>
        )}
        
        {/* Filtered or searched chats */}
        {(searchTerm || filter !== 'all') && 
          filterChats(chats)
            .filter(chat => !chat.pinned || filter !== 'pinned')
            .map((chat) => (
              <ChatItem 
                key={chat.id}
                chat={chat}
                isSelected={selectedChatId === chat.id}
                onClick={() => onSelectChat(chat.id)}
              />
            ))
        }
      </div>
    </div>
  );
};

// Chat Item Component
const ChatItem: React.FC<{
  chat: Chat;
  isSelected: boolean;
  onClick: () => void;
}> = ({ chat, isSelected, onClick }) => {
  return (
    <div 
      className={`
        flex items-center gap-3 p-3 mx-2 rounded-lg cursor-pointer relative
        ${isSelected 
          ? 'bg-accent-primary/5 before:absolute before:left-0 before:top-0 before:bottom-0 before:w-1 before:bg-gradient-to-b before:from-accent-primary before:to-accent-secondary' 
          : 'hover:bg-white/5'
        }
        transition-colors duration-300
      `}
      onClick={onClick}
    >
      <Avatar text={chat.name} src={chat.avatar} status={chat.online ? 'online' : 'offline'} />
      <div className="flex-1 min-w-0">
        <div className="flex justify-between">
          <div className="flex items-center gap-1">
            <p className="font-semibold text-text-primary truncate">{chat.name}</p>
            {chat.pinned && <Pin size={12} className="text-accent-primary" />}
          </div>
          <span className="text-xs text-text-secondary">{chat.time}</span>
        </div>
        <div className="flex justify-between">
          <p className={`text-sm truncate ${chat.muted ? 'text-text-secondary/50' : 'text-text-secondary'}`}>
            {chat.lastMessage}
          </p>
          {chat.unread > 0 ? (
            <span className="bg-gradient-to-r from-accent-primary to-accent-secondary text-xs text-bg-primary font-semibold min-w-5 h-5 flex items-center justify-center rounded-full px-1.5">
              {chat.unread}
            </span>
          ) : chat.muted ? (
            <span className="w-3 h-3 flex items-center justify-center text-text-secondary">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" className="w-3 h-3">
                <path d="M10.707 2.293a1 1 0 00-1.414 0l-7 7a1 1 0 001.414 1.414L4 10.414V13a1 1 0 001 1h2a1 1 0 001-1v-2a1 1 0 011-1h2a1 1 0 011 1v2a1 1 0 001 1h2a1 1 0 001-1v-2.586l.293.293a1 1 0 001.414-1.414l-7-7z" />
              </svg>
            </span>
          ) : null}
        </div>
      </div>
    </div>
  );
};

export default ChatList;