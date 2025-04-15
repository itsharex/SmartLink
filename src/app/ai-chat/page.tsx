"use client";

import React, { useState, useEffect } from 'react';
import SideNav from '@/components/layout/SideNav';
import AiChatList from '@/components/ai-chat/AiChatList';
import AiChatWindow from '@/components/ai-chat/AiChatWindow';
import { v4 as uuidv4 } from 'uuid';

export type AiChat = {
  id: string;
  topic: string;
  summary: string; // (Unused now for the list view)
  time: string;
};

export type AiMessage = {
  id: string;
  content: string;
  sender: 'user' | 'other';
  type: 'text' | 'image' | 'file' | 'voice';
  timestamp: Date;
};

export default function AiChatPage() {
  // Start with an empty list of chats and messages.
  const [selectedChatId, setSelectedChatId] = useState<string | null>(null);
  const [aiChats, setAiChats] = useState<AiChat[]>([]);
  const [aiMessages, setAiMessages] = useState<Record<string, AiMessage[]>>({});

  const handleSelectChat = (chatId: string) => {
    setSelectedChatId(chatId);
  };

  // Function to add a new chat.
  const handleAddChat = () => {
    const newChat: AiChat = {
      id: uuidv4(),
      topic: 'New Chat',
      summary: 'No summary yet', // This field is no longer used for display.
      time: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
    };
    setAiChats(prev => [newChat, ...prev]);
    setAiMessages(prev => ({ ...prev, [newChat.id]: [] }));
    setSelectedChatId(newChat.id);
  };

  // Function to update the chat title.
  const handleUpdateChatTitle = (chatId: string, newTitle: string) => {
    setAiChats(prev =>
      prev.map(chat =>
        chat.id === chatId ? { ...chat, topic: newTitle } : chat
      )
    );
  };

  const handleSendAiMessage = async (content: string) => {
    if (!selectedChatId) return;
    
    // Add the user's message.
    const userMessage: AiMessage = {
      id: uuidv4(),
      content,
      sender: 'user',
      type: 'text',
      timestamp: new Date(),
    };
    const updatedMessages = [...(aiMessages[selectedChatId] || []), userMessage];
    setAiMessages(prev => ({
      ...prev,
      [selectedChatId]: updatedMessages,
    }));
    
    // Prepare the conversation history for the API call.
    const conversationForAPI = updatedMessages.map(msg => ({
      role: msg.sender === 'user' ? 'user' : 'assistant',
      content: msg.content,
    }));
    
    try {
      const response = await fetch('/api/groq/ai-chat', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ messages: conversationForAPI }),
      });
      
      if (!response.body) {
        console.error("No response body");
        return;
      }
      
      // Add a temporary streaming assistant message.
      setAiMessages(prev => ({
        ...prev,
        [selectedChatId]: [
          ...prev[selectedChatId],
          { id: 'assistant_stream', content: "", sender: 'other', timestamp: new Date(), type: 'text' }
        ]
      }));
      
      const reader = response.body.getReader();
      const decoder = new TextDecoder();
      let done = false;
      let aiMessageContent = "";
      
      while (!done) {
        const { done: doneReading, value } = await reader.read();
        done = doneReading;
        if (value) {
          const chunk = decoder.decode(value);
          aiMessageContent += chunk;
          setAiMessages(prev => ({
            ...prev,
            [selectedChatId]: prev[selectedChatId].map(msg =>
              msg.id === 'assistant_stream'
                ? { ...msg, content: aiMessageContent }
                : msg
            )
          }));
        }
      }
      
      setAiMessages(prev => ({
        ...prev,
        [selectedChatId]: prev[selectedChatId].map(msg =>
          msg.id === 'assistant_stream'
            ? { ...msg, id: uuidv4() }
            : msg
        )
      }));

      // Optionally, update the chat title if it's a new chat.
      setAiChats(prev => prev.map(chat => {
        if (chat.id === selectedChatId && chat.topic === 'New Chat') {
          const words = aiMessageContent.split(' ');
          const title = words.slice(0, 5).join(' ') + (words.length > 5 ? '...' : '');
          return { ...chat, topic: title };
        }
        return chat;
      }));
      
    } catch (error) {
      console.error("Error fetching AI response:", error);
    }
  };

  // Auto-select the first chat if available.
  useEffect(() => {
    if (aiChats.length > 0 && !selectedChatId) {
      setSelectedChatId(aiChats[0].id);
    }
  }, [aiChats, selectedChatId]);

  return (
    <div className="flex h-screen">
      <SideNav userName="User" />
      <AiChatList 
        chats={aiChats} 
        aiMessages={aiMessages}
        onSelectChat={handleSelectChat} 
        selectedChatId={selectedChatId || undefined}
        onAddChat={handleAddChat}
        onUpdateChatTitle={handleUpdateChatTitle}
      />
      {selectedChatId ? (
        <div className="flex-1">
          <AiChatWindow 
            chatTopic={aiChats.find(c => c.id === selectedChatId)?.topic || ''}
            messages={aiMessages[selectedChatId] || []}
            onSendMessage={handleSendAiMessage}
          />
        </div>
      ) : (
        <div className="flex-1 flex items-center justify-center bg-bg-tertiary">
          <div className="text-center text-text-primary">
            <p className="text-md mb-2">Select a conversation or start a new conversation to chat with AI</p>
          </div>
        </div>
      )}
    </div>
  );
}
