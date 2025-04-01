import React from 'react';

type AiChat = {
  id: string;
  topic: string;
  summary: string;
  time: string;
};

type AiChatListProps = {
  chats: AiChat[];
  selectedChatId?: string;
  onSelectChat: (chatId: string) => void;
};

const AiChatList: React.FC<AiChatListProps> = ({ chats, selectedChatId, onSelectChat }) => {
  return (
    <div className="w-80 h-full bg-bg-primary flex flex-col border-r-[1.5px] border-text-primary-5">
      <div className="p-5 h-20 border-b border-text-primary-5 flex justify-between items-center shadow-[0_4px_10px_rgba(0,0,0,0.05)]">
        <h2 className="flex items-center text-lg font-semibold text-text-primary">
          AI Chats
        </h2>
      </div>
      <div className="flex-1 overflow-y-auto">
        {chats.map(chat => (
          <div
            key={chat.id}
            onClick={() => onSelectChat(chat.id)}
            className={`p-4 border-b border-text-primary-5 cursor-pointer ${selectedChatId === chat.id ? 'bg-accent-primary-10' : 'hover:bg-bg-tertiary'}`}
          >
            <h3 className="font-semibold text-text-primary">{chat.topic}</h3>
            <p className="text-sm text-text-primary-30">{chat.summary}</p>
            <span className="text-xs text-text-primary-30">{chat.time}</span>
          </div>
        ))}
      </div>
    </div>
  );
};

export default AiChatList;
