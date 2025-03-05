'use client';

import React, { useState } from 'react';
import SideNav from '@/components/layout/SideNav';
import MomentsFeed from '@/components/moments/MomentsFeed';
import GlassCard from '@/components/ui/GlassCard';
import Button from '@/components/ui/Button';
import { Image, FileText, MapPin, Smile, Send } from 'lucide-react';

export default function MomentsPage() {
  const [newPost, setNewPost] = useState('');
  const [selectedImages, setSelectedImages] = useState<string[]>([]);
  const [isSubmitting, setIsSubmitting] = useState(false);

  // 处理发布新动态
  const handleSubmitPost = () => {
    if (!newPost.trim() && selectedImages.length === 0) return;
    
    setIsSubmitting(true);
    
    // 模拟发布过程
    setTimeout(() => {
      console.log('发布新动态:', { content: newPost, images: selectedImages });
      setNewPost('');
      setSelectedImages([]);
      setIsSubmitting(false);
    }, 1500);
  };
  
  // 模拟选择图片
  const handleSelectImage = () => {
    // 在实际应用中，这会调用文件选择对话框
    // 这里我们模拟添加一个占位图片
    setSelectedImages(prev => [...prev, `/api/placeholder/300/200?random=${Date.now()}`]);
  };

  return (
    <div className="flex h-screen overflow-hidden">
      {/* 侧边导航 */}
      <SideNav />
      
      <div className="flex-1 bg-bg-primary overflow-auto">
        <div className="max-w-6xl mx-auto p-6">
          {/* 发布新动态 */}
          <GlassCard className="mb-8 p-6">
            <h2 className="text-xl font-semibold mb-4">发布动态</h2>
            <textarea
              className="w-full bg-white/5 border border-white/10 rounded-lg p-4 text-text-primary resize-none focus:outline-none focus:ring-2 focus:ring-accent-primary/50 transition"
              placeholder="分享你的想法..."
              rows={3}
              value={newPost}
              onChange={(e) => setNewPost(e.target.value)}
            />
            
            {/* 已选图片预览 */}
            {selectedImages.length > 0 && (
              <div className="grid grid-cols-3 gap-3 mt-4">
                {selectedImages.map((img, index) => (
                  <div key={index} className="relative aspect-video rounded-lg overflow-hidden group">
                    <img src={img} alt={`Selected image ${index}`} className="w-full h-full object-cover" />
                    <button 
                      className="absolute top-2 right-2 w-6 h-6 rounded-full bg-bg-primary/80 text-white flex items-center justify-center opacity-0 group-hover:opacity-100 transition"
                      onClick={() => setSelectedImages(prev => prev.filter((_, i) => i !== index))}
                    >
                      ×
                    </button>
                  </div>
                ))}
              </div>
            )}
            
            <div className="flex justify-between items-center mt-4">
              <div className="flex gap-3">
                <button 
                  className="w-10 h-10 rounded-full flex items-center justify-center bg-white/5 text-text-secondary hover:bg-accent-primary/10 hover:text-accent-primary transition"
                  onClick={handleSelectImage}
                >
                  <Image size={20} />
                </button>
                <button className="w-10 h-10 rounded-full flex items-center justify-center bg-white/5 text-text-secondary hover:bg-accent-primary/10 hover:text-accent-primary transition">
                  <FileText size={20} />
                </button>
                <button className="w-10 h-10 rounded-full flex items-center justify-center bg-white/5 text-text-secondary hover:bg-accent-primary/10 hover:text-accent-primary transition">
                  <MapPin size={20} />
                </button>
                <button className="w-10 h-10 rounded-full flex items-center justify-center bg-white/5 text-text-secondary hover:bg-accent-primary/10 hover:text-accent-primary transition">
                  <Smile size={20} />
                </button>
              </div>
              
              <Button 
                variant="primary" 
                onClick={handleSubmitPost}
                disabled={(!newPost.trim() && selectedImages.length === 0) || isSubmitting}
              >
                {isSubmitting ? '发布中...' : '发布'}
                <Send size={16} className="ml-2" />
              </Button>
            </div>
          </GlassCard>
          
          {/* 动态内容 */}
          <MomentsFeed />
        </div>
      </div>
    </div>
  );
}