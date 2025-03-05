import React, { useState, useRef, useEffect } from 'react';
import MessageBubble from './MessageBubble';
import ChatInput from './ChatInput';
import Avatar from '../ui/Avatar';
import EncryptionIndicator from '../features/EncryptionIndicator';
import { Phone, Video, MoreVertical, PlusCircle, Search, ShieldCheck, Lock } from 'lucide-react';

export type Message = {
  id: string;
  content: string;
  sender: 'user' | 'other';
  timestamp: Date;
  status?: 'sent' | 'delivered' | 'read';
  type?: 'text' | 'image' | 'file' | 'voice';
  media?: string;
};

type ChatWindowProps = {
  contactName: string;
  contactStatus?: 'online' | 'offline' | 'away' | 'busy';
  messages: Message[];
  isEncrypted?: boolean;
  isGroup?: boolean;
  onSendMessage: (content: string, type: 'text' | 'image' | 'file' | 'voice') => void;
};

const ChatWindow: React.FC<ChatWindowProps> = ({
  contactName,
  contactStatus = 'online',
  messages,
  isEncrypted = true,
  isGroup = false,
  onSendMessage
}) => {
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const [showEncryptionInfo, setShowEncryptionInfo] = useState(false);
  
  // Scroll to bottom when messages change
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  return (
    <div className="flex flex-col h-full bg-bg-secondary">
      {/* Chat Header */}
      <div className="p-4 border-b border-white/5 flex justify-between items-center">
        <div className="flex items-center gap-3">
          <Avatar text={contactName} status={contactStatus} />
          <div>
            <h3 className="font-semibold text-text-primary">{contactName}</h3>
            <div className="flex items-center gap-2">
              <p className="text-sm text-text-secondary">
                {contactStatus === 'online' ? '在线' : 
                 contactStatus === 'away' ? '离开' : 
                 contactStatus === 'busy' ? '忙碌中' : '离线'}
              </p>
              
              {/* Encryption Indicator */}
              {isEncrypted && (
                <div 
                  className="flex items-center gap-1 text-xs text-accent-primary cursor-pointer"
                  onClick={() => setShowEncryptionInfo(!showEncryptionInfo)}
                >
                  <Lock size={12} />
                  <span>端到端加密</span>
                </div>
              )}
            </div>
          </div>
        </div>
        <div className="flex gap-2">
          <button className="w-9 h-9 flex items-center justify-center rounded-full bg-white/5 text-text-primary hover:bg-accent-primary/10 hover:text-accent-primary transition">
            <Search size={18} />
          </button>
          <button className="w-9 h-9 flex items-center justify-center rounded-full bg-white/5 text-text-primary hover:bg-accent-primary/10 hover:text-accent-primary transition">
            <Phone size={18} />
          </button>
          <button className="w-9 h-9 flex items-center justify-center rounded-full bg-white/5 text-text-primary hover:bg-accent-primary/10 hover:text-accent-primary transition">
            <Video size={18} />
          </button>
          <button className="w-9 h-9 flex items-center justify-center rounded-full bg-white/5 text-text-primary hover:bg-accent-primary/10 hover:text-accent-primary transition">
            <MoreVertical size={18} />
          </button>
        </div>
      </div>
      
      {/* Encryption Info Panel */}
      {showEncryptionInfo && (
        <div className="p-3 bg-accent-primary/10 border-b border-accent-primary/20 flex items-center gap-3">
          <div className="w-10 h-10 rounded-full bg-accent-primary/20 flex items-center justify-center text-accent-primary">
            <ShieldCheck size={20} />
          </div>
          <div className="flex-1">
            <p className="text-sm text-text-primary">此对话使用端到端加密，仅对话双方可以查看内容。</p>
            <button className="text-xs text-accent-primary mt-1">了解更多</button>
          </div>
          <button 
            className="text-text-secondary hover:text-text-primary"
            onClick={() => setShowEncryptionInfo(false)}
          >
            ×
          </button>
        </div>
      )}
      
      {/* Messages */}
      <div className="flex-1 p-6 overflow-y-auto bg-bg-primary/30 flex flex-col gap-6">
        {messages.map((message) => (
          <MessageBubble
            key={message.id}
            message={message}
            showAvatar={true}
          />
        ))}
        <div ref={messagesEndRef} />
      </div>
      
      {/* Input Area */}
      <ChatInput onSendMessage={onSendMessage} />
    </div>
  );
};

export default ChatWindow;