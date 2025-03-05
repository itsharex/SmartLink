import React, { useState } from 'react';
import { useRouter } from 'next/navigation';
import GlassCard from '../ui/GlassCard';
import Button from '../ui/Button';
import { Mail, Lock, Eye, EyeOff, CheckCircle, AlertCircle } from 'lucide-react';

type LoginFormProps = {
  onToggleForm?: () => void; // 切换到注册表单
};

const LoginForm: React.FC<LoginFormProps> = ({ onToggleForm }) => {
  const router = useRouter();
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [showPassword, setShowPassword] = useState(false);
  const [rememberMe, setRememberMe] = useState(true);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    // 简单校验
    if (!email || !password) {
      setError('请输入邮箱和密码');
      return;
    }
    
    setIsLoading(true);
    setError(null);
    
    try {
      // 模拟登录请求
      await new Promise(resolve => setTimeout(resolve, 1500));
      
      // 登录成功后跳转
      router.push('/chat');
    } catch (err) {
      setError('登录失败，请检查账号和密码');
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <GlassCard className="w-full max-w-md p-8">
      <div className="text-center mb-8">
        <h2 className="text-2xl font-bold font-orbitron text-text-primary mb-2">登录 SmartLink</h2>
        <p className="text-text-secondary">连接你的智能社交世界</p>
      </div>
      
      {error && (
        <div className="mb-6 p-3 rounded-lg bg-red-500/10 border border-red-500/30 text-red-400 flex items-center gap-2">
          <AlertCircle size={18} />
          <p>{error}</p>
        </div>
      )}
      
      <form onSubmit={handleSubmit} className="space-y-6">
        <div className="space-y-2">
          <label className="block text-sm text-text-secondary">邮箱</label>
          <div className="relative">
            <div className="absolute left-3 top-1/2 transform -translate-y-1/2 text-text-secondary">
              <Mail size={18} />
            </div>
            <input
              type="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              className="w-full bg-white/5 border border-white/10 rounded-lg px-10 py-3 text-text-primary focus:outline-none focus:ring-2 focus:ring-accent-primary/50 transition"
              placeholder="your@email.com"
            />
          </div>
        </div>
        
        <div className="space-y-2">
          <label className="block text-sm text-text-secondary">密码</label>
          <div className="relative">
            <div className="absolute left-3 top-1/2 transform -translate-y-1/2 text-text-secondary">
              <Lock size={18} />
            </div>
            <input
              type={showPassword ? "text" : "password"}
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              className="w-full bg-white/5 border border-white/10 rounded-lg px-10 py-3 text-text-primary focus:outline-none focus:ring-2 focus:ring-accent-primary/50 transition"
              placeholder="••••••••"
            />
            <button
              type="button"
              className="absolute right-3 top-1/2 transform -translate-y-1/2 text-text-secondary hover:text-text-primary transition"
              onClick={() => setShowPassword(!showPassword)}
            >
              {showPassword ? <EyeOff size={18} /> : <Eye size={18} />}
            </button>
          </div>
        </div>
        
        <div className="flex items-center justify-between">
          <label className="flex items-center gap-2 cursor-pointer">
            <div className="relative">
              <input
                type="checkbox"
                checked={rememberMe}
                onChange={() => setRememberMe(!rememberMe)}
                className="sr-only"
              />
              <div className={`w-4 h-4 border rounded ${rememberMe ? 'bg-accent-primary border-accent-primary' : 'border-white/30'} transition`}>
                {rememberMe && <CheckCircle size={16} className="text-bg-primary" />}
              </div>
            </div>
            <span className="text-sm text-text-secondary">记住我</span>
          </label>
          
          <a href="#" className="text-sm text-accent-primary hover:underline">忘记密码?</a>
        </div>
        
        <Button
          type="submit"
          variant="primary"
          className="w-full py-3"
          disabled={isLoading}
        >
          {isLoading ? '登录中...' : '登录'}
        </Button>
        
        <div className="relative my-6">
          <div className="absolute inset-0 flex items-center">
            <div className="w-full border-t border-white/10"></div>
          </div>
          <div className="relative flex justify-center">
            <span className="px-2 bg-bg-secondary text-text-secondary text-sm">或</span>
          </div>
        </div>
        
        <div className="grid grid-cols-2 gap-4">
          <button
            type="button"
            className="py-2.5 px-4 border border-white/10 rounded-lg flex items-center justify-center gap-2 hover:bg-white/5 transition"
          >
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="20" height="20" fill="currentColor">
              <path d="M12 2C6.477 2 2 6.477 2 12c0 4.42 2.865 8.166 6.839 9.489.5.092.682-.217.682-.482 0-.237-.008-.866-.013-1.7-2.782.603-3.369-1.34-3.369-1.34-.454-1.157-1.11-1.465-1.11-1.465-.908-.62.069-.608.069-.608 1.003.07 1.531 1.03 1.531 1.03.892 1.529 2.341 1.087 2.91.831.092-.647.35-1.086.634-1.336-2.22-.253-4.555-1.11-4.555-4.943 0-1.091.39-1.984 1.029-2.683-.103-.253-.446-1.27.099-2.646 0 0 .84-.269 2.75 1.026A9.564 9.564 0 0112 6.839c.85.004 1.705.114 2.504.337 1.909-1.295 2.747-1.026 2.747-1.026.546 1.376.202 2.394.1 2.646.64.699 1.028 1.592 1.028 2.683 0 3.842-2.339 4.687-4.566 4.935.359.309.678.919.678 1.852 0 1.336-.012 2.415-.012 2.743 0 .267.18.578.688.48C19.137 20.163 22 16.418 22 12c0-5.523-4.477-10-10-10z"/>
            </svg>
            <span>GitHub</span>
          </button>
          
          <button
            type="button"
            className="py-2.5 px-4 border border-white/10 rounded-lg flex items-center justify-center gap-2 hover:bg-white/5 transition"
          >
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="20" height="20" fill="#DB4437">
              <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm5.46 8.07l-2.12 1.8c-.47.4-.75 1-.75 1.63v.97c0 .63.28 1.23.75 1.63l2.12 1.8c.3.26.48.63.48 1.03 0 .75-.73 1.28-1.42 1.03l-2.33-.84c-.53-.19-1.1-.19-1.62 0l-2.33.84c-.7.25-1.42-.28-1.42-1.03 0-.4.18-.77.48-1.03l2.12-1.8c.47-.4.75-1 .75-1.63v-.97c0-.63-.28-1.23-.75-1.63l-2.12-1.8C6.18 9.33 6 8.97 6 8.57c0-.75.73-1.28 1.42-1.03l2.33.84c.53.19 1.1.19 1.62 0l2.33-.84c.7-.25 1.42.28 1.42 1.03 0 .4-.18.77-.48 1.03z"/>
            </svg>
            <span>Google</span>
          </button>
        </div>
      </form>
      
      <p className="text-center mt-8 text-text-secondary">
        还没有账号? 
        <button 
          onClick={onToggleForm}
          className="ml-1 text-accent-primary hover:underline"
        >
          立即注册
        </button>
      </p>
    </GlassCard>
  );
};

export default LoginForm;