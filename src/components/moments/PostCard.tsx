import React, { useState } from 'react';
import GlassCard from '../ui/GlassCard';
import Avatar from '../ui/Avatar';
import { Heart, MessageSquare, Share, MoreVertical } from 'lucide-react';

type PostTag = {
  id: string;
  name: string;
};

type PostCardProps = {
  id: string;
  username: string;
  userAvatar?: string;
  time: string;
  content: string;
  image?: string;
  likes: number;
  comments: number;
  shares: number;
  tags?: PostTag[];
  hasLiked?: boolean;
  onLike: (postId: string) => void;
  onComment: (postId: string) => void;
  onShare: (postId: string) => void;
};

const PostCard: React.FC<PostCardProps> = ({
  id,
  username,
  userAvatar,
  time,
  content,
  image,
  likes,
  comments,
  shares,
  tags = [],
  hasLiked = false,
  onLike,
  onComment,
  onShare
}) => {
  const [liked, setLiked] = useState(hasLiked);
  const [likeCount, setLikeCount] = useState(likes);
  
  const handleLike = () => {
    setLiked(!liked);
    setLikeCount(prev => liked ? prev - 1 : prev + 1);
    onLike(id);
  };

  return (
    <GlassCard className="overflow-hidden transition duration-300 hover:-translate-y-1 hover:shadow-xl">
      {/* Post Header */}
      <div className="flex items-center justify-between p-4">
        <div className="flex items-center gap-3">
          <Avatar text={username} src={userAvatar} />
          <div>
            <h3 className="font-semibold text-text-primary">{username}</h3>
            <p className="text-sm text-text-secondary">{time}</p>
          </div>
        </div>
        <button className="text-text-secondary hover:text-text-primary transition">
          <MoreVertical size={20} />
        </button>
      </div>
      
      {/* Post Image (if available) */}
      {image && (
        <img 
          src={image} 
          alt="Post content" 
          className="w-full max-h-96 object-cover"
        />
      )}
      
      {/* Post Content */}
      <div className="p-4">
        <p className="text-text-primary leading-relaxed whitespace-pre-line">{content}</p>
        
        {/* Tags */}
        {tags.length > 0 && (
          <div className="flex flex-wrap gap-2 mt-4">
            {tags.map((tag) => (
              <span 
                key={tag.id}
                className="bg-accent-primary/10 text-accent-primary px-3 py-1 rounded-md text-sm"
              >
                #{tag.name}
              </span>
            ))}
          </div>
        )}
      </div>
      
      {/* Post Actions */}
      <div className="flex border-t border-white/5">
        <button 
          className={`
            flex items-center justify-center gap-2 flex-1 py-3 
            transition-colors duration-200
            ${liked 
              ? 'text-accent-tertiary' 
              : 'text-text-secondary hover:text-accent-tertiary'
            }
          `}
          onClick={handleLike}
        >
          <Heart size={18} fill={liked ? 'currentColor' : 'none'} />
          <span>{likeCount}</span>
        </button>
        
        <button 
          className="flex items-center justify-center gap-2 flex-1 py-3 text-text-secondary hover:text-accent-primary transition-colors duration-200"
          onClick={() => onComment(id)}
        >
          <MessageSquare size={18} />
          <span>{comments}</span>
        </button>
        
        <button 
          className="flex items-center justify-center gap-2 flex-1 py-3 text-text-secondary hover:text-accent-primary transition-colors duration-200"
          onClick={() => onShare(id)}
        >
          <Share size={18} />
          <span>{shares}</span>
        </button>
      </div>
    </GlassCard>
  );
};

export default PostCard;