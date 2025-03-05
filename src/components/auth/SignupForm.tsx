import React, { useState } from 'react';
import { useRouter } from 'next/navigation';
import GlassCard from '../ui/GlassCard';
import Button from '../ui/Button';
import { Mail, Lock, User, Eye, EyeOff, CheckCircle, AlertCircle } from 'lucide-react';

type SignupFormProps = {
  onToggleForm?: () => void; // 切换到登录表单
};

const SignupForm: React.FC<SignupFormProps> = ({ onToggleForm }) => {
  const router = useRouter();
  const [username, setUsername] = useState('');
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [showPassword, setShowPassword] = useState(false);
  const [agreeTerms, setAgreeTerms] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    // 简单校验
    if (!username || !email || !password) {
      setError('请填写所有必填字段');
      return;
    }
    
    if (password !== confirmPassword) {
      setError('两次输入的密码不一致');
      return;
    }
    
    if (!agreeTerms) {
      setError('请阅读并同意服务条款');
      return;
    }
    
    setIsLoading(true);
    setError(null);
    
    try {
      // 模拟注册请求
      await new Promise(resolve => setTimeout(resolve, 1500));
      
      // 注册成功后跳转
      router.push('/onboarding');
    } catch (err) {
      setError('注册失败，请稍后重试');
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <GlassCard className="w-full max-w-md p-8">
      <div className="text-center mb-8">
        <h2 className="text-2xl font-bold font-orbitron text-text-primary mb-2">创建 SmartLink 账号</h2>
        <p className="text-text-secondary">开始你的智能社交之旅</p>
      </div>
      
      {error && (
        <div className="mb-6 p-3 rounded-lg bg-red-500/10 border border-red-500/30 text-red-400 flex items-center gap-2">
          <AlertCircle size={18} />
          <p>{error}</p>
        </div>
      )}
      
      <form onSubmit={handleSubmit} className="space-y-5">
        <div className="space-y-2">
          <label className="block text-sm text-text-secondary">用户名</label>
          <div className="relative">
            <div className="absolute left-3 top-1/2 transform -translate-y-1/2 text-text-secondary">
              <User size={18} />
            </div>
            <input
              type="text"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              className="w-full bg-white/5 border border-white/10 rounded-lg px-10 py-3 text-text-primary focus:outline-none focus:ring-2 focus:ring-accent-primary/50 transition"
              placeholder="用户名"
            />
          </div>
        </div>
        
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
        
        <div className="space-y-2">
          <label className="block text-sm text-text-secondary">确认密码</label>
          <div className="relative">
            <div className="absolute left-3 top-1/2 transform -translate-y-1/2 text-text-secondary">
              <Lock size={18} />
            </div>
            <input
              type={showPassword ? "text" : "password"}
              value={confirmPassword}
              onChange={(e) => setConfirmPassword(e.target.value)}
              className="w-full bg-white/5 border border-white/10 rounded-lg px-10 py-3 text-text-primary focus:outline-none focus:ring-2 focus:ring-accent-primary/50 transition"
              placeholder="••••••••"
            />
          </div>
        </div>
        
        <div className="flex items-center gap-2">
          <div className="relative">
            <input
              type="checkbox"
              checked={agreeTerms}
              onChange={() => setAgreeTerms(!agreeTerms)}
              className="sr-only"
            />
            <div className={`w-4 h-4 border rounded ${agreeTerms ? 'bg-accent-primary border-accent-primary' : 'border-white/30'} transition cursor-pointer`} onClick={() => setAgreeTerms(!agreeTerms)}>
              {agreeTerms && <CheckCircle size={16} className="text-bg-primary" />}
            </div>
          </div>
          <span className="text-sm text-text-secondary">
            我已阅读并同意 <a href="#" className="text-accent-primary hover:underline">服务条款</a> 和 <a href="#" className="text-accent-primary hover:underline">隐私政策</a>
          </span>
        </div>
        
        <Button
          type="submit"
          variant="primary"
          className="w-full py-3 mt-4"
          disabled={isLoading}
        >
          {isLoading ? '注册中...' : '创建账号'}
        </Button>
      </form>
      
      <p className="text-center mt-8 text-text-secondary">
        已有账号? 
        <button 
          onClick={onToggleForm}
          className="ml-1 text-accent-primary hover:underline"
        >
          立即登录
        </button>
      </p>
    </GlassCard>
  );
};

export default SignupForm;