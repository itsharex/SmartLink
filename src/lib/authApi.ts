import { invoke } from '@tauri-apps/api/core';

export interface authUser {
  id: string;
  email: string;
  name?: string;
  [key: string]: any;
}

export interface LoginResponse {
  user: authUser;
  token: string;
  token_expires: string;
}

export async function loginWithEmail(email: string, password: string): Promise<authUser> {
  const response = await invoke<LoginResponse>('login_with_email', { request: { email, password } });

  localStorage.setItem('authToken', response.token);

  return response.user;
}

export async function registerUser(name: string, email: string, password: string): Promise<authUser> {
  return invoke<authUser>('register_user', { email, password, name });
}

export async function getOAuthUrl(provider: string): Promise<string> {
  return invoke<string>('get_oauth_url', { provider });
}

export async function handleOAuthCallback(provider: string, code: string, state?: string): Promise<authUser> {
  return invoke<authUser>('handle_oauth_callback', { provider, code, state });
}

export async function getCurrentUser(): Promise<authUser> {
  const token = localStorage.getItem('authToken');
  if (!token) throw new Error("No authentication token found");

  return invoke<authUser>('get_current_user', { token });
}


export async function logout(): Promise<void> {
  return invoke<void>('logout');
}

export async function refreshToken(): Promise<authUser> {
  return invoke<authUser>('refresh_token');
}