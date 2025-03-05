"use client";

import React, { useState } from 'react';
import Link from 'next/link';
import Button from '../ui/Button';
import { Menu, X } from 'lucide-react';

type NavItem = {
  name: string;
  href: string;
};

const navItems: NavItem[] = [
  { name: '功能', href: '#features' },
  { name: '应用', href: '#app' },
  { name: '动态', href: '#moments' },
  { name: 'AI 助手', href: '#ai' },
];

const Header: React.FC = () => {
  const [isMenuOpen, setIsMenuOpen] = useState(false);

  return (
    <header className="py-6 relative z-50">
      <div className="container mx-auto px-6">
        <div className="flex justify-between items-center">
          {/* Logo */}
          <Link 
            href="/" 
            className="flex items-center text-2xl text-text-primary font-orbitron font-bold"
          >
            <div className="relative mr-3">
              <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M12 2L2 7L12 12L22 7L12 2Z" fill="var(--accent-primary)"/>
                <path d="M2 17L12 22L22 17" stroke="var(--accent-secondary)" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
                <path d="M2 12L12 17L22 12" stroke="var(--accent-primary)" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
              </svg>
              {/* Logo glow effect */}
              <div className="absolute inset-0 -m-1 bg-gradient-to-r from-accent-primary to-accent-secondary rounded-full blur-md opacity-70 -z-10" />
            </div>
            SmartLink
          </Link>
          
          {/* Desktop Navigation */}
          <nav className="hidden md:flex space-x-8">
            {navItems.map((item) => (
              <Link 
                key={item.name}
                href={item.href}
                className="relative text-text-primary font-medium text-lg py-2 hover:text-accent-primary transition-colors duration-300"
              >
                {item.name}
                <span className="absolute left-0 bottom-0 w-0 h-0.5 bg-gradient-to-r from-accent-primary to-accent-secondary group-hover:w-full transition-all duration-300" />
              </Link>
            ))}
          </nav>
          
          {/* Auth Buttons */}
          <div className="hidden md:flex space-x-4">
            <Button variant="outline">登录</Button>
            <Button variant="primary">注册</Button>
          </div>
          
          {/* Mobile Menu Button */}
          <button 
            className="md:hidden text-text-primary"
            onClick={() => setIsMenuOpen(!isMenuOpen)}
          >
            {isMenuOpen ? <X size={24} /> : <Menu size={24} />}
          </button>
        </div>
      </div>
      
      {/* Mobile Menu */}
      {isMenuOpen && (
        <div className="absolute top-full left-0 right-0 bg-bg-secondary/95 backdrop-blur-lg py-4 md:hidden">
          <div className="container mx-auto px-6">
            <nav className="flex flex-col space-y-4">
              {navItems.map((item) => (
                <Link 
                  key={item.name}
                  href={item.href}
                  className="text-text-primary py-2 hover:text-accent-primary"
                  onClick={() => setIsMenuOpen(false)}
                >
                  {item.name}
                </Link>
              ))}
              <div className="flex flex-col space-y-3 pt-4 border-t border-white/10">
                <Button variant="outline" className="w-full">登录</Button>
                <Button variant="primary" className="w-full">注册</Button>
              </div>
            </nav>
          </div>
        </div>
      )}
    </header>
  );
};

export default Header;