import { Injectable } from '@angular/core';
import { ApiService } from './api.service';
import { AuthTokens, User } from '../models/auth';
import {jwtDecode} from 'jwt-decode';
import { tap } from 'rxjs/operators';

@Injectable({ providedIn: 'root' })
export class AuthService {
  private readonly KEY = 'tokens';
  profile: User | null = null;

  constructor(private api: ApiService) {}

  register(email: string, password: string, displayName: string){
    return this.api.post<{user:User, tokens:AuthTokens}>('/auth/register', { email, password, displayName })
      .pipe(tap(res => this._afterAuth(res)));
  }
  login(email: string, password: string){
    return this.api.post<{user:User, tokens:AuthTokens}>('/auth/login', { email, password })
      .pipe(tap(res => this._afterAuth(res)));
  }
  private _afterAuth(res: {user:User, tokens:AuthTokens}) {
    this.profile = res.user;
    localStorage.setItem(this.KEY, JSON.stringify(res.tokens));
  }

  get accessToken(): string | null {
    const raw = localStorage.getItem(this.KEY); if (!raw) return null;
    return (JSON.parse(raw) as AuthTokens).accessToken;
  }
  logout(){ localStorage.removeItem(this.KEY); this.profile = null; }

  isAuthenticated(): boolean {
    const token = this.accessToken; if (!token) return false;
    try { const { exp } = jwtDecode<{exp:number}>(token); return !exp || Date.now()/1000 < exp; }
    catch { return false; }
  }
}
