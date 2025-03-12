// context/AuthContext.tsx
'use client';

import React, { createContext, useState, useEffect, useContext } from 'react';
import { useRouter } from 'next/navigation';
import { getCurrentUser, logout as apiLogout } from '@/lib/authApi';

const AuthContext = createContext<any>(undefined);

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [user, setUser] = useState<any>(null);
  const [isLoading, setIsLoading] = useState(true);
  const router = useRouter();

  // 组件挂载时检查用户是否已登录
  useEffect(() => {
    const checkAuthStatus = async () => {
      try {
        // 尝试从存储中获取令牌
        const token = localStorage.getItem('authToken');
        
        if (!token) {
          setIsLoading(false);
          return;
        }
        
        // 令牌存在，尝试获取当前用户
        const userData = await getCurrentUser();
        setUser(userData);
      } catch (err) {
        console.error('Auth check failed:', err);
        // 清除可能无效的令牌
        localStorage.removeItem('authToken');
        localStorage.removeItem('user');
      } finally {
        setIsLoading(false);
      }
    };

    checkAuthStatus();
  }, []);

  // 监听 OAuth 弹出窗口发送的认证事件
  useEffect(() => {
    const handleMessage = (event: MessageEvent) => {
      if (event.data.type === 'oauth-success') {
        setUser(event.data.user);
        router.push('/chat');
      }
    };

    window.addEventListener('message', handleMessage);
    return () => window.removeEventListener('message', handleMessage);
  }, [router]);

  const logout = async () => {
    try {
      await apiLogout();
    } catch (err) {
      console.error('Logout error:', err);
    } finally {
      // 无论API是否成功，都清除本地存储
      localStorage.removeItem('authToken');
      localStorage.removeItem('user');
      setUser(null);
      router.push('/auth');
    }
  };

  const value = {
    user,
    isAuthenticated: !!user,
    isLoading,
    logout,
  };

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};