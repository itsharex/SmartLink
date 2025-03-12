// src/lib/authApi.ts
import { invoke } from '@tauri-apps/api/core';

export interface User {
  id: string;
  email: string;
  name?: string;
  [key: string]: any; 
}

export interface LoginResponse {
  user: User;
  token: string;
  token_expires: string;
}

export async function loginWithEmail(email: string, password: string): Promise<LoginResponse> {
  return invoke<LoginResponse>('login_with_email', { 
    request: { email, password } 
  });
}

export async function registerUser(name: string, email: string, password: string): Promise<LoginResponse> {
  return invoke<LoginResponse>('register_user', { 
    request: { email, password, name } 
  });
}

export async function getOAuthUrl(provider: string): Promise<string> {
  return invoke<string>('get_oauth_url', { 
    request: { provider } 
  });
}

export async function handleOAuthCallback(provider: string, code: string, state?: string): Promise<LoginResponse> {
  return invoke<LoginResponse>('handle_oauth_callback', { 
    request: { provider, code, state } 
  });
}

export async function getCurrentUser(): Promise<User> {
  return invoke<User>('get_current_user');
}

export async function logout(): Promise<void> {
  return invoke<void>('logout');
}

export async function refreshToken(): Promise<LoginResponse> {
  return invoke<LoginResponse>('refresh_token');
}