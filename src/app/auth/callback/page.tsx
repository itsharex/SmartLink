// app/auth/callback/page.tsx
'use client';

import { useEffect, useState } from 'react';
import { useRouter, useSearchParams } from 'next/navigation';
import { handleOAuthCallback } from '@/lib/authApi';

export default function OAuthCallbackPage() {
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const router = useRouter();
  const searchParams = useSearchParams();

  useEffect(() => {
    const processOAuthCallback = async () => {
      try {
        // 获取URL参数
        const code = searchParams.get('code');
        const state = searchParams.get('state');
        const provider = searchParams.get('provider') || '';
        
        if (!code) {
          throw new Error('No authorization code received');
        }
        
        // 处理OAuth回调
        const response = await handleOAuthCallback(provider, code, state || undefined);
        
        // 存储认证数据
        localStorage.setItem('authToken', response.token);
        localStorage.setItem('user', JSON.stringify(response.user));
        
        // 如果是弹出窗口，关闭它并发送消息给主窗口
        if (window.opener) {
          window.opener.postMessage({ type: 'oauth-success', user: response.user }, '*');
          window.close();
        } else {
          // 导航到聊天页面
          router.push('/chat');
        }
      } catch (err: any) {
        console.error('OAuth callback error:', err);
        setError(err.message || 'Authentication failed');
      } finally {
        setIsLoading(false);
      }
    };

    processOAuthCallback();
  }, [router, searchParams]);

  // 渲染加载或错误状态
  // ...
}