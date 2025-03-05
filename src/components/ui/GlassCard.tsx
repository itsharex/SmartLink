import React from 'react';

type GlassCardProps = {
  children: React.ReactNode;
  className?: string;
  intensity?: 'low' | 'medium' | 'high';
  hover?: boolean;
  border?: boolean;
  onClick?: () => void;
};

const GlassCard: React.FC<GlassCardProps> = ({
  children,
  className = '',
  intensity = 'medium',
  hover = false,
  border = true,
  onClick
}) => {
  // Map intensity to background opacity
  const bgOpacity = {
    low: 'bg-opacity-30',
    medium: 'bg-opacity-50',
    high: 'bg-opacity-70',
  };
  
  // Define hover effects class
  const hoverClass = hover 
    ? 'transition-all duration-300 hover:transform hover:-translate-y-2 hover:shadow-xl' 
    : '';
  
  // Define border class
  const borderClass = border 
    ? 'border border-white/5' 
    : '';

  return (
    <div 
      className={`
        bg-bg-tertiary ${bgOpacity[intensity]} 
        backdrop-blur-lg rounded-2xl ${borderClass} ${hoverClass}
        ${className}
      `}
      onClick={onClick}
    >
      {children}
    </div>
  );
};

export default GlassCard;