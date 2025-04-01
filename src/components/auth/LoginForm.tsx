'use client';

import React, { useState } from 'react';
import { useAuth } from '@/context/AuthContext'; // Import useAuth
import { loginWithEmail } from '../../lib/authApi';

interface LoginFormProps {
  handleAuth: (e: React.FormEvent) => void;
  onLoginSuccess: (userData: any) => void;
}

export default function LoginForm({ handleAuth, onLoginSuccess }: LoginFormProps) {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const { setUser } = useAuth(); // Get setUser from AuthContext

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!email || !password) {
      setError('Email and password are required');
      return;
    }
    
    setIsLoading(true);
    setError('');
    
    try {
      const user = await loginWithEmail(email, password); // Expect authUser
      console.log("Logged in user:", user);
      
      // Store user in localStorage (optional)
      localStorage.setItem('user', JSON.stringify(user));
      
      // Update AuthProvider state
      setUser(user);
      
      // Trigger onLoginSuccess (e.g., redirect)
      onLoginSuccess(user);
    } catch (err: any) {
      console.error('Login error:', err);
      setError(err.message || 'Failed to login');
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      {error && (
        <div className="p-3 bg-red-500/10 border border-red-500/20 rounded-lg text-red-500 text-sm">
          {error}
        </div>
      )}
      
      <div className="space-y-2">
        <label className="block text-sm text-text-primary-70">Email</label>
        <input
          type="email"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
          className="w-full border bg-bg-tertiary rounded-lg px-4 py-3 text-text-primary-70"
          placeholder="your@email.com"
        />
      </div>
      
      <div className="space-y-2">
        <div className="flex justify-between">
          <label className="block text-sm text-text-primary-70">Password</label>
          <a href="#" className="text-sm text-accent-primary-80 hover:underline">
            Forgot Password?
          </a>
        </div>
        <input
          type="password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          className="w-full border bg-bg-tertiary rounded-lg px-4 py-3 text-text-primary-70"
          placeholder="••••••••"
        />
      </div>
      
      <button
        type="submit"
        disabled={isLoading}
        className="w-full py-3 px-4 rounded-lg bg-gradient-to-r from-accent-primary to-accent-secondary text-white font-medium transition transform hover:opacity-90 disabled:opacity-70"
      >
        {isLoading ? 'Logging In...' : 'Log In'}
      </button>
    </form>
  );
}