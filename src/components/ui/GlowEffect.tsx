import React from 'react';

type GlowEffectProps = {
  color?: 'primary' | 'secondary' | 'tertiary';
  size?: 'sm' | 'md' | 'lg';
  intensity?: 'low' | 'medium' | 'high';
  className?: string;
  style?: React.CSSProperties;
};

const GlowEffect: React.FC<GlowEffectProps> = ({
  color = 'primary',
  size = 'md',
  intensity = 'medium',
  className = '',
  style
}) => {
  // Map sizes to actual pixel values
  const sizeMap = {
    sm: '300px',
    md: '500px',
    lg: '700px',
  };
  
  // Map colors to CSS variables
  const colorMap = {
    primary: 'var(--accent-primary)',
    secondary: 'var(--accent-secondary)',
    tertiary: 'var(--accent-tertiary)',
  };
  
  // Map intensity to opacity values
  const intensityMap = {
    low: '0.1',
    medium: '0.15',
    high: '0.25',
  };

  return (
    <div 
      className={`absolute rounded-full pointer-events-none ${className}`}
      style={{
        width: sizeMap[size],
        height: sizeMap[size],
        background: colorMap[color],
        opacity: intensityMap[intensity],
        filter: 'blur(80px)',
        zIndex: -1,
        ...style
      }}
    />
  );
};

export default GlowEffect;