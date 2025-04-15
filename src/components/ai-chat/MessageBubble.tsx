import React, { useState } from 'react';
import Avatar from '../ui/Avatar';
import { type Message } from '@/components/chat/ChatWindow';
import { Check, CheckCheck, Image, File, Mic, ChevronDown, RotateCcw } from 'lucide-react';
import ReactMarkdown from 'react-markdown';
import { languageOptions } from '@/lib/constants/languages';

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
  const [targetLanguage, setTargetLanguage] = useState('en');
  const [translationError, setTranslationError] = useState<string | null>(null);
  const [showLanguageDropdown, setShowLanguageDropdown] = useState(false);

  const messageType = message.type || 'text';

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
      setTranslatedContent(null);
      setTranslationError(null);
      return;
    }

    setIsTranslating(true);
    setTranslationError(null);
    try {
      const response = await fetch('/api/groq/translate', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          text: message.content,
          fromLang: 'auto',
          toLang: targetLanguage,
        }),
      });

      if (!response.ok) {
        throw new Error('Translation request failed');
      }

      const data = await response.json();
      if (data.translated === message.content) {
        // If the translated text is the same as the original, assume the translation failed
        setTranslationError(`Translation to ${languageOptions.find(lang => lang.code === targetLanguage)?.name} failed.`);
        setTranslatedContent(null);
      } else {
        setTranslatedContent(data.translated);
      }
    } catch (error) {
      console.error('Translation error:', error);
      setTranslationError(`Translation to ${languageOptions.find(lang => lang.code === targetLanguage)?.name} failed.`);
      setTranslatedContent(null);
    } finally {
      setIsTranslating(false);
    }
  };

  const renderMessageContent = () => {
    const content = translatedContent || message.content;
    switch (messageType) {
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
            {translationError && (
              <p className="text-sm text-red-400 mt-1">{translationError}</p>
            )}
          </div>
        );
    }
  };

  const selectedLanguageName = languageOptions.find(lang => lang.code === targetLanguage)?.name || 'English';

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
          {messageType === 'text' && isIncoming && (
            <div className="mt-2 flex items-center gap-2 border-t border-white/10 pt-2">
              <div className="relative">
                <button
                  onClick={() => setShowLanguageDropdown(!showLanguageDropdown)}
                  className="flex items-center gap-1 px-2 py-1 text-sm bg-bg-secondary text-text-primary rounded-md transition-colors"
                  disabled={isTranslating}
                >
                  <span>{selectedLanguageName}</span>
                  <ChevronDown size={14} className={`transition-transform ${showLanguageDropdown ? 'rotate-180' : ''}`} />
                </button>
                {showLanguageDropdown && (
                  <div className="absolute z-10 mt-1 w-40 max-h-48 overflow-y-auto rounded-md bg-bg-tertiary shadow-lg border border-white/10">
                    <div className="py-1">
                      {languageOptions.map((lang) => (
                        <button
                          key={lang.code}
                          onClick={() => {
                            setTargetLanguage(lang.code);
                            setShowLanguageDropdown(false);
                            setTranslatedContent(null);
                          }}
                          className={`block w-full text-left px-3 py-2 text-sm ${
                            targetLanguage === lang.code 
                              ? 'bg-bg-secondary text-accent-primary' 
                              : 'text-text-primary hover:bg-accent-primary-10'
                          }`}
                        >
                          {lang.name}
                        </button>
                      ))}
                    </div>
                  </div>
                )}
              </div>
              <button
                onClick={handleTranslate}
                disabled={isTranslating}
                className={`flex items-center gap-1 px-3 py-1 text-sm rounded-md transition-colors bg-accent-primary hover:bg-accent-primary-80 text-text-secondary`}
              >
                {isTranslating ? (
                  <>
                    <div className="w-3 h-3 rounded-full border-2 border-white border-t-transparent animate-spin mr-1"></div>
                    <span>Translating...</span>
                  </>
                ) : translatedContent ? (
                  <>
                    <RotateCcw size={14} />
                    <span>Show original text</span>
                  </>
                ) : (
                  <span>Translate</span>
                )}
              </button>
            </div>
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