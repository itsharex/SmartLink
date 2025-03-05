import React from 'react';
import { Minus, X, Square, Minimize2 } from 'lucide-react';

type AppWindowFrameProps = {
  children: React.ReactNode;
  title?: string;
};

const AppWindowFrame: React.FC<AppWindowFrameProps> = ({ 
  children, 
  title = 'SmartLink'
}) => {
  // 这些函数需要与 Tauri 的窗口管理 API 集成
  const handleMinimize = () => {
    console.log('Minimize window');
    // 在实际集成时替换为 Tauri API 调用
    // window.__TAURI__.window.getCurrent().minimize();
  };
  
  const handleMaximize = () => {
    console.log('Maximize window');
    // 在实际集成时替换为 Tauri API 调用
    // window.__TAURI__.window.getCurrent().maximize();
  };
  
  const handleClose = () => {
    console.log('Close window');
    // 在实际集成时替换为 Tauri API 调用
    // window.__TAURI__.window.getCurrent().close();
  };

  return (
    <div className="flex flex-col h-screen overflow-hidden bg-bg-primary">
      {/* 应用标题栏 - 使用 data-tauri-drag-region 使其可拖动 */}
      <div 
        className="h-10 bg-bg-secondary border-b border-white/5 flex items-center justify-between px-4 select-none"
        data-tauri-drag-region
      >
        <div className="flex items-center gap-2" data-tauri-drag-region>
          <svg width="16" height="16" viewBox="0 0 24 24" className="text-accent-primary">
            <path d="M12 2L2 7L12 12L22 7L12 2Z" fill="currentColor"/>
            <path d="M2 17L12 22L22 17" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
            <path d="M2 12L12 17L22 12" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
          </svg>
          <span className="text-text-primary font-medium" data-tauri-drag-region>{title}</span>
        </div>
        
        <div className="flex">
          <button 
            className="w-10 h-8 flex items-center justify-center text-text-secondary hover:text-text-primary hover:bg-white/5 transition-colors"
            onClick={handleMinimize}
          >
            <Minus size={16} />
          </button>
          <button 
            className="w-10 h-8 flex items-center justify-center text-text-secondary hover:text-text-primary hover:bg-white/5 transition-colors"
            onClick={handleMaximize}
          >
            <Square size={14} />
          </button>
          <button 
            className="w-10 h-8 flex items-center justify-center text-text-secondary hover:text-red-500 hover:bg-red-500/10 transition-colors"
            onClick={handleClose}
          >
            <X size={16} />
          </button>
        </div>
      </div>
      
      {/* 应用内容 */}
      <div className="flex-1 overflow-hidden">
        {children}
      </div>
    </div>
  );
};

export default AppWindowFrame;