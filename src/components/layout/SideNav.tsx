"use client"

import React, { useState } from 'react';
import Avatar from '../ui/Avatar';
import { 
  MessageSquare, Users, Compass, Bell, Settings, LogOut, 
  Camera, Plus, ChevronDown, Moon, Shield
} from 'lucide-react';
import { useRouter } from 'next/navigation';

type NavItem = {
  icon: React.ReactNode;
  label: string;
  path: string;
  badge?: number;
};

type UserStatus = 'online' | 'away' | 'busy' | 'offline';

type SideNavProps = {
  userName: string;
  userAvatar?: string;
}

const SideNav: React.FC<SideNavProps> = ({ userName, userAvatar }) => {
  const router = useRouter();
  const [userStatus, setUserStatus] = useState<UserStatus>('online');
  const [statusMenuOpen, setStatusMenuOpen] = useState(false);

  const navItems: NavItem[] = [
    { icon: <MessageSquare size={22} />, label: '聊天', path: '/chat', badge: 3 },
    { icon: <Users size={22} />, label: '联系人', path: '/contacts' },
    { icon: <Compass size={22} />, label: '发现', path: '/discovery' },
    { icon: <Camera size={22} />, label: '动态', path: '/moments' },
    { icon: <Bell size={22} />, label: '通知', path: '/notifications', badge: 2 },
    { icon: <Settings size={22} />, label: '设置', path: '/settings' },
  ];

  // 状态显示文本
  const statusText = {
    online: '在线',
    away: '离开',
    busy: '忙碌',
    offline: '隐身'
  };

  // 状态颜色映射
  const statusColors = {
    online: 'bg-green-500',
    away: 'bg-yellow-500',
    busy: 'bg-red-500',
    offline: 'bg-gray-500'
  };

  const handleNavigation = (path: string) => {
    router.push(path);
  };

  return (
    <div className="w-20 border-r border-white/5 h-screen bg-bg-secondary flex flex-col items-center pt-titlebar">
      {/* 用户头像和状态 */}
      <div className="mt-6 mb-10 relative">
        <div className="relative">
          <Avatar 
            text={userName} 
            src={userAvatar}
            size="lg" 
            status={userStatus}
            glow
          />
          <button 
            className="absolute bottom-0 right-0 w-6 h-6 rounded-full bg-bg-secondary border border-white/10 flex items-center justify-center text-text-secondary hover:text-accent-primary transition-colors"
            onClick={() => setStatusMenuOpen(!statusMenuOpen)}
          >
            <ChevronDown size={14} />
          </button>
        </div>
        
        {/* 状态选择菜单 */}
        {statusMenuOpen && (
          <div className="absolute top-full mt-2 right-0 w-40 bg-bg-tertiary/90 backdrop-blur-md rounded-lg shadow-lg border border-white/5 z-50">
            <div className="py-2">
              {(Object.keys(statusText) as UserStatus[]).map(status => (
                <button
                  key={status}
                  className="flex items-center w-full px-4 py-2 hover:bg-white/5 text-left"
                  onClick={() => {
                    setUserStatus(status);
                    setStatusMenuOpen(false);
                  }}
                >
                  <span className={`w-2 h-2 rounded-full ${statusColors[status]} mr-2`}></span>
                  <span className="text-text-primary">{statusText[status]}</span>
                </button>
              ))}
            </div>
          </div>
        )}
      </div>
      
      {/* 导航项 */}
      <div className="flex-1 flex flex-col items-center gap-7 py-4">
        {navItems.map((item) => (
          <button
            key={item.label}
            className="group relative w-12 h-12 flex items-center justify-center rounded-xl hover:bg-accent-primary/10 transition-colors"
            onClick={() => handleNavigation(item.path)}
          >
            <span className="text-text-secondary group-hover:text-accent-primary transition-colors">
              {item.icon}
            </span>
            
            {/* Badge */}
            {item.badge && (
              <span className="absolute top-0 right-0 w-5 h-5 rounded-full bg-accent-tertiary text-xs flex items-center justify-center text-white font-medium">
                {item.badge}
              </span>
            )}
            
            {/* Tooltip */}
            <span className="absolute left-full ml-2 px-2 py-1 bg-bg-tertiary/90 backdrop-blur-md rounded text-sm text-text-primary whitespace-nowrap opacity-0 invisible group-hover:opacity-100 group-hover:visible transition-all">
              {item.label}
            </span>
          </button>
        ))}
      </div>
      
      {/* Create new chat button */}
      <button className="w-12 h-12 flex items-center justify-center rounded-xl bg-gradient-to-r from-accent-primary to-accent-secondary text-bg-primary mb-4 hover:shadow-glow-sm transition-all">
        <Plus size={20} />
      </button>
      
      {/* Logout */}
      <button className="w-12 h-12 flex items-center justify-center rounded-xl text-text-secondary hover:text-red-400 hover:bg-red-400/10 mb-6 transition-colors">
        <LogOut size={20} />
      </button>
    </div>
  );
};

export default SideNav;