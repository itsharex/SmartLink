// components/ai-chat/AiChatPage.tsx
"use client";

import React, { useState, useEffect } from 'react';
import SideNav from '@/components/layout/SideNav';
import AiChatList from '@/components/ai-chat/AiChatList';
import AiChatWindow from '@/components/ai-chat/AiChatWindow';
import { v4 as uuidv4 } from 'uuid';

export type AiChat = {
  id: string;
  topic: string;
  summary: string;
  time: string;
};

export type AiMessage = {
  id: string;
  content: string;
  sender: 'user' | 'other';
  timestamp: Date;
};

const sampleAiChats: AiChat[] = [
  { id: 'ai-1', topic: 'Travel Recommendations', summary: 'Find the best destinations for summer vacations', time: '10:30 AM' },
  { id: 'ai-2', topic: 'Recipe Ideas', summary: 'Quick recipes based on your ingredients', time: 'Yesterday' },
  { id: 'ai-3', topic: 'Fitness Tips', summary: 'New workout routines and diet plans', time: 'Monday' },
];

export default function AiChatPage() {
  const [selectedChatId, setSelectedChatId] = useState<string | null>(null);
  const [aiChats, setAiChats] = useState<AiChat[]>(sampleAiChats);
  const [aiMessages, setAiMessages] = useState<Record<string, AiMessage[]>>({
    'ai-1': [],
    'ai-2': [],
    'ai-3': [],
  });

  // Handle selecting a chat conversation from the list
  const handleSelectChat = (chatId: string) => {
    setSelectedChatId(chatId);
  };

  // When a user sends a message, update local state and stream the AI response.
  const handleSendAiMessage = async (content: string) => {
    if (!selectedChatId) return;
    
    // Create and add the user's message locally.
    const userMessage: AiMessage = {
      id: uuidv4(),
      content,
      sender: 'user',
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
      const response = await fetch('/api/groqChat', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ messages: conversationForAPI }),
      });
      
      if (!response.body) {
        console.error("No response body");
        return;
      }
      
      // Add a temporary streaming message.
      setAiMessages(prev => ({
        ...prev,
        [selectedChatId]: [
          ...prev[selectedChatId],
          { id: 'assistant_stream', content: "", sender: 'other', timestamp: new Date() }
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
          // Update the temporary streaming assistant message.
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
      
      // Once streaming is complete, replace the temporary ID with a new one.
      setAiMessages(prev => ({
        ...prev,
        [selectedChatId]: prev[selectedChatId].map(msg =>
          msg.id === 'assistant_stream'
            ? { ...msg, id: uuidv4() }
            : msg
        )
      }));
    } catch (error) {
      console.error("Error fetching AI response:", error);
    }
  };

  // Auto-select the first chat if none is selected.
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
        onSelectChat={handleSelectChat} 
        selectedChatId={selectedChatId || undefined} 
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
        <div className="flex-1 flex items-center justify-center bg-bg-secondary">
          <div className="text-center text-text-secondary">
            <p className="text-xl mb-2">Select a conversation to start chatting with AI</p>
          </div>
        </div>
      )}
    </div>
  );
}
