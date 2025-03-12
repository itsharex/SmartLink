'use client';  
import React, { useState } from 'react'; 
import { registerUser } from '../../lib/authApi';   

interface SignupFormProps {   
  onSignupSuccess: (userData: any) => void; 
}  

export default function SignupForm({ onSignupSuccess }: SignupFormProps) {   
  const [name, setName] = useState('');   
  const [email, setEmail] = useState('');   
  const [password, setPassword] = useState('');   
  const [confirmPassword, setConfirmPassword] = useState('');   
  const [termsAccepted, setTermsAccepted] = useState(false);   
  const [error, setError] = useState('');   
  const [isLoading, setIsLoading] = useState(false);    

  const handleSubmit = async (e: React.FormEvent) => {     
    e.preventDefault();          
    
    if (!name || !email || !password) {       
      setError('All fields are required');       
      return;     
    }          
    
    if (password !== confirmPassword) {       
      setError('Passwords do not match');       
      return;     
    }          
    
    if (!termsAccepted) {       
      setError('You must accept the Terms of Service');       
      return;     
    }          
    
    setIsLoading(true);     
    setError('');          
    
    try {       
      const response = await registerUser(name, email, password);              
      localStorage.setItem('authToken', response.token);       
      localStorage.setItem('user', JSON.stringify(response.user));              
      onSignupSuccess(response.user);     
    } catch (err: any) {       
      console.error('Registration error:', err);       
      setError(err.message || 'Failed to register');     
    } finally {       
      setIsLoading(false);     
    }   
  };    

  return (     
    <form onSubmit={handleSubmit} className="space-y-4">       
      {error && (
        <div className="p-3 bg-red-500/10 border border-red-500/20 rounded-lg text-red-500 text-sm">
          {error}
        </div>
      )}
      
      <div className="space-y-2">         
        <label className="block text-sm text-text-primary-70">Username</label>         
        <input           
          type="text"           
          value={name}
          onChange={(e) => setName(e.target.value)}
          className="w-full border bg-bg-tertiary rounded-lg px-4 py-2 text-text-primary-70"           
          placeholder="Your username"         
        />       
      </div>       
      
      <div className="space-y-2">         
        <label className="block text-sm text-text-primary-70">Email</label>         
        <input           
          type="email"           
          value={email}
          onChange={(e) => setEmail(e.target.value)}
          className="w-full border bg-bg-tertiary rounded-lg px-4 py-2 text-text-primary-70"           
          placeholder="your@email.com"         
        />       
      </div>       
      
      <div className="space-y-2">         
        <label className="block text-sm text-text-primary-70">Password</label>         
        <input           
          type="password"           
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          className="w-full border bg-bg-tertiary rounded-lg px-4 py-2 text-text-primary-70"           
          placeholder="••••••••"         
        />       
      </div>       
      
      <div className="space-y-2">         
        <label className="block text-sm text-text-primary-70">Confirm Password</label>         
        <input           
          type="password"           
          value={confirmPassword}
          onChange={(e) => setConfirmPassword(e.target.value)}
          className="w-full border bg-bg-tertiary rounded-lg px-4 py-2 text-text-primary-70"           
          placeholder="••••••••"         
        />       
      </div>       
      
      <div className="flex items-center">         
        <input           
          id="terms"           
          type="checkbox"           
          checked={termsAccepted}
          onChange={(e) => setTermsAccepted(e.target.checked)}
          className="w-4 h-4 border-text-primary-30 bg-bg-tertiary rounded"         
        />         
        <label htmlFor="terms" className="ml-2 text-sm text-text-primary-30">           
          I agree to the Terms of Service and Privacy Policy         
        </label>       
      </div>       
      
      <button         
        type="submit"         
        disabled={isLoading}
        className="w-full py-3 px-4 rounded-lg bg-gradient-to-r from-accent-primary to-accent-secondary text-white font-medium transition transform hover:bg-gradient-to-r hover:from-accent-primary-80 hover:to-accent-secondary-80 disabled:opacity-70"       
      >         
        {isLoading ? 'Creating Account...' : 'Sign Up'}        
      </button>     
    </form>   
  ); 
}