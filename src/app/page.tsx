'use client';

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function HomePage() {
  const [isLogin, setIsLogin] = useState(true);
  const router = useRouter();
  
  const handleAuth = (e: React.FormEvent) => {
    e.preventDefault();
    router.push('/chat');
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-[#0A1128] p-4">
      {/* 页面内容 */}
      <div className="relative w-full max-w-md">
        {/* Logo */}
        <div className="text-center mb-8">
          <div className="relative inline-block">
            <svg width="64" height="64" viewBox="0 0 24 24" className="mx-auto">
              <path d="M12 2L2 7L12 12L22 7L12 2Z" fill="#00E5FF"/>
              <path d="M2 17L12 22L22 17" stroke="#7B68EE" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
              <path d="M2 12L12 17L22 12" stroke="#00E5FF" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
            </svg>
            <div className="absolute inset-0 -m-3 bg-[#00E5FF]/20 rounded-full blur-xl"></div>
          </div>
          
          <h1 className="mt-4 text-4xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-[#00E5FF] via-[#7B68EE] to-[#FF3864]">
            SmartLink
          </h1>
          <p className="mt-2 text-[#9E9E9E]">连接人与AI的桥梁，让你的社交与工作体验超越想象</p>
        </div>
        
        {/* 表单卡片 */}
        <div className="bg-[#162955]/50 backdrop-blur-lg border border-white/10 rounded-xl shadow-2xl overflow-hidden">
          <div className="p-8">
            <h2 className="text-2xl font-semibold text-center text-[#E0E0E0] mb-6">
              {isLogin ? '欢迎回来' : '创建账号'}
            </h2>
            
            <form onSubmit={handleAuth} className="space-y-6">
              {!isLogin && (
                <div className="space-y-2">
                  <label className="block text-sm text-[#9E9E9E]">用户名</label>
                  <input
                    type="text"
                    className="w-full bg-white/5 border border-white/10 rounded-lg px-4 py-3 text-[#E0E0E0] focus:outline-none focus:ring-2 focus:ring-[#00E5FF]/50"
                    placeholder="您的用户名"
                  />
                </div>
              )}
              
              <div className="space-y-2">
                <label className="block text-sm text-[#9E9E9E]">邮箱</label>
                <input
                  type="email"
                  className="w-full bg-white/5 border border-white/10 rounded-lg px-4 py-3 text-[#E0E0E0] focus:outline-none focus:ring-2 focus:ring-[#00E5FF]/50"
                  placeholder="your@email.com"
                  defaultValue={isLogin ? "u7541840@gmail.com" : ""}
                />
              </div>
              
              <div className="space-y-2">
                <div className="flex justify-between">
                  <label className="block text-sm text-[#9E9E9E]">密码</label>
                  {isLogin && (
                    <a href="#" className="text-xs text-[#00E5FF] hover:underline">忘记密码?</a>
                  )}
                </div>
                <input
                  type="password"
                  className="w-full bg-white/5 border border-white/10 rounded-lg px-4 py-3 text-[#E0E0E0] focus:outline-none focus:ring-2 focus:ring-[#00E5FF]/50"
                  placeholder="••••••••"
                />
              </div>
              
              {!isLogin && (
                <div className="space-y-2">
                  <label className="block text-sm text-[#9E9E9E]">确认密码</label>
                  <input
                    type="password"
                    className="w-full bg-white/5 border border-white/10 rounded-lg px-4 py-3 text-[#E0E0E0] focus:outline-none focus:ring-2 focus:ring-[#00E5FF]/50"
                    placeholder="••••••••"
                  />
                </div>
              )}
              
              <div className="flex items-center">
                <input
                  id="remember"
                  type="checkbox"
                  className="w-4 h-4 border-white/10 bg-white/5 rounded accent-[#00E5FF]"
                />
                <label htmlFor="remember" className="ml-2 text-sm text-[#9E9E9E]">
                  {isLogin ? "记住我" : "我同意服务条款和隐私政策"}
                </label>
              </div>
              
              <button
                type="submit"
                className="w-full py-3 px-4 rounded-lg bg-gradient-to-r from-[#00E5FF] to-[#7B68EE] text-[#0A1128] font-medium transition transform hover:-translate-y-0.5"
              >
                {isLogin ? "登录" : "注册"}
              </button>
            </form>
            
            <div className="relative my-6">
              <div className="absolute inset-0 flex items-center">
                <div className="w-full border-t border-white/10"></div>
              </div>
              <div className="relative flex justify-center">
                <span className="px-2 bg-[#162955] text-[#9E9E9E] text-sm">或</span>
              </div>
            </div>
            
            <div className="grid grid-cols-2 gap-4">
              <button
                type="button"
                className="py-2.5 px-4 border border-white/10 rounded-lg flex items-center justify-center gap-2 hover:bg-white/5"
              >
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="20" height="20" fill="#4285F4">
                  <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm5.36 14.8c-1.43 1.33-3.32 2-5.36 2-1.75 0-3.38-.58-4.67-1.54l-1.83-1.41L8.62 12l2.42-2.42L10 8.46l-1.23.95C9.53 8.35 10.73 7.68 12 7.68c3.25 0 5.9 2.66 5.9 5.92 0 1.15-.33 2.23-.91 3.13l1.42 1.42c.39-.51.72-1.07.98-1.66l1.92 1.76c-.41.5-.86.94-1.36 1.33c-.95.74-2.02 1.28-3.19 1.54l1.5 1.5c1.45-.49 2.73-1.32 3.74-2.39c1.79-1.91 2.74-4.36 2.74-6.98 0-5.52-4.48-10-10-10C6.48 2 2 6.48 2 12c0 2.49.9 4.77 2.4 6.53l1.33-1.53C4.6 15.55 4 13.85 4 12c0-4.41 3.59-8 8-8s8 3.59 8 8c0 1.86-.64 3.57-1.71 4.93l-1.27-1.07z"/>
                </svg>
                <span>Google</span>
              </button>
              <button
                type="button"
                className="py-2.5 px-4 border border-white/10 rounded-lg flex items-center justify-center gap-2 hover:bg-white/5"
              >
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="20" height="20" fill="currentColor">
                  <path d="M12 2C6.477 2 2 6.477 2 12c0 4.42 2.865 8.166 6.839 9.489.5.092.682-.217.682-.482 0-.237-.008-.866-.013-1.7-2.782.603-3.369-1.34-3.369-1.34-.454-1.157-1.11-1.465-1.11-1.465-.908-.62.069-.608.069-.608 1.003.07 1.531 1.03 1.531 1.03.892 1.529 2.341 1.087 2.91.831.092-.647.35-1.086.634-1.336-2.22-.253-4.555-1.11-4.555-4.943 0-1.091.39-1.984 1.029-2.683-.103-.253-.446-1.27.099-2.646 0 0 .84-.269 2.75 1.026A9.564 9.564 0 0112 6.839c.85.004 1.705.114 2.504.337 1.909-1.295 2.747-1.026 2.747-1.026.546 1.376.202 2.394.1 2.646.64.699 1.028 1.592 1.028 2.683 0 3.842-2.339 4.687-4.566 4.935.359.309.678.919.678 1.852 0 1.336-.012 2.415-.012 2.743 0 .267.18.578.688.48C19.137 20.163 22 16.418 22 12c0-5.523-4.477-10-10-10z"/>
                </svg>
                <span>GitHub</span>
              </button>
            </div>
            
            <p className="text-center mt-8 text-[#9E9E9E]">
              {isLogin ? '还没有账号?' : '已有账号?'}{' '}
              <button 
                onClick={() => setIsLogin(!isLogin)}
                className="text-[#00E5FF] hover:underline"
              >
                {isLogin ? '立即注册' : '立即登录'}
              </button>
            </p>
          </div>
        </div>
        
        {/* 底部加密标识 */}
        <div className="mt-8 text-center">
          <div className="inline-flex items-center gap-2 text-[#9E9E9E] text-sm">
            <svg className="w-4 h-4 text-[#00E5FF]" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M12 14V17M6 21H18C19.1046 21 20 20.1046 20 19V13C20 11.8954 19.1046 11 18 11H6C4.89543 11 4 11.8954 4 13V19C4 20.1046 4.89543 21 6 21ZM16 11V7C16 4.79086 14.2091 3 12 3C9.79086 3 8 4.79086 8 7V11H16Z" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
            </svg>
            <span>端到端加密，安全无忧</span>
          </div>
        </div>
      </div>
      
      {/* 背景装饰 */}
      <div className="fixed inset-0 -z-10 pointer-events-none">
        <div className="absolute top-1/4 left-1/4 w-1/2 h-1/2 rounded-full bg-[#00E5FF] opacity-5 blur-3xl"></div>
        <div className="absolute bottom-1/4 right-1/4 w-1/2 h-1/2 rounded-full bg-[#7B68EE] opacity-5 blur-3xl"></div>
      </div>
    </div>
  );
}