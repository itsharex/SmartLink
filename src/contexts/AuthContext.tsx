// src/contexts/AuthContext.tsx
"use client"

import React, { createContext, useState, useEffect, useCallback } from 'react';

type User = {
  id: string;
  displayName: string | null;
  email: string | null;
  photoURL: string | null;
};

type AuthContextType = {
  user: User | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  initialize: () => Promise<void>;
  login: (email: string, password: string) => Promise<void>;
  signup: (email: string, password: string, displayName: string) => Promise<void>;
  logout: () => Promise<void>;
};

export const AuthContext = createContext<AuthContextType | null>(null);

export const AuthProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [user, setUser] = useState<User | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  
  const initialize = useCallback(async () => {
    setIsLoading(true);
    try {
      // 在实际实现中，这里应该调用 Tauri 命令来检查当前的认证状态
      // const response = await invoke<User | null>('get_current_user');
      
      // 现在我们模拟这个过程
      const storedUser = localStorage.getItem('smartlink_user');
      if (storedUser) {
        setUser(JSON.parse(storedUser));
      }
    } catch (error) {
      console.error('初始化认证状态失败:', error);
      setUser(null);
    } finally {
      setIsLoading(false);
    }
  }, []);
  
  const login = async (email: string, password: string) => {
    setIsLoading(true);
    try {
      // 在实际实现中，这里应该调用 Tauri 命令进行认证
      // const user = await invoke<User>('login', { email, password });
      
      // 现在我们模拟这个过程
      await new Promise(resolve => setTimeout(resolve, 1000));
      const mockUser = {
        id: '123',
        displayName: email.split('@')[0],
        email,
        photoURL: null
      };
      
      localStorage.setItem('smartlink_user', JSON.stringify(mockUser));
      setUser(mockUser);
    } catch (error) {
      console.error('登录失败:', error);
      throw error;
    } finally {
      setIsLoading(false);
    }
  };
  
  const signup = async (email: string, password: string, displayName: string) => {
    setIsLoading(true);
    try {
      // 在实际实现中，这里应该调用 Tauri 命令进行注册
      // const user = await invoke<User>('signup', { email, password, displayName });
      
      // 现在我们模拟这个过程
      await new Promise(resolve => setTimeout(resolve, 1500));
      const mockUser = {
        id: '123',
        displayName,
        email,
        photoURL: null
      };
      
      localStorage.setItem('smartlink_user', JSON.stringify(mockUser));
      setUser(mockUser);
    } catch (error) {
      console.error('注册失败:', error);
      throw error;
    } finally {
      setIsLoading(false);
    }
  };
  
  const logout = async () => {
    setIsLoading(true);
    try {
      // 在实际实现中，这里应该调用 Tauri 命令进行登出
      // await invoke('logout');
      
      // 现在我们模拟这个过程
      localStorage.removeItem('smartlink_user');
      setUser(null);
    } catch (error) {
      console.error('登出失败:', error);
      throw error;
    } finally {
      setIsLoading(false);
    }
  };
  
  // 初始化认证状态
  useEffect(() => {
    initialize();
  }, [initialize]);
  
  const value = {
    user,
    isAuthenticated: !!user,
    isLoading,
    initialize,
    login,
    signup,
    logout
  };
  
  return (
    <AuthContext.Provider value={value}>
      {children}
    </AuthContext.Provider>
  );
};