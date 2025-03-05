'use client';

import React, { useState } from 'react';
import SideNav from '@/components/layout/SideNav';
import GlassCard from '@/components/ui/GlassCard';
import Avatar from '@/components/ui/Avatar';
import Button from '@/components/ui/Button';
import { Search, UserPlus, Star, Users, Settings, Phone, MessageSquare, Video } from 'lucide-react';

// 示例联系人数据
const contactsData = [
  { id: '1', name: '王小明', status: 'online', avatar: '', favorite: true, tags: ['同事', '项目A'] },
  { id: '2', name: '张伟', status: 'offline', avatar: '', favorite: false, tags: ['朋友'] },
  { id: '3', name: '刘明', status: 'away', avatar: '', favorite: true, tags: ['家人'] },
  { id: '4', name: '李华', status: 'busy', avatar: '', favorite: false, tags: ['同学', '项目B'] },
  { id: '5', name: '陈晨', status: 'online', avatar: '', favorite: false, tags: ['同事'] },
  // 添加更多联系人...
];

// 示例请求数据
const requestsData = [
  { id: '1', name: '赵强', avatar: '', timeAgo: '2天前' },
  { id: '2', name: '钱蓝', avatar: '', timeAgo: '1周前' },
];

export default function ContactsPage() {
  const [activeTab, setActiveTab] = useState<'all' | 'favorites' | 'groups'>('all');
  const [searchTerm, setSearchTerm] = useState('');
  
  // 过滤联系人
  const filteredContacts = contactsData.filter(contact => {
    // 按搜索词过滤
    if (searchTerm && !contact.name.toLowerCase().includes(searchTerm.toLowerCase())) {
      return false;
    }
    
    // 按标签过滤
    if (activeTab === 'favorites' && !contact.favorite) {
      return false;
    }
    
    return true;
  });

  return (
    <div className="flex h-screen overflow-hidden">
      {/* 侧边导航 */}
      <SideNav userName={''} />
      
      <div className="flex-1 bg-bg-primary p-6 overflow-auto">
        <div className="max-w-6xl mx-auto">
          <header className="mb-8">
            <h1 className="text-3xl font-bold font-orbitron mb-2 bg-gradient-to-r from-accent-primary to-accent-secondary bg-clip-text text-transparent">联系人</h1>
            <p className="text-text-secondary">管理你的联系人和群组</p>
          </header>
          
          <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
            {/* 左侧栏 */}
            <div className="md:col-span-1 space-y-6">
              {/* 搜索框 */}
              <GlassCard className="p-4">
                <div className="flex items-center gap-2 bg-white/5 rounded-lg px-4 py-2 focus-within:bg-accent-primary/10 focus-within:shadow-glow-sm transition duration-300">
                  <Search size={18} className="text-text-secondary" />
                  <input 
                    type="text" 
                    placeholder="搜索联系人" 
                    className="bg-transparent w-full outline-none border-none text-text-primary placeholder:text-text-secondary"
                    value={searchTerm}
                    onChange={(e) => setSearchTerm(e.target.value)}
                  />
                </div>
              </GlassCard>
              
              {/* 导航标签 */}
              <GlassCard className="overflow-hidden">
                <button 
                  className={`flex items-center gap-3 w-full p-4 text-left ${activeTab === 'all' ? 'bg-accent-primary/10 text-accent-primary' : 'text-text-primary hover:bg-white/5'}`}
                  onClick={() => setActiveTab('all')}
                >
                  <Users size={18} />
                  <span>所有联系人</span>
                </button>
                
                <button 
                  className={`flex items-center gap-3 w-full p-4 text-left ${activeTab === 'favorites' ? 'bg-accent-primary/10 text-accent-primary' : 'text-text-primary hover:bg-white/5'}`}
                  onClick={() => setActiveTab('favorites')}
                >
                  <Star size={18} />
                  <span>收藏</span>
                </button>
                
                <button 
                  className={`flex items-center gap-3 w-full p-4 text-left ${activeTab === 'groups' ? 'bg-accent-primary/10 text-accent-primary' : 'text-text-primary hover:bg-white/5'}`}
                  onClick={() => setActiveTab('groups')}
                >
                  <Users size={18} />
                  <span>群组</span>
                </button>
              </GlassCard>
              
              {/* 好友请求 */}
              <GlassCard className="p-4">
                <h3 className="text-lg font-semibold mb-4 flex items-center gap-2">
                  <UserPlus size={18} className="text-accent-primary" />
                  好友请求
                </h3>
                
                {requestsData.length === 0 ? (
                  <p className="text-text-secondary text-center py-4">暂无好友请求</p>
                ) : (
                  <div className="space-y-4">
                    {requestsData.map(request => (
                      <div key={request.id} className="flex items-center justify-between">
                        <div className="flex items-center gap-3">
                          <Avatar text={request.name} size="sm" />
                          <div>
                            <p className="font-medium text-text-primary">{request.name}</p>
                            <p className="text-xs text-text-secondary">{request.timeAgo}</p>
                          </div>
                        </div>
                        <div className="flex gap-2">
                          <Button variant="primary" size="sm">接受</Button>
                          <Button variant="outline" size="sm">拒绝</Button>
                        </div>
                      </div>
                    ))}
                  </div>
                )}
              </GlassCard>
            </div>
            
            {/* 联系人列表 */}
            <div className="md:col-span-3">
              <GlassCard className="p-6">
                <div className="flex justify-between items-center mb-6">
                  <h2 className="text-xl font-semibold">
                    {activeTab === 'all' ? '所有联系人' : 
                     activeTab === 'favorites' ? '收藏联系人' : '群组'}
                  </h2>
                  <Button variant="outline" size="sm">
                    <UserPlus size={16} className="mr-2" />
                    添加联系人
                  </Button>
                </div>
                
                {filteredContacts.length === 0 ? (
                  <div className="text-center py-12">
                    <p className="text-text-secondary mb-4">没有找到符合条件的联系人</p>
                    <Button variant="outline">添加新联系人</Button>
                  </div>
                ) : (
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                    {filteredContacts.map(contact => (
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
                          <button className="w-8 h-8 flex items-center justify-center rounded-full bg-white/5 text-text-primary hover:bg-accent-primary/10 hover:text-accent-primary transition">
                            <MessageSquare size={16} />
                          </button>
                          <button className="w-8 h-8 flex items-center justify-center rounded-full bg-white/5 text-text-primary hover:bg-accent-primary/10 hover:text-accent-primary transition">
                            <Phone size={16} />
                          </button>
                          <button className="w-8 h-8 flex items-center justify-center rounded-full bg-white/5 text-text-primary hover:bg-accent-primary/10 hover:text-accent-primary transition">
                            <Video size={16} />
                          </button>
                        </div>
                      </div>
                    ))}
                  </div>
                )}
              </GlassCard>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}