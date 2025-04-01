'use client';

import React from 'react';
import GlassCard from '@/components/ui/GlassCard';
import { Search, UserPlus, Star, Users, BookUser } from 'lucide-react';

interface ContactsSidebarProps {
  activeTab: 'all' | 'favorites' | 'groups' | 'friendRequests';
  setActiveTab: (tab: 'all' | 'favorites' | 'groups' | 'friendRequests') => void;
  searchTerm: string;
  setSearchTerm: (term: string) => void;
  friendRequestCount?: number;
}

const ContactsSidebar: React.FC<ContactsSidebarProps> = ({
  activeTab,
  setActiveTab,
  searchTerm,
  setSearchTerm,
  friendRequestCount = 0,
}) => {
  return (
    <div className="space-y-6">
      {/* Search Box */}
      <div className="flex items-center gap-2 bg-bg-tertiary-80 rounded-lg p-3">
        <Search size={18} className="text-text-primary-30" />
        <input 
          type="text" 
          placeholder="Search contacts" 
          className="bg-transparent w-full outline-none border-none text-text-primary placeholder:text-text-primary-30"
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
        />
      </div>
      
      {/* Navigation Tabs */}
      <GlassCard className="overflow-hidden">
        <button 
          className={`flex items-center gap-3 w-full p-4 text-left ${
            activeTab === 'all' 
              ? 'bg-bg-tertiary-80 text-accent-primary' 
              : 'text-text-primary hover:bg-white/5'
          }`}
          onClick={() => setActiveTab('all')}
        >
          <BookUser size={18} />
          <span>All Contacts</span>
        </button>
        
        <button 
          className={`flex items-center gap-3 w-full p-4 text-left ${
            activeTab === 'favorites'
              ? 'bg-bg-tertiary-80 text-accent-primary'
              : 'text-text-primary hover:bg-white/5'
          }`}
          onClick={() => setActiveTab('favorites')}
        >
          <Star size={18} />
          <span>Favorites</span>
        </button>
        
        <button 
          className={`flex items-center gap-3 w-full p-4 text-left ${
            activeTab === 'groups'
              ? 'bg-bg-tertiary-80 text-accent-primary'
              : 'text-text-primary hover:bg-white/5'
          }`}
          onClick={() => setActiveTab('groups')}
        >
          <Users size={18} />
          <span>Groups</span>
        </button>

        <button 
          className={`flex items-center gap-3 w-full p-4 text-left relative ${
            activeTab === 'friendRequests'
              ? 'bg-bg-tertiary-80 text-accent-primary'
              : 'text-text-primary hover:bg-white/5'
          }`}
          onClick={() => setActiveTab('friendRequests')}
        >
          <UserPlus size={18} />
          <span>Friend Requests</span>
          {friendRequestCount > 0 && (
            <span className="ml-auto bg-gradient-to-r from-accent-primary to-accent-secondary text-bg-primary text-xs font-bold rounded-full h-5 w-5 flex items-center justify-center">
              {friendRequestCount}
            </span>
          )}
        </button>
      </GlassCard>
    </div>
  );
};

export default ContactsSidebar;
