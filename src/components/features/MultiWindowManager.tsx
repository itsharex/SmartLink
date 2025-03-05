import React, { useState } from 'react';
import GlassCard from '../ui/GlassCard';
import Button from '../ui/Button';
import { LayoutGrid, Maximize2, Minimize2, X, Monitor, Copy, PlusSquare, MoveHorizontal } from 'lucide-react';

// 每个窗口的状态
type WindowState = {
  id: string;
  title: string;
  type: 'chat' | 'settings' | 'contacts' | 'group';
  minimized: boolean;
  maximized: boolean;
  width: number;
  height: number;
  posX: number;
  posY: number;
  active: boolean;
  chatId?: string;
};

type MultiWindowManagerProps = {
  onCreateWindow: (type: string, options?: any) => void;
  onSwitchWindow: (windowId: string) => void;
  onCloseWindow: (windowId: string) => void;
};

const MultiWindowManager: React.FC<MultiWindowManagerProps> = ({
  onCreateWindow,
  onSwitchWindow,
  onCloseWindow
}) => {
  // 示例窗口数据
  const [windows, setWindows] = useState<WindowState[]>([
    {
      id: 'main',
      title: 'SmartLink',
      type: 'chat',
      minimized: false,
      maximized: false,
      width: 1200,
      height: 800,
      posX: 100,
      posY: 100,
      active: true
    },
    {
      id: 'chat-1',
      title: '李明 - 聊天',
      type: 'chat',
      minimized: false,
      maximized: false,
      width: 800,
      height: 600,
      posX: 200,
      posY: 150,
      active: false,
      chatId: 'user-1'
    },
    {
      id: 'settings',
      title: '设置',
      type: 'settings',
      minimized: true,
      maximized: false,
      width: 900,
      height: 700,
      posX: 300,
      posY: 200,
      active: false
    }
  ]);
  
  const [layoutMode, setLayoutMode] = useState<'grid' | 'stack'>('stack');
  const [createMenuOpen, setCreateMenuOpen] = useState(false);
  
  // 切换最小化状态
  const toggleMinimize = (id: string) => {
    setWindows(prev => 
      prev.map(win => 
        win.id === id ? { ...win, minimized: !win.minimized } : win
      )
    );
  };
  
  // 切换最大化状态
  const toggleMaximize = (id: string) => {
    setWindows(prev => 
      prev.map(win => 
        win.id === id ? { ...win, maximized: !win.maximized } : win
      )
    );
  };
  
  // 切换激活窗口
  const activateWindow = (id: string) => {
    setWindows(prev => 
      prev.map(win => 
        win.id === id ? { ...win, active: true } : { ...win, active: false }
      )
    );
    onSwitchWindow(id);
  };
  
  // 关闭窗口
  const closeWindow = (id: string) => {
    if (id === 'main') return; // 不允许关闭主窗口
    setWindows(prev => prev.filter(win => win.id !== id));
    onCloseWindow(id);
  };
  
  // 创建新窗口
  const createNewWindow = (type: string) => {
    const newId = `${type}-${Date.now()}`;
    const newWindow: WindowState = {
      id: newId,
      title: type === 'chat' ? '新聊天窗口' : type === 'settings' ? '设置' : '新窗口',
      type: type as any,
      minimized: false,
      maximized: false,
      width: 800,
      height: 600,
      posX: 150,
      posY: 150,
      active: false
    };
    
    setWindows(prev => [...prev, newWindow]);
    onCreateWindow(type);
    setCreateMenuOpen(false);
  };
  
  // 应用网格布局
  const applyGridLayout = () => {
    setLayoutMode('grid');
    // 这里应该通过 Tauri API 实际应用网格布局
    console.log('Apply grid layout');
  };
  
  // 应用堆叠布局
  const applyStackLayout = () => {
    setLayoutMode('stack');
    // 这里应该通过 Tauri API 实际应用堆叠布局
    console.log('Apply stack layout');
  };

  return (
    <GlassCard className="w-96 p-4 border border-white/10 shadow-xl">
      <div className="flex justify-between items-center mb-4">
        <h3 className="font-semibold text-text-primary flex items-center gap-2">
          <Monitor size={18} className="text-accent-primary" />
          窗口管理
        </h3>
        <div className="flex items-center gap-2">
          <button 
            className={`w-8 h-8 flex items-center justify-center rounded ${layoutMode === 'grid' ? 'bg-accent-primary/20 text-accent-primary' : 'bg-white/5 text-text-secondary'}`}
            onClick={applyGridLayout}
            title="网格布局"
          >
            <LayoutGrid size={16} />
          </button>
          <button 
            className={`w-8 h-8 flex items-center justify-center rounded ${layoutMode === 'stack' ? 'bg-accent-primary/20 text-accent-primary' : 'bg-white/5 text-text-secondary'}`}
            onClick={applyStackLayout}
            title="堆叠布局"
          >
            <Copy size={16} />
          </button>
        </div>
      </div>
      
      <div className="mb-4 relative">
        <Button 
          variant="outline" 
          className="w-full flex items-center justify-center gap-2"
          onClick={() => setCreateMenuOpen(!createMenuOpen)}
        >
          <PlusSquare size={16} />
          创建新窗口
        </Button>
        
        {createMenuOpen && (
          <div className="absolute top-full left-0 right-0 mt-1 bg-bg-tertiary/90 backdrop-blur-md rounded-lg border border-white/10 shadow-lg z-10">
            <button 
              className="w-full px-4 py-2 text-left hover:bg-white/5 transition text-text-primary"
              onClick={() => createNewWindow('chat')}
            >
              聊天窗口
            </button>
            <button 
              className="w-full px-4 py-2 text-left hover:bg-white/5 transition text-text-primary"
              onClick={() => createNewWindow('settings')}
            >
              设置窗口
            </button>
            <button 
              className="w-full px-4 py-2 text-left hover:bg-white/5 transition text-text-primary"
              onClick={() => createNewWindow('contacts')}
            >
              联系人窗口
            </button>
          </div>
        )}
      </div>
      
      <div className="space-y-2 max-h-80 overflow-y-auto pr-1">
        {windows.map((window) => (
          <div 
            key={window.id}
            className={`p-3 rounded-lg border ${window.active ? 'border-accent-primary/30 bg-accent-primary/5' : 'border-white/5 bg-white/5'} flex flex-col`}
          >
            <div className="flex justify-between items-center">
              <div 
                className="flex-1 font-medium text-text-primary truncate cursor-pointer"
                onClick={() => activateWindow(window.id)}
              >
                {window.title}
              </div>
              <div className="flex gap-1">
                <button 
                  className="w-6 h-6 flex items-center justify-center text-text-secondary hover:text-text-primary hover:bg-white/10 rounded transition"
                  onClick={() => toggleMinimize(window.id)}
                >
                  {window.minimized ? <Maximize2 size={12} /> : <Minimize2 size={12} />}
                </button>
                {window.id !== 'main' && (
                  <button 
                    className="w-6 h-6 flex items-center justify-center text-text-secondary hover:text-red-500 hover:bg-red-500/10 rounded transition"
                    onClick={() => closeWindow(window.id)}
                  >
                    <X size={12} />
                  </button>
                )}
              </div>
            </div>
            
            <div className="flex items-center justify-between mt-2 text-xs text-text-secondary">
              <div className="flex items-center gap-1">
                <MoveHorizontal size={12} />
                <span>{window.width} × {window.height}</span>
              </div>
              <span>{window.minimized ? '最小化' : window.maximized ? '最大化' : '正常'}</span>
            </div>
          </div>
        ))}
      </div>
      
      <div className="flex justify-between mt-4 pt-4 border-t border-white/10 text-sm">
        <span className="text-text-secondary">当前活动窗口: {windows.find(w => w.active)?.title}</span>
        <span className="text-text-secondary">总计: {windows.length}</span>
      </div>
    </GlassCard>
  );
};

export default MultiWindowManager;