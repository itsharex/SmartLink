"use client";

import React, { useState, useRef, useEffect } from 'react';
import GlassCard from '../ui/GlassCard';
import Button from '../ui/Button';
import { Sparkles, X, Send, Bot, Mic, Volume2 } from 'lucide-react';

type Message = {
  id: string;
  content: string;
  role: 'user' | 'assistant';
  timestamp: Date;
};

const AiAssistant: React.FC = () => {
  const [isOpen, setIsOpen] = useState(false);
  const [messages, setMessages] = useState<Message[]>([
    {
      id: '1',
      content: '你好，我是 SmartLink 的 AI 助手。有什么我能帮到你的吗？',
      role: 'assistant',
      timestamp: new Date()
    }
  ]);
  const [inputValue, setInputValue] = useState('');
  const [isProcessing, setIsProcessing] = useState(false);
  const endOfMessagesRef = useRef<HTMLDivElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  
  // Automatically scroll to bottom when messages change
  useEffect(() => {
    endOfMessagesRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);
  
  // Close assistant when clicking outside
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      // 检查 containerRef 是否存在
      if (!containerRef.current) return;

      // 确保 event.target 是 HTMLElement 类型，并检查非空
      const target = event.target as Node | null;
      if (!target || !(target instanceof HTMLElement)) return;

      // 检查点击是否在 containerRef 外部且不是 ai-toggle-button
      if (
        !containerRef.current.contains(target) &&
        !target.classList.contains('ai-toggle-button')
      ) {
        setIsOpen(false);
      }
    };

    document.addEventListener('mousedown', handleClickOutside);
    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, []);
  
  // Send message
  const handleSendMessage = () => {
    if (!inputValue.trim()) return;
    
    // Add user message
    const userMessage: Message = {
      id: Date.now().toString(),
      content: inputValue,
      role: 'user',
      timestamp: new Date()
    };
    
    setMessages(prev => [...prev, userMessage]);
    setInputValue('');
    setIsProcessing(true);
    
    // Simulate AI response (in a real app, you'd call an API here)
    setTimeout(() => {
      const aiResponse: Message = {
        id: (Date.now() + 1).toString(),
        content: getAIResponse(inputValue),
        role: 'assistant',
        timestamp: new Date()
      };
      
      setMessages(prev => [...prev, aiResponse]);
      setIsProcessing(false);
    }, 1500);
  };
  
  // Simple AI response generator (would be replaced with actual AI in a real app)
  const getAIResponse = (query: string): string => {
    if (query.toLowerCase().includes('翻译')) {
      return '我可以帮你翻译文本。请告诉我你想翻译什么内容，以及目标语言。';
    }
    if (query.toLowerCase().includes('天气')) {
      return '目前我无法获取实时天气信息，但我可以帮你连接到天气服务。';
    }
    if (query.toLowerCase().includes('提醒') || query.toLowerCase().includes('备忘录')) {
      return '我已创建提醒。你想什么时候收到通知？';
    }
    if (query.toLowerCase().includes('谢谢') || query.toLowerCase().includes('感谢')) {
      return '不客气！如果有任何其他问题，随时告诉我。';
    }
    
    return '我理解你的问题。作为 SmartLink 的 AI 助手，我可以帮助翻译文本、创建提醒、语音转文字等。有什么特定的任务需要我协助吗？';
  };
  
  // Format timestamp
  const formatTime = (date: Date): string => {
    return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  };
  
  return (
    <>
      {/* Floating Button */}
      <button 
        className="fixed bottom-8 right-8 w-14 h-14 rounded-full bg-gradient-to-r from-accent-primary to-accent-secondary text-bg-primary shadow-glow-primary flex items-center justify-center transition duration-300 hover:transform hover:-translate-y-1 hover:shadow-glow-md z-50 ai-toggle-button"
        onClick={() => setIsOpen(!isOpen)}
      >
        <Sparkles size={24} />
      </button>
      
      {/* AI Assistant Panel */}
      {isOpen && (
        <div 
          ref={containerRef}
          className="fixed bottom-28 right-8 w-96 shadow-2xl z-50 transition-all duration-300"
          style={{ 
            transform: isOpen ? 'translateY(0)' : 'translateY(20px)',
            opacity: isOpen ? 1 : 0
          }}
        >
          <GlassCard className="overflow-hidden">
            {/* Header */}
            <div className="bg-bg-tertiary/80 px-4 py-3 flex justify-between items-center border-b border-white/5">
              <div className="flex items-center gap-2">
                <Bot size={20} className="text-accent-primary" />
                <h3 className="font-semibold text-text-primary">AI 助手</h3>
              </div>
              <button 
                className="w-8 h-8 flex items-center justify-center rounded-full hover:bg-white/10 text-text-secondary hover:text-text-primary transition"
                onClick={() => setIsOpen(false)}
              >
                <X size={18} />
              </button>
            </div>
            
            {/* Messages Area */}
            <div className="h-96 overflow-y-auto p-4 space-y-4">
              {messages.map((message) => (
                <div 
                  key={message.id}
                  className={`flex ${message.role === 'assistant' ? 'justify-start' : 'justify-end'}`}
                >
                  <div 
                    className={`
                      max-w-[80%] px-4 py-3 rounded-2xl
                      ${message.role === 'assistant' 
                        ? 'bg-bg-tertiary/80 rounded-tl-none' 
                        : 'bg-gradient-to-br from-accent-primary/20 to-accent-secondary/20 rounded-br-none'
                      }
                    `}
                  >
                    <p className="text-text-primary">{message.content}</p>
                    <p className="text-right text-xs text-text-secondary mt-1">
                      {formatTime(message.timestamp)}
                    </p>
                  </div>
                </div>
              ))}
              
              {/* Processing indicator */}
              {isProcessing && (
                <div className="flex justify-start">
                  <div className="bg-bg-tertiary/80 px-4 py-3 rounded-2xl rounded-tl-none">
                    <div className="flex gap-1">
                      <span className="w-2 h-2 bg-text-secondary rounded-full animate-bounce" style={{ animationDelay: '0ms' }} />
                      <span className="w-2 h-2 bg-text-secondary rounded-full animate-bounce" style={{ animationDelay: '150ms' }} />
                      <span className="w-2 h-2 bg-text-secondary rounded-full animate-bounce" style={{ animationDelay: '300ms' }} />
                    </div>
                  </div>
                </div>
              )}
              
              {/* Invisible element to scroll to */}
              <div ref={endOfMessagesRef} />
            </div>
            
            {/* Input Area */}
            <div className="p-4 border-t border-white/5">
              <div className="flex gap-2">
                <button className="text-text-secondary hover:text-accent-primary transition">
                  <Mic size={20} />
                </button>
                <input 
                  type="text" 
                  value={inputValue}
                  onChange={(e) => setInputValue(e.target.value)}
                  placeholder="发送消息给 AI 助手..." 
                  className="flex-1 bg-white/5 rounded-lg px-4 py-2 text-text-primary placeholder:text-text-secondary outline-none focus:bg-accent-primary/10 focus:shadow-glow-sm transition"
                  onKeyDown={(e) => e.key === 'Enter' && handleSendMessage()}
                />
                <button 
                  className={`
                    w-10 h-10 flex items-center justify-center rounded-full
                    ${inputValue.trim() 
                      ? 'bg-gradient-to-r from-accent-primary to-accent-secondary text-bg-primary' 
                      : 'bg-white/10 text-text-secondary'
                    }
                    transition
                  `}
                  onClick={handleSendMessage}
                  disabled={!inputValue.trim()}
                >
                  <Send size={18} />
                </button>
              </div>
              
              {/* Quick Actions */}
              <div className="flex gap-2 mt-3">
                <Button variant="outline" size="sm" className="text-xs py-1 px-2">
                  翻译
                </Button>
                <Button variant="outline" size="sm" className="text-xs py-1 px-2">
                  总结消息
                </Button>
                <Button variant="outline" size="sm" className="text-xs py-1 px-2">
                  语音转文字
                </Button>
              </div>
            </div>
          </GlassCard>
        </div>
      )}
    </>
  );
};

export default AiAssistant;