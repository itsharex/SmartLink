'use client';

import React, { useState } from 'react';
import SideNav from '@/components/layout/SideNav';
import GlassCard from '@/components/ui/GlassCard';
import Button from '@/components/ui/Button';
import Avatar from '@/components/ui/Avatar';
import { Search, Compass, TrendingUp, Users, Gift, Globe, UserPlus, ChevronRight } from 'lucide-react';

// 热门话题数据
const trendingTopics = [
  { id: '1', title: '新一代AI技术', posts: 1245, tag: '科技' },
  { id: '2', title: '春季旅行推荐', posts: 892, tag: '旅行' },
  { id: '3', title: '远程工作效率提升', posts: 756, tag: '职场' },
  { id: '4', title: '全球科技峰会', posts: 534, tag: '科技' },
];

// 推荐用户数据
const recommendedUsers = [
  { id: '1', name: '张三', avatar: '', bio: 'UI设计师 | 喜欢旅行和摄影', mutualFriends: 5 },
  { id: '2', name: '李四', avatar: '', bio: '软件工程师 | 科技爱好者', mutualFriends: 3 },
  { id: '3', name: '王五', avatar: '', bio: '数据分析师 | 音乐发烧友', mutualFriends: 1 },
];

// 热门群组数据
const popularGroups = [
  { id: '1', name: '设计灵感', avatar: '', members: 1245, category: '设计' },
  { id: '2', name: '前端开发', avatar: '', members: 986, category: '技术' },
  { id: '3', name: '摄影分享', avatar: '', members: 762, category: '摄影' },
];

export default function DiscoveryPage() {
  const [searchTerm, setSearchTerm] = useState('');

  return (
    <div className="flex h-screen overflow-hidden">
      {/* 侧边导航 */}
      <SideNav userName={''} />
      
      <div className="flex-1 bg-bg-primary overflow-auto">
        <div className="max-w-6xl mx-auto p-6">
          <header className="mb-8">
            <h1 className="text-3xl font-bold font-orbitron mb-2 bg-gradient-to-r from-accent-primary to-accent-secondary bg-clip-text text-transparent flex items-center gap-2">
              <Compass size={32} className="text-accent-primary" />
              发现
            </h1>
            <p className="text-text-secondary">探索好友、群组和热门话题</p>
          </header>
          
          {/* 搜索栏 */}
          <div className="mb-8">
            <div className="max-w-2xl mx-auto">
              <div className="flex items-center gap-2 bg-white/5 rounded-lg px-4 py-3 focus-within:bg-accent-primary/10 focus-within:shadow-glow-sm transition duration-300">
                <Search size={20} className="text-text-secondary" />
                <input 
                  type="text" 
                  placeholder="搜索用户、群组或话题..." 
                  className="bg-transparent w-full outline-none border-none text-text-primary placeholder:text-text-secondary text-lg"
                  value={searchTerm}
                  onChange={(e) => setSearchTerm(e.target.value)}
                />
              </div>
            </div>
          </div>
          {/* 探索内容 */}
          <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
            {/* 热门话题 */}
            <div className="space-y-6">
              <div className="flex items-center justify-between">
                <h2 className="text-xl font-semibold flex items-center gap-2">
                  <TrendingUp size={20} className="text-accent-primary" />
                  热门话题
                </h2>
                <button className="text-text-secondary hover:text-accent-primary transition">
                  查看全部 <ChevronRight size={16} className="inline" />
                </button>
              </div>
              
              <GlassCard className="overflow-hidden">
                {trendingTopics.map((topic, index) => (
                  <div 
                    key={topic.id}
                    className={`p-4 flex justify-between items-center ${index !== trendingTopics.length - 1 ? 'border-b border-white/5' : ''}`}
                  >
                    <div>
                      <h3 className="font-medium text-text-primary">{topic.title}</h3>
                      <div className="flex items-center gap-3 mt-1">
                        <span className="text-xs px-2 py-0.5 rounded bg-accent-primary/10 text-accent-primary">
                          #{topic.tag}
                        </span>
                        <span className="text-xs text-text-secondary">
                          {topic.posts} 个动态
                        </span>
                      </div>
                    </div>
                    <Button variant="outline" size="sm">浏览</Button>
                  </div>
                ))}
              </GlassCard>
            </div>
            
            {/* 推荐好友 */}
            <div className="space-y-6">
              <div className="flex items-center justify-between">
                <h2 className="text-xl font-semibold flex items-center gap-2">
                  <Users size={20} className="text-accent-primary" />
                  推荐好友
                </h2>
                <button className="text-text-secondary hover:text-accent-primary transition">
                  查看全部 <ChevronRight size={16} className="inline" />
                </button>
              </div>
              
              <GlassCard className="overflow-hidden">
                {recommendedUsers.map((user, index) => (
                  <div 
                    key={user.id}
                    className={`p-4 ${index !== recommendedUsers.length - 1 ? 'border-b border-white/5' : ''}`}
                  >
                    <div className="flex items-center gap-3 mb-3">
                      <Avatar text={user.name} />
                      <div>
                        <h3 className="font-medium text-text-primary">{user.name}</h3>
                        <p className="text-xs text-text-secondary">{user.mutualFriends} 个共同好友</p>
                      </div>
                    </div>
                    <p className="text-sm text-text-secondary mb-3">{user.bio}</p>
                    <div className="flex gap-2">
                      <Button variant="primary" size="sm">
                        <UserPlus size={14} className="mr-1" />
                        添加好友
                      </Button>
                      <Button variant="outline" size="sm">忽略</Button>
                    </div>
                  </div>
                ))}
              </GlassCard>
            </div>
            
            {/* 热门群组 */}
            <div className="space-y-6">
              <div className="flex items-center justify-between">
                <h2 className="text-xl font-semibold flex items-center gap-2">
                  <Users size={20} className="text-accent-primary" />
                  热门群组
                </h2>
                <button className="text-text-secondary hover:text-accent-primary transition">
                  查看全部 <ChevronRight size={16} className="inline" />
                </button>
              </div>
              
              <GlassCard className="overflow-hidden">
                {popularGroups.map((group, index) => (
                  <div 
                    key={group.id}
                    className={`p-4 flex justify-between items-center ${index !== popularGroups.length - 1 ? 'border-b border-white/5' : ''}`}
                  >
                    <div className="flex items-center gap-3">
                      <Avatar text={group.name} />
                      <div>
                        <h3 className="font-medium text-text-primary">{group.name}</h3>
                        <div className="flex items-center gap-2 mt-1">
                          <span className="text-xs px-2 py-0.5 rounded bg-accent-primary/10 text-accent-primary">
                            {group.category}
                          </span>
                          <span className="text-xs text-text-secondary">
                            {group.members} 个成员
                          </span>
                        </div>
                      </div>
                    </div>
                    <Button variant="outline" size="sm">加入</Button>
                  </div>
                ))}
              </GlassCard>
            </div>
          </div>
          
          {/* 活动广告 */}
          <div className="mt-12">
            <GlassCard className="p-6 relative overflow-hidden">
              <div className="relative z-10">
                <h2 className="text-2xl font-bold mb-2">加入科技峰会</h2>
                <p className="text-text-secondary mb-4 max-w-xl">探讨前沿技术趋势，连接全球科技爱好者，不要错过这个与业界专家面对面的机会。</p>
                <Button variant="primary">查看详情</Button>
              </div>
              <div className="absolute top-0 right-0 bottom-0 w-1/3 bg-gradient-to-l from-accent-primary/20 to-transparent pointer-events-none"></div>
            </GlassCard>
          </div>
        </div>
      </div>
    </div>
  );
}