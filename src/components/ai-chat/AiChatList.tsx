import React, { useState, useEffect } from 'react';
import { MdAdd } from "react-icons/md";
import { AiChat, AiMessage } from '@/app/ai-chat/page';

type AiChatListProps = {
  chats: AiChat[];
  aiMessages: Record<string, AiMessage[]>;
  selectedChatId?: string;
  onSelectChat: (chatId: string) => void;
  onAddChat: () => void;
  onUpdateChatTitle: (chatId: string, newTitle: string) => void;
};

// Sub-component for an editable chat list item.
const ChatListItem: React.FC<{
  chat: AiChat;
  isSelected: boolean;
  latestMessage: string;
  latestTime: string;
  onSelect: () => void;
  onUpdateTitle: (chatId: string, newTitle: string) => void;
}> = ({ chat, isSelected, latestMessage, latestTime, onSelect, onUpdateTitle }) => {
  const [isEditing, setIsEditing] = useState(false);
  const [title, setTitle] = useState(chat.topic);

  useEffect(() => {
    setTitle(chat.topic);
  }, [chat.topic]);

  const handleDoubleClick = (e: React.MouseEvent) => {
    e.stopPropagation();
    setIsEditing(true);
  };

  const handleBlur = () => {
    setIsEditing(false);
    onUpdateTitle(chat.id, title);
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter") {
      setIsEditing(false);
      onUpdateTitle(chat.id, title);
    }
  };

  return (
    <div
      onClick={onSelect}
      className={`p-4 border-b border-text-primary-5 cursor-pointer ${
        isSelected ? 'bg-accent-primary-10' : 'hover:bg-bg-tertiary'
      }`}
    >
      {isEditing ? (
        <input
          type="text"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          onBlur={handleBlur}
          onKeyDown={handleKeyDown}
          autoFocus
          className="font-semibold text-text-primary bg-transparent outline-none"
        />
      ) : (
        <h3 onDoubleClick={handleDoubleClick} className="font-semibold text-text-primary">
          {chat.topic}
        </h3>
      )}
      <p className="text-sm text-text-primary-30 line-clamp-2">
        {latestMessage || "No AI message yet."}
      </p>
      <span className="text-xs text-text-primary-30">{latestTime}</span>
    </div>
  );
};

const AiChatList: React.FC<AiChatListProps> = ({
  chats,
  aiMessages,
  selectedChatId,
  onSelectChat,
  onAddChat,
  onUpdateChatTitle,
}) => {
  return (
    <div className="w-80 h-full bg-bg-primary flex flex-col border-r-[1.5px] border-text-primary-5">
      <div className="p-5 h-20 border-b border-text-primary-5 flex justify-between items-center shadow-[0_4px_10px_rgba(0,0,0,0.05)]">
        <h2 className="flex items-center text-lg font-semibold text-text-primary">
          AI Chats
        </h2>
        <button
          className="group relative w-8 h-8 rounded-full flex items-center justify-center transition-colors bg-transparent bg-gradient-to-br from-accent-primary-80 to-accent-secondary-80"
          onClick={onAddChat}
        >
          <MdAdd size={26} className='text-white' />
        </button>
      </div>
      <div className="flex-1 overflow-y-auto">
        {chats.map(chat => {
          // Get the messages for this chat and find the latest AI message.
          const messagesForChat = aiMessages[chat.id] || [];
          const lastAiMessage = [...messagesForChat].reverse().find(msg => msg.sender === 'other');
          const latestMessage = lastAiMessage ? lastAiMessage.content : "";
          const latestTime = lastAiMessage
            ? new Date(lastAiMessage.timestamp).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
            : chat.time;
          return (
            <ChatListItem 
              key={chat.id}
              chat={chat}
              isSelected={selectedChatId === chat.id}
              latestMessage={latestMessage}
              latestTime={latestTime}
              onSelect={() => onSelectChat(chat.id)}
              onUpdateTitle={onUpdateChatTitle}
            />
          );
        })}
      </div>
    </div>
  );
};

export default AiChatList;
