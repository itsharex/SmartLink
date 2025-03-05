import React, { useState } from 'react';
import StoryReel from './StoryReel';
import PostCard from './PostCard';
import GlassCard from '../ui/GlassCard';
import Avatar from '../ui/Avatar';
import { Users, Flame, Search } from 'lucide-react';

// Types for the stories
const sampleStories = [
  { id: '1', username: '王小明', avatar: '/api/placeholder/70/70', hasUnviewed: true },
  { id: '2', username: '张伟', avatar: '/api/placeholder/70/70', hasUnviewed: true },
  { id: '3', username: '刘明', avatar: '/api/placeholder/70/70', hasUnviewed: false },
  { id: '4', username: '李华', avatar: '/api/placeholder/70/70', hasUnviewed: true },
  { id: '5', username: '陈晨', avatar: '/api/placeholder/70/70', hasUnviewed: false },
  { id: '6', username: '黄小红', avatar: '/api/placeholder/70/70', hasUnviewed: true },
];

// Sample posts
const samplePosts = [
  {
    id: '1',
    username: '王小明',
    userAvatar: '/api/placeholder/100/100',
    time: '2小时前',
    content: '今天和团队一起完成了新项目的初步设计，感觉非常有成就感！感谢大家的努力！🚀',
    likes: 24,
    comments: 5,
    shares: 2,
    tags: [
      { id: '1', name: '团队协作' },
      { id: '2', name: '项目进展' },
    ],
  },
  {
    id: '2',
    username: '张伟',
    userAvatar: '/api/placeholder/100/100',
    time: '昨天',
    content: '分享一张我在公园拍的照片，春天真美！',
    image: '/api/placeholder/600/400',
    likes: 56,
    comments: 12,
    shares: 4,
    tags: [
      { id: '3', name: '摄影' },
      { id: '4', name: '春天' },
    ],
  },
];

// Types for the hot topics
type HotTopic = {
  id: string;
  title: string;
  tag: string;
  ranking: number;
};

const hotTopics: HotTopic[] = [
  { id: '1', title: '新一代AI技术发布', tag: '科技', ranking: 1 },
  { id: '2', title: '春季健康小贴士', tag: '健康', ranking: 2 },
  { id: '3', title: '本周最佳电影推荐', tag: '娱乐', ranking: 3 },
  { id: '4', title: '远程工作效率提升方法', tag: '职场', ranking: 4 },
];

const MomentsFeed: React.FC = () => {
  const [viewedStory, setViewedStory] = useState<string | null>(null);
  
  const handleViewStory = (storyId: string) => {
    setViewedStory(storyId);
    // In a real app, you'd show a story viewer here
    console.log(`Viewing story: ${storyId}`);
  };
  
  const handleLike = (postId: string) => {
    console.log(`Liked post: ${postId}`);
  };
  
  const handleComment = (postId: string) => {
    console.log(`Commenting on post: ${postId}`);
  };
  
  const handleShare = (postId: string) => {
    console.log(`Sharing post: ${postId}`);
  };

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">
        {/* Sidebar */}
        <div className="lg:col-span-1 space-y-6">
          {/* User Profile */}
          <GlassCard className="p-6 flex flex-col items-center text-center">
            <div className="mb-4">
              <Avatar size="xl" text="李明" glow />
            </div>
            <h3 className="text-xl font-semibold text-text-primary mb-2">李明</h3>
            
            {/* User Stats */}
            <div className="flex justify-between w-full mt-4">
              <div className="text-center">
                <p className="text-xl font-semibold text-accent-primary">245</p>
                <p className="text-sm text-text-secondary">关注</p>
              </div>
              <div className="text-center">
                <p className="text-xl font-semibold text-accent-primary">186</p>
                <p className="text-sm text-text-secondary">粉丝</p>
              </div>
              <div className="text-center">
                <p className="text-xl font-semibold text-accent-primary">57</p>
                <p className="text-sm text-text-secondary">动态</p>
              </div>
            </div>
          </GlassCard>
          
          {/* Hot Topics */}
          <GlassCard className="p-6">
            <h3 className="text-xl font-semibold flex items-center gap-2 mb-6">
              <Flame size={20} className="text-accent-primary" />
              热门话题
            </h3>
            
            <div className="space-y-4">
              {hotTopics.map((topic) => (
                <div 
                  key={topic.id} 
                  className="flex gap-4 pb-4 border-b border-white/5 last:border-0 last:pb-0"
                >
                  <span className="text-2xl font-bold text-accent-primary opacity-60">
                    {topic.ranking}
                  </span>
                  <div>
                    <p className="text-text-primary font-medium">{topic.title}</p>
                    <p className="text-sm text-accent-secondary mt-1">#{topic.tag}</p>
                  </div>
                </div>
              ))}
            </div>
          </GlassCard>
        </div>
        
        {/* Main Content */}
        <div className="lg:col-span-3 space-y-6">
          {/* Stories */}
          <StoryReel stories={sampleStories} onViewStory={handleViewStory} />
          
          {/* Posts */}
          <div className="space-y-6">
            {samplePosts.map((post) => (
              <PostCard 
                key={post.id}
                id={post.id}
                username={post.username}
                userAvatar={post.userAvatar}
                time={post.time}
                content={post.content}
                image={post.image}
                likes={post.likes}
                comments={post.comments}
                shares={post.shares}
                tags={post.tags}
                onLike={handleLike}
                onComment={handleComment}
                onShare={handleShare}
              />
            ))}
          </div>
        </div>
      </div>
    </div>
  );
};

export default MomentsFeed;