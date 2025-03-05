import React, { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';

const SplashScreen: React.FC = () => {
  const router = useRouter();
  const [progress, setProgress] = useState(0);
  
  useEffect(() => {
    // 模拟加载过程
    const timer = setInterval(() => {
      setProgress(prev => {
        if (prev >= 100) {
          clearInterval(timer);
          return 100;
        }
        return prev + 5;
      });
    }, 100);
    
    // 加载完成后跳转
    const redirectTimer = setTimeout(() => {
      router.push('/login');
    }, 2500);
    
    return () => {
      clearInterval(timer);
      clearTimeout(redirectTimer);
    };
  }, [router]);

  return (
    <div className="fixed inset-0 flex flex-col items-center justify-center bg-bg-primary z-50">
      {/* 背景光效 */}
      <div className="absolute inset-0 pointer-events-none">
        <div className="absolute top-1/4 left-1/4 w-2/3 h-2/3 rounded-full bg-accent-primary opacity-5 blur-3xl"></div>
        <div className="absolute bottom-1/4 right-1/4 w-2/3 h-2/3 rounded-full bg-accent-secondary opacity-5 blur-3xl"></div>
      </div>
      
      {/* Logo */}
      <div className="relative mb-10">
        <svg width="80" height="80" viewBox="0 0 24 24" className="text-text-primary">
          <path d="M12 2L2 7L12 12L22 7L12 2Z" fill="var(--accent-primary)"/>
          <path d="M2 17L12 22L22 17" stroke="var(--accent-secondary)" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
          <path d="M2 12L12 17L22 12" stroke="var(--accent-primary)" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
        </svg>
        
        {/* Pulsing glow effect */}
        <div className="absolute inset-0 -m-2 rounded-full bg-accent-primary opacity-20 blur-xl animate-pulse"></div>
        
        {/* Rotating particles */}
        <div className="absolute inset-0 -m-12 w-[calc(100%+6rem)] h-[calc(100%+6rem)]">
          {[...Array(8)].map((_, i) => (
            <div 
              key={i}
              className="absolute w-2 h-2 rounded-full bg-accent-primary"
              style={{
                top: '50%',
                left: '50%',
                transformOrigin: '0 0',
                transform: `rotate(${i * 45}deg) translateX(5rem) translateY(-50%)`,
                opacity: 0.5,
                animation: `orbit ${4 + i * 0.5}s linear infinite`
              }}
            ></div>
          ))}
        </div>
      </div>
      
      <h1 className="text-3xl font-bold font-orbitron text-text-primary mb-8">SmartLink</h1>
      
      {/* Progress bar */}
      <div className="w-64 h-1 bg-white/10 rounded-full overflow-hidden relative">
        <div 
          className="absolute top-0 left-0 h-full bg-gradient-to-r from-accent-primary to-accent-secondary"
          style={{ width: `${progress}%` }}
        ></div>
      </div>
      
      <p className="text-text-secondary mt-4 text-sm">启动中...</p>
      
      {/* CSS for rotating particles */}
      <style jsx global>{`
        @keyframes orbit {
          from { transform: rotate(0deg) translateX(5rem) translateY(-50%) rotate(0deg); }
          to { transform: rotate(360deg) translateX(5rem) translateY(-50%) rotate(-360deg); }
        }
      `}</style>
    </div>
  );
};

export default SplashScreen;