// components/ai-chat/AiChatWindow.tsx
import React, { useRef, useEffect } from 'react';
import ChatInput from '@/components/chat/ChatInput';
import MessageBubble from '@/components/chat/MessageBubble';

export type AiMessage = {
    id: string;
    content: string;
    sender: 'user' | 'other';
    timestamp: Date;
  };

type AiChatWindowProps = {
  chatTopic: string;
  messages: AiMessage[];
  onSendMessage: (content: string) => void;
};

const AiChatWindow: React.FC<AiChatWindowProps> = ({ chatTopic, messages, onSendMessage }) => {
  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  return (
    <div className="flex flex-col h-full bg-bg-primary">
      {/* Simplified Chat Header */}
      <div className="p-4 h-20 border-b border-text-primary-5 flex items-center shadow-[0_4px_10px_rgba(0,0,0,0.05)]">
        <div>
          <h3 className="font-semibold text-text-primary">{chatTopic}</h3>
          <p className="text-sm text-text-primary-30">AI Chatbot</p>
        </div>
      </div>
      
      {/* Message List */}
      <div className="flex-1 p-6 overflow-y-auto flex flex-col gap-6">
        {messages.map((message) => (
          <MessageBubble key={message.id} message={message} showAvatar={true} />
        ))}
        <div ref={messagesEndRef} />
      </div>
      
      {/* Chat Input */}
      <ChatInput onSendMessage={onSendMessage} />
    </div>
  );
};

export default AiChatWindow;
