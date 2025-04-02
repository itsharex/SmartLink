import React, { useState } from 'react';
import Avatar from '../ui/Avatar';
import { type Message } from '@/components/chat/ChatWindow';
import { Check, CheckCheck, Image, File, Mic } from 'lucide-react';
import ReactMarkdown from 'react-markdown';

type MessageBubbleProps = {
  message: Message;
  showAvatar?: boolean;
};

const MessageBubble: React.FC<MessageBubbleProps> = ({
  message,
  showAvatar = true,
}) => {
  const isIncoming = message.sender === 'other';
  const [translatedContent, setTranslatedContent] = useState<string | null>(null);
  const [isTranslating, setIsTranslating] = useState(false);

  const formatTime = (date: Date) => {
    return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  };

  const StatusIcon = () => {
    if (message.status === 'sent') return <Check size={14} />;
    if (message.status === 'delivered') return <CheckCheck size={14} />;
    if (message.status === 'read') return <CheckCheck size={14} className="text-accent-primary" />;
    return null;
  };

  const handleTranslate = async () => {
    if (translatedContent) {
      setTranslatedContent(null); // Toggle back to original
      return;
    }

    setIsTranslating(true);
    try {
      const response = await fetch('/api/groq/translate', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          text: message.content,
          fromLang: 'auto', // Auto-detect source language
          toLang: 'en',     // Translate to English (adjust as needed)
        }),
      });

      if (!response.ok) {
        throw new Error('Translation request failed');
      }

      const data = await response.json();
      setTranslatedContent(data.translated || message.content); // Fallback to original if no translation
    } catch (error) {
      console.error('Translation error:', error);
      setTranslatedContent(message.content); // Fallback to original on error
    } finally {
      setIsTranslating(false);
    }
  };

  const renderMessageContent = () => {
    const content = translatedContent || message.content;
    switch (message.type) {
      case 'image':
        return (
          <div className="relative rounded-lg overflow-hidden">
            <img src={message.media} alt="Shared image" className="max-w-xs rounded-lg" />
          </div>
        );
      case 'file':
        return (
          <div className="flex items-center gap-3 bg-white/5 rounded-lg p-3">
            <div className="w-10 h-10 flex items-center justify-center rounded-full bg-accent-primary/20 text-accent-primary">
              <File size={20} />
            </div>
            <div className="flex-1 min-w-0">
              <p className="text-text-primary font-medium truncate">{content}</p>
              <p className="text-sm text-text-secondary">文件</p>
            </div>
          </div>
        );
      case 'voice':
        return (
          <div className="flex items-center gap-3 bg-white/5 rounded-lg p-3">
            <div className="w-10 h-10 flex items-center justify-center rounded-full bg-accent-primary/20 text-accent-primary">
              <Mic size={20} />
            </div>
            <div className="flex-1 min-w-0">
              <div className="w-36 h-8 bg-white/10 rounded-full overflow-hidden">
                <div className="h-full flex items-center justify-around px-2">
                  {Array.from({ length: 20 }).map((_, i) => (
                    <div
                      key={i}
                      className="w-1 bg-accent-primary"
                      style={{
                        height: `${Math.random() * 100}%`,
                        opacity: isIncoming ? 0.7 : 1,
                      }}
                    />
                  ))}
                </div>
              </div>
              <p className="text-sm text-text-secondary mt-1">0:12</p>
            </div>
          </div>
        );
      default:
        return (
          <div className={isIncoming ? 'text-text-primary' : 'text-white'}>
            <ReactMarkdown>{content}</ReactMarkdown>
          </div>
        );
    }
  };

  return (
    <div className={`max-w-[80%] flex ${isIncoming ? 'self-start' : 'self-end flex-row-reverse'}`}>
      {showAvatar && (
        <div className={`${isIncoming ? 'mr-3' : 'ml-3'}`}>
          <Avatar
            size="sm"
            text={isIncoming ? 'Other' : 'Me'}
            status={isIncoming ? 'online' : undefined}
          />
        </div>
      )}
      <div className="flex flex-col">
        <div
          className={`
            p-3 rounded-2xl relative
            ${isIncoming ? 'bg-bg-tertiary' : 'bg-accent-primary'}
          `}
        >
          {renderMessageContent()}
          {message.type === 'text' && (
            <button
              onClick={handleTranslate}
              disabled={isTranslating}
              className="mt-2 text-sm text-gray-400 hover:text-gray-200"
            >
              {isTranslating ? 'Translating...' : translatedContent ? 'Show Original' : 'Translate to English'}
            </button>
          )}
        </div>
        <div className={`flex items-center mt-1 text-xs text-text-primary-30 ${isIncoming ? 'self-start' : 'self-end'}`}>
          <span>{formatTime(message.timestamp)}</span>
          {!isIncoming && message.status && (
            <span className="ml-1">
              <StatusIcon />
            </span>
          )}
        </div>
      </div>
    </div>
  );
};

export default MessageBubble;