'use client';

import React, { useState } from 'react';
import SideNav from '@/components/layout/SideNav';
import ContactsSidebar from '@/components/contacts/ContactsSidebar';
import ContactsList, { Contact } from '@/components/contacts/ContactsList';

// Sample contacts data in English
const contactsData: Contact[] = [
  { id: '1', name: 'John Doe', status: 'online', avatar: '', favorite: true, tags: ['Colleague', 'Project A'] },
  { id: '2', name: 'Jane Smith', status: 'offline', avatar: '', favorite: false, tags: ['Friend'] },
  { id: '3', name: 'Michael Brown', status: 'away', avatar: '', favorite: true, tags: ['Family'] },
  { id: '4', name: 'Emily Johnson', status: 'busy', avatar: '', favorite: false, tags: ['Classmate', 'Project B'] },
  { id: '5', name: 'David Lee', status: 'online', avatar: '', favorite: false, tags: ['Colleague'] },
];

// Sample friend requests data in English
const requestsData = [
  { id: '1', name: 'Robert Green', avatar: '', timeAgo: '2 days ago' },
  { id: '2', name: 'Linda White', avatar: '', timeAgo: '1 week ago' },
];

export default function ContactsPage() {
  const [activeTab, setActiveTab] = useState<'all' | 'favorites' | 'groups' | 'friendRequests'>('all');
  const [searchTerm, setSearchTerm] = useState('');

  const filteredContacts = contactsData.filter(contact => {
    if (searchTerm && !contact.name.toLowerCase().includes(searchTerm.toLowerCase())) {
      return false;
    }
    if (activeTab === 'favorites' && !contact.favorite) {
      return false;
    }
    return true;
  });

  return (
    <div className="flex h-screen overflow-hidden">
      <SideNav userName={''} />
      
      <div className="flex-1 bg-bg-primary p-6 overflow-auto">
        <div className="max-w-6xl mx-auto">
          <header className="mb-8">
            <h1 className="text-3xl font-bold font-orbitron mb-2 bg-gradient-to-r from-accent-primary to-accent-secondary bg-clip-text text-transparent">
              Contacts
            </h1>
            <p className="text-text-primary">Manage your contacts and groups</p>
          </header>
          
          <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
            {/* Sidebar */}
            <div className="md:col-span-1">
              <ContactsSidebar 
                activeTab={activeTab}
                setActiveTab={setActiveTab}
                searchTerm={searchTerm}
                setSearchTerm={setSearchTerm}
                friendRequestCount={requestsData.length}
              />
            </div>
            
            {/* Contacts List */}
            <div className="md:col-span-3">
              {activeTab === 'friendRequests' ? (
                <ContactsList 
                  activeTab={activeTab}
                  friendRequestsData={requestsData}
                  contacts={[]} 
                />
              ) : (
                <ContactsList 
                  activeTab={activeTab}
                  contacts={filteredContacts}
                />
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
