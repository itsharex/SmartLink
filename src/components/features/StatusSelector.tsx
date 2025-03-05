import React, { useState, useRef, useEffect } from 'react';
import { ChevronDown } from 'lucide-react';
import Avatar from '../ui/Avatar';

type UserStatus = 'online' | 'away' | 'busy' | 'offline';

type StatusSelectorProps = {
  currentStatus: UserStatus;
  onStatusChange: (status: UserStatus) => void;
  userName: string;
  userAvatar?: string;
};

const StatusSelector: React.FC<StatusSelectorProps> = ({
  currentStatus,
  onStatusChange,
  userName,
  userAvatar
}) => {
  const [isOpen, setIsOpen] = useState(false);
  const menuRef = useRef<HTMLDivElement>(null);
  
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
  
  // 自定义状态消息
  const [customStatus, setCustomStatus] = useState<string>('');
  
  // 点击外部关闭菜单
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (menuRef.current && !menuRef.current.contains(event.target as Node)) {
        setIsOpen(false);
      }
    };
    
    if (isOpen) {
      document.addEventListener('mousedown', handleClickOutside);
    }
    
    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, [isOpen]);
  
  return (
    <div className="relative" ref={menuRef}>
      <button 
        className="flex items-center gap-2 px-3 py-2 rounded-lg hover:bg-white/5 transition"
        onClick={() => setIsOpen(!isOpen)}
      >
        <Avatar text={userName} src={userAvatar} status={currentStatus} size="sm" />
        <div className="text-left">
          <p className="font-medium text-text-primary">{userName}</p>
          <div className="flex items-center gap-1 text-xs text-text-secondary">
            <span className={`w-1.5 h-1.5 rounded-full ${statusColors[currentStatus]}`}></span>
            <span>{statusText[currentStatus]}</span>
            {customStatus && <span>· {customStatus}</span>}
          </div>
        </div>
        <ChevronDown size={16} className={`text-text-secondary transform transition-transform ${isOpen ? 'rotate-180' : ''}`} />
      </button>
      
      {isOpen && (
        <div className="absolute top-full left-0 mt-2 w-64 bg-bg-tertiary/90 backdrop-blur-md rounded-lg shadow-lg border border-white/5 z-50">
          <div className="p-4 border-b border-white/10">
            <div className="flex items-center gap-3 mb-3">
              <Avatar text={userName} src={userAvatar} size="md" glow />
              <div>
                <p className="font-medium text-text-primary">{userName}</p>
                <p className="text-xs text-text-secondary">ID: user1234</p>
              </div>
            </div>
            
            <input 
              type="text" 
              placeholder="设置状态消息..." 
              value={customStatus}
              onChange={(e) => setCustomStatus(e.target.value)}
              className="w-full bg-white/5 border border-white/10 rounded-lg px-3 py-2 text-sm text-text-primary focus:outline-none focus:ring-1 focus:ring-accent-primary/50 transition"
            />
          </div>
          
          <div className="py-2">
            {(Object.keys(statusText) as UserStatus[]).map(status => (
              <button
                key={status}
                className={`flex items-center w-full px-4 py-2 hover:bg-white/5 text-left ${currentStatus === status ? 'bg-white/5' : ''}`}
                onClick={() => {
                  onStatusChange(status);
                  setIsOpen(false);
                }}
              >
                <span className={`w-2 h-2 rounded-full ${statusColors[status]} mr-2`}></span>
                <span className="text-text-primary">{statusText[status]}</span>
              </button>
            ))}
          </div>
          
          <div className="border-t border-white/10 p-3">
            <button 
              className="w-full text-center text-accent-primary text-sm hover:underline"
            >
              管理个人资料
            </button>
          </div>
        </div>
      )}
    </div>
  );
};

export default StatusSelector;