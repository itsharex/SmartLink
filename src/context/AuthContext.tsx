'use client';

import React, { createContext, useState, useEffect, useContext } from 'react';
import { useRouter, usePathname, useSearchParams } from 'next/navigation';
import { getCurrentUser, logout as apiLogout, authUser } from '@/lib/authApi';

interface AuthContextType {
  user: authUser | null;
  setUser: (user: authUser | null) => void;
  isAuthenticated: boolean;
  isLoading: boolean;
  logout: () => Promise<void>;
  refresh: () => Promise<void>;
}

export const AuthContext = createContext<AuthContextType | undefined>(undefined);

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [user, setUser] = useState<authUser | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const router = useRouter();
  const pathname = usePathname();
  const searchParams = useSearchParams();

  const checkAuthStatus = async () => {
    setIsLoading(true);
    try {
      const userData = await getCurrentUser();
      
      if (userData) {
        // 用户已登录
        setUser(userData);
        if (pathname === '/auth') {
          router.push('/chat');
        }
      } else {
        // 用户未登录
        setUser(null);
        localStorage.removeItem('authToken');
        if (!pathname.startsWith('/auth')) {
          router.push('/auth');
        }
      }
    } catch (error) {
      setUser(null);
      if (!pathname.startsWith('/auth')) {
        router.push('/auth');
      }
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    checkAuthStatus();
  }, []);

  useEffect(() => {
    const handleMessage = (event: MessageEvent) => {
      if (event.origin !== window.location.origin) return;
      if (event.data.type === 'oauth-success') {
        setUser(event.data.user);
        localStorage.setItem('authToken', event.data.token);
        router.push('/chat');
      }
    };

    window.addEventListener('message', handleMessage);
    return () => window.removeEventListener('message', handleMessage);
  }, [router]);

  useEffect(() => {
    const handleRouteChange = (url: string) => {
      if (!user && !url.startsWith('/auth')) {
        router.push('/auth');
      }
    };

    // Use pathname and searchParams to construct the full URL
    const fullUrl = `${pathname}${searchParams ? `?${searchParams.toString()}` : ''}`;
    handleRouteChange(fullUrl);

    // No need to listen to router.events in Next.js 13+ with App Router
  }, [user, pathname, searchParams, router]);

  const logout = async () => {
    try {
      await apiLogout();
    } catch (err) {
      console.error('Logout error:', err);
    } finally {
      localStorage.removeItem('user');
      localStorage.removeItem('authToken');
      sessionStorage.clear();
      setUser(null);
      router.push('/auth');
      window.location.reload();
    }
  };

  const value = {
    user,
    setUser,
    isAuthenticated: !!user,
    isLoading,
    logout,
    refresh: checkAuthStatus,
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