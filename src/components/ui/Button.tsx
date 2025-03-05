import React from 'react';

type ButtonVariant = 'primary' | 'outline' | 'ghost' | 'icon';
type ButtonSize = 'sm' | 'md' | 'lg';

type ButtonProps = {
  children: React.ReactNode;
  variant?: ButtonVariant;
  size?: ButtonSize;
  className?: string;
  onClick?: () => void;
  disabled?: boolean;
  icon?: React.ReactNode;
  type?: 'button' | 'submit' | 'reset';
};

const Button: React.FC<ButtonProps> = ({
  children,
  variant = 'primary',
  size = 'md',
  className = '',
  onClick,
  disabled = false,
  icon
}) => {
  // Base styles for all buttons
  const baseStyles = 'relative overflow-hidden transition-all duration-300 font-medium rounded-lg inline-flex items-center justify-center';
  
  // Size-specific styles
  const sizeStyles = {
    sm: 'px-3 py-2 text-sm',
    md: 'px-4 py-2.5 text-base',
    lg: 'px-6 py-3 text-lg',
  };
  
  // Variant-specific styles
  const variantStyles = {
    primary: 'bg-gradient-to-r from-accent-primary to-accent-secondary text-bg-primary hover:shadow-glow-primary',
    outline: 'border border-accent-primary bg-transparent text-text-primary hover:bg-accent-primary/10',
    ghost: 'bg-transparent text-text-primary hover:bg-white/5',
    icon: 'w-10 h-10 p-0 rounded-full bg-white/5 text-text-primary hover:bg-accent-primary/10 hover:text-accent-primary'
  };

  // Handle the onClick event, if disabled, do nothing
  const handleClick = disabled ? undefined : onClick;

  return (
    <button
      className={`
        ${baseStyles} 
        ${sizeStyles[size]} 
        ${variantStyles[variant]}
        ${disabled ? 'opacity-50 cursor-not-allowed' : 'hover:transform hover:-translate-y-1'}
        ${className}
      `}
      onClick={handleClick}
      disabled={disabled}
    >
      {/* Glow animation for primary buttons */}
      {variant === 'primary' && (
        <span className="absolute inset-0 overflow-hidden">
          <span className="absolute top-0 -left-3/4 w-full h-full bg-white/20 transform skew-x-12 transition-all duration-1000 animate-glow" />
        </span>
      )}
      
      {/* Icon + content layout */}
      {icon && <span className="mr-2">{icon}</span>}
      {children}
    </button>
  );
};

export default Button;