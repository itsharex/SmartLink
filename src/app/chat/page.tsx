'use client';

import React, { useState, useEffect } from 'react';
import SideNav from '@/components/layout/SideNav';
import ChatList from '@/components/chat/ChatList';
import ChatWindow from '@/components/chat/ChatWindow';
import { type Message } from '@/components/chat/ChatWindow';
import { v4 as uuidv4 } from 'uuid';

// 示例数据
const sampleChats = [
  { id: '1', name: '王小明', avatar: '', lastMessage: '好的，下午见！', time: '09:42', unread: 2, online: true },
  { id: '2', name: '张伟', avatar: '', lastMessage: '项目文档已发送', time: '昨天', unread: 0, online: false },
  { id: '3', name: '项目小组', avatar: '', lastMessage: '刘明：明天会议取消', time: '昨天', unread: 0, online: true },
  { id: '4', name: 'AI 助手', avatar: '', lastMessage: '已创建会议提醒', time: '周一', unread: 0, online: true },
];

// 示例消息数据
const sampleMessages: Record<string, Message[]> = {
  '1': [
    { id: '1-1', content: '嗨，李明！今天下午有空吗？想请教你关于那个新项目的一些问题。', sender: 'other', timestamp: new Date(Date.now() - 3600000) },
    { id: '1-2', content: '你好，有空的。大概几点？', sender: 'user', timestamp: new Date(Date.now() - 3500000), status: 'read' },
    { id: '1-3', content: '下午3点怎么样？可以在公司的会议室A见面，已经预订好了。顺便，你能把上次我们讨论的那个原型设计文档带上吗？', sender: 'other', timestamp: new Date(Date.now() - 3400000) },
    { id: '1-4', content: '3点可以，我会把文档带上。需要我提前准备什么其他资料吗？', sender: 'user', timestamp: new Date(Date.now() - 3300000), status: 'read' },
    { id: '1-5', content: '暂时不需要其他资料，我们先讨论一下初步方案。等确定了再准备后续详细的。', sender: 'other', timestamp: new Date(Date.now() - 3200000) },
  ],
};

export default function ChatPage() {
  const [selectedChatId, setSelectedChatId] = useState<string | null>(null);
  const [chats, setChats] = useState(sampleChats);
  const [messages, setMessages] = useState<Record<string, Message[]>>(sampleMessages);
  
  // 处理选择聊天
  const handleSelectChat = (chatId: string) => {
    setSelectedChatId(chatId);
    
    // 标记为已读
    setChats(prev => 
      prev.map(chat => 
        chat.id === chatId ? { ...chat, unread: 0 } : chat
      )
    );
  };
  
  // 处理发送消息
  const handleSendMessage = (content: string, type: 'text' | 'image' | 'file' | 'voice') => {
    if (!selectedChatId) return;
    
    const newMessage: Message = {
      id: uuidv4(),
      content,
      sender: 'user',
      timestamp: new Date(),
      status: 'sent',
      type
    };
    
    // 添加消息到状态
    setMessages(prev => ({
      ...prev,
      [selectedChatId]: [...(prev[selectedChatId] || []), newMessage]
    }));
    
    // 模拟消息状态变化
    setTimeout(() => {
      setMessages(prev => {
        const updatedMessages = [...prev[selectedChatId]];
        const lastIndex = updatedMessages.length - 1;
        updatedMessages[lastIndex] = {
          ...updatedMessages[lastIndex],
          status: 'delivered'
        };
        return { ...prev, [selectedChatId]: updatedMessages };
      });
    }, 1000);
    
    // 模拟回复
    if (selectedChatId !== '4') { // 不是AI助手
      setTimeout(() => {
        const replyMessage: Message = {
          id: uuidv4(),
          content: '好的，我已收到你的消息。',
          sender: 'other',
          timestamp: new Date(),
        };
        
        setMessages(prev => ({
          ...prev,
          [selectedChatId]: [...prev[selectedChatId], replyMessage]
        }));
        
        // 如果不是当前聊天，增加未读数
        if (selectedChatId) {
          setChats(prev => 
            prev.map(chat => 
              chat.id === selectedChatId 
                ? { ...chat, lastMessage: content, time: '刚刚' } 
                : chat
            )
          );
        }
      }, 3000);
    }
  };
  
  // 默认选择第一个聊天
  useEffect(() => {
    if (chats.length > 0 && !selectedChatId) {
      setSelectedChatId(chats[0].id);
    }
  }, [chats, selectedChatId]);

  return (
    <div className="flex h-screen overflow-hidden">
      {/* 侧边导航 */}
      <SideNav />
      
      {/* 聊天列表 */}
      <ChatList 
        chats={chats}
        onSelectChat={handleSelectChat}
        selectedChatId={selectedChatId || undefined}
      />
      
      {/* 聊天窗口 */}
      {selectedChatId ? (
        <div className="flex-1">
          <ChatWindow 
            contactName={chats.find(c => c.id === selectedChatId)?.name || ''}
            contactStatus={chats.find(c => c.id === selectedChatId)?.online ? 'online' : 'offline'}
            messages={messages[selectedChatId] || []}
            onSendMessage={handleSendMessage}
          />
        </div>
      ) : (
        <div className="flex-1 flex items-center justify-center bg-bg-secondary">
          <div className="text-center text-text-secondary">
            <p className="text-xl mb-2">选择一个聊天开始交流</p>
            <p>或者创建新的对话</p>
          </div>
        </div>
      )}
    </div>
  );
}