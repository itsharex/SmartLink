import React, { useState } from 'react';
import GlassCard from '../ui/GlassCard';
import { Lock, Fingerprint, Key, ShieldCheck, ExternalLink } from 'lucide-react';

type EncryptionIndicatorProps = {
  isEncrypted: boolean;
  encryptionType?: 'e2ee' | 'tls';
  lastVerified?: string;
  contactName: string;
  onViewDetails?: () => void;
};

const EncryptionIndicator: React.FC<EncryptionIndicatorProps> = ({
  isEncrypted = true,
  encryptionType = 'e2ee',
  lastVerified = '2023-11-15',
  contactName,
  onViewDetails
}) => {
  const [showTooltip, setShowTooltip] = useState(false);
  
  // 简易描述
  const getDescription = () => {
    if (!isEncrypted) return '此对话未加密';
    return encryptionType === 'e2ee' 
      ? '端到端加密，仅对话双方可以读取消息' 
      : '传输层加密，服务器可以读取消息';
  };
  
  // 状态颜色
  const statusColor = !isEncrypted 
    ? 'text-red-500' 
    : encryptionType === 'e2ee' 
      ? 'text-accent-primary' 
      : 'text-yellow-500';
  
  return (
    <div className="relative">
      {/* 点击图标 */}
      <button 
        className={`flex items-center gap-1 ${statusColor} hover:underline`}
        onMouseEnter={() => setShowTooltip(true)}
        onMouseLeave={() => setShowTooltip(false)}
        onClick={() => onViewDetails?.()}
      >
        <Lock size={14} />
        <span className="text-xs font-medium">
          {isEncrypted ? (encryptionType === 'e2ee' ? '端到端加密' : '传输层加密') : '未加密'}
        </span>
      </button>
      
      {/* 详细信息弹出框 */}
      {showTooltip && (
        <div className="absolute bottom-full left-0 mb-2 w-72 z-50">
          <GlassCard className="p-4 border border-white/10 shadow-xl">
            <div className="flex items-center gap-2 mb-3">
              <div className={`w-8 h-8 rounded-full flex items-center justify-center ${statusColor} bg-white/10`}>
                <Lock size={16} />
              </div>
              <div>
                <p className="font-medium text-text-primary">
                  {isEncrypted ? (encryptionType === 'e2ee' ? '端到端加密' : '传输层加密') : '未加密'}
                </p>
                <p className="text-xs text-text-secondary">{getDescription()}</p>
              </div>
            </div>
            
            {isEncrypted && encryptionType === 'e2ee' && (
              <>
                <div className="bg-white/5 rounded-lg p-3 mb-3">
                  <div className="flex items-center gap-2">
                    <Fingerprint size={14} className="text-accent-primary" />
                    <p className="text-xs text-text-secondary">
                      与 <span className="text-text-primary">{contactName}</span> 的加密会话安全
                    </p>
                  </div>
                  
                  <div className="flex items-center gap-2 mt-2">
                    <Key size={14} className="text-accent-primary" />
                    <p className="text-xs text-text-secondary">
                      密钥指纹:
                      <span className="text-text-primary font-mono text-[10px] ml-1">
                        05:AC:8D:52:25:C8:24:45:35:CB:D5:87:9F:FB:60:79
                      </span>
                    </p>
                  </div>
                  
                  <div className="mt-2 flex items-center gap-2">
                    <ShieldCheck size={14} className="text-accent-primary" />
                    <p className="text-xs text-text-secondary">
                      上次验证: {lastVerified}
                    </p>
                  </div>
                </div>
                
                <button 
                  className="w-full flex items-center justify-center gap-1 text-xs text-accent-primary hover:underline"
                  onClick={() => onViewDetails?.()}
                >
                  <ExternalLink size={12} />
                  查看加密详情
                </button>
              </>
            )}
          </GlassCard>
        </div>
      )}
    </div>
  );
};

export default EncryptionIndicator;