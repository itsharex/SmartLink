import React from 'react';
import GlassCard from '../ui/GlassCard';
import { ChevronRight } from 'lucide-react';

type Story = {
  id: string;
  username: string;
  avatar: string;
  hasUnviewed: boolean;
};

type StoryReelProps = {
  stories: Story[];
  onViewStory: (storyId: string) => void;
};

const StoryReel: React.FC<StoryReelProps> = ({ stories, onViewStory }) => {
  return (
    <GlassCard className="p-6">
      <div className="flex justify-between items-center mb-6">
        <h3 className="text-xl font-semibold text-text-primary">今日故事</h3>
        <button className="w-8 h-8 flex items-center justify-center rounded-full bg-white/5 text-text-primary hover:bg-accent-primary/10 hover:text-accent-primary transition">
          <ChevronRight size={18} />
        </button>
      </div>
      
      <div className="grid grid-cols-6 gap-4">
        {stories.map((story) => (
          <div 
            key={story.id}
            className="flex flex-col items-center gap-2"
            onClick={() => onViewStory(story.id)}
          >
            <div className="relative cursor-pointer">
              {/* Story ring - gradient if unviewed, gray if viewed */}
              <div 
                className={`
                  absolute -inset-1 rounded-full 
                  ${story.hasUnviewed 
                    ? 'bg-gradient-to-br from-accent-primary via-accent-secondary to-accent-tertiary' 
                    : 'bg-gray-700'
                  }
                `}
              />
              
              {/* Avatar */}
              <div className="relative w-16 h-16 rounded-full border-2 border-bg-primary overflow-hidden">
                <img 
                  src={story.avatar || `/api/placeholder/70/70`} 
                  alt={story.username} 
                  className="w-full h-full object-cover"
                />
              </div>
            </div>
            
            <span className="text-sm text-text-primary text-center truncate max-w-[80px]">
              {story.username}
            </span>
          </div>
        ))}
      </div>
    </GlassCard>
  );
};

export default StoryReel;