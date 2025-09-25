export interface User { id: string; email: string; displayName: string; }
export interface AuthTokens { accessToken: string; refreshToken?: string; exp?: number; }
