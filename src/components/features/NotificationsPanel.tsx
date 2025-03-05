import React, { useState } from 'react';
import GlassCard from '../ui/GlassCard';
import Avatar from '../ui/Avatar';
import { Bell, Settings, Trash2, X, MessageSquare, UserPlus, Calendar, Info } from 'lucide-react';

type NotificationType = 'message' | 'friend' | 'system' | 'event';

type Notification = {
  id: string;
  type: NotificationType;
  title: string;
  description: string;
  time: string;
  read: boolean;
  sender?: {
    name: string;
    avatar?: string;
  };
  action?: string;
};

type NotificationsPanelProps = {
  onClose: () => void;
};

const NotificationsPanel: React.FC<NotificationsPanelProps> = ({ onClose }) => {
  // 示例通知数据
  const [notifications, setNotifications] = useState<Notification[]>([
    {
      id: '1',
      type: 'message',
      title: '新消息',
      description: '李明: 你好，我们下午3点开会，别忘了带资料。',
      time: '5分钟前',
      read: false,
      sender: {
        name: '李明',
        avatar: ''
      }
    },
    {
      id: '2',
      type: 'friend',
      title: '好友请求',
      description: '张伟请求添加您为好友',
      time: '30分钟前',
      read: false,
      sender: {
        name: '张伟',
        avatar: ''
      },
      action: '接受'
    },
    {
      id: '3',
      type: 'event',
      title: '日程提醒',
      description: '明天上午10点 - 项目启动会议',
      time: '1小时前',
      read: true
    },
    {
      id: '4',
      type: 'system',
      title: '系统通知',
      description: 'SmartLink 已更新到最新版本 v1.2.0',
      time: '昨天',
      read: true
    }
  ]);
  
  const [activeFilter, setActiveFilter] = useState<'all' | NotificationType>('all');
  
  // 标记所有为已读
  const markAllAsRead = () => {
    setNotifications(prev => prev.map(n => ({ ...n, read: true })));
  };
  
  // 清除所有通知
  const clearAll = () => {
    setNotifications([]);
  };
  
  // 获取特定类型的图标
  const getTypeIcon = (type: NotificationType) => {
    switch (type) {
      case 'message':
        return <MessageSquare size={16} />;
      case 'friend':
        return <UserPlus size={16} />;
      case 'system':
        return <Info size={16} />;
      case 'event':
        return <Calendar size={16} />;
    }
  };
  
  // 过滤通知
  const filteredNotifications = activeFilter === 'all' 
    ? notifications 
    : notifications.filter(n => n.type === activeFilter);
  
  // 未读通知数量
  const unreadCount = notifications.filter(n => !n.read).length;

  return (
    <GlassCard className="w-96 h-[85vh] max-h-[700px] flex flex-col border border-white/10 shadow-2xl">
      {/* Header */}
      <div className="flex items-center justify-between p-4 border-b border-white/10">
        <div className="flex items-center gap-2">
          <Bell size={18} className="text-accent-primary" />
          <h3 className="font-semibold text-text-primary">通知</h3>
          {unreadCount > 0 && (
            <span className="px-1.5 py-0.5 text-xs bg-accent-primary/20 text-accent-primary rounded-full">
              {unreadCount}
            </span>
          )}
        </div>
        
        <div className="flex items-center gap-2">
          <button className="text-text-secondary hover:text-text-primary transition" title="通知设置">
            <Settings size={16} />
          </button>
          <button 
            className="text-text-secondary hover:text-text-primary transition" 
            title="清除所有通知"
            onClick={clearAll}
          >
            <Trash2 size={16} />
          </button>
          <button 
            className="text-text-secondary hover:text-text-primary transition" 
            onClick={onClose}
          >
            <X size={18} />
          </button>
        </div>
      </div>
      
      {/* Filters */}
      <div className="flex p-2 gap-1 border-b border-white/10">
        <button 
          className={`flex-1 py-1.5 px-2 text-sm rounded ${activeFilter === 'all' ? 'bg-white/10 text-text-primary' : 'text-text-secondary'}`}
          onClick={() => setActiveFilter('all')}
        >
          全部
        </button>
        <button 
          className={`flex-1 py-1.5 px-2 text-sm rounded ${activeFilter === 'message' ? 'bg-white/10 text-text-primary' : 'text-text-secondary'}`}
          onClick={() => setActiveFilter('message')}
        >
          消息
        </button>
        <button 
          className={`flex-1 py-1.5 px-2 text-sm rounded ${activeFilter === 'friend' ? 'bg-white/10 text-text-primary' : 'text-text-secondary'}`}
          onClick={() => setActiveFilter('friend')}
        >
          好友
        </button>
        <button 
          className={`flex-1 py-1.5 px-2 text-sm rounded ${activeFilter === 'system' ? 'bg-white/10 text-text-primary' : 'text-text-secondary'}`}
          onClick={() => setActiveFilter('system')}
        >
          系统
        </button>
      </div>
      
      {/* Notifications List */}
      <div className="flex-1 overflow-y-auto py-2">
        {filteredNotifications.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-full text-text-secondary p-4">
            <Bell size={40} className="opacity-20 mb-4" />
            <p>暂无通知</p>
          </div>
        ) : (
          filteredNotifications.map((notification) => (
            <div 
              key={notification.id}
              className={`p-4 border-b border-white/5 hover:bg-white/5 transition ${!notification.read ? 'bg-accent-primary/5' : ''}`}
            >
              <div className="flex gap-3">
                {notification.sender ? (
                  <Avatar text={notification.sender.name} src={notification.sender.avatar} size="md" />
                ) : (
                  <div className={`w-10 h-10 rounded-full flex items-center justify-center ${notification.type === 'system' ? 'bg-accent-secondary/20 text-accent-secondary' : 'bg-accent-primary/20 text-accent-primary'}`}>
                    {getTypeIcon(notification.type)}
                  </div>
                )}
                
                <div className="flex-1 min-w-0">
                  <div className="flex justify-between">
                    <p className="font-semibold text-text-primary">{notification.title}</p>
                    <span className="text-xs text-text-secondary">{notification.time}</span>
                  </div>
                  <p className="text-sm text-text-secondary mt-1 line-clamp-2">{notification.description}</p>
                  
                  {notification.action && (
                    <button className="mt-2 px-3 py-1 text-xs bg-accent-primary/10 text-accent-primary rounded hover:bg-accent-primary/20 transition">
                      {notification.action}
                    </button>
                  )}
                </div>
              </div>
            </div>
          ))
        )}
      </div>
      
      {/* Footer */}
      {notifications.length > 0 && (
        <div className="p-3 border-t border-white/10">
          <button 
            className="w-full py-2 text-sm text-accent-primary hover:underline"
            onClick={markAllAsRead}
          >
            标记所有为已读
          </button>
        </div>
      )}
    </GlassCard>
  );
};

export default NotificationsPanel;