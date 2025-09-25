import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
// import { environment } from '../../../environments/environment';

@Injectable({ providedIn: 'root' })
export class ApiService {
  constructor(private http: HttpClient) {}
  get base() { return "environment.apiBase"; }
  get<T>(url: string, params?: any){ return this.http.get<T>(`${this.base}${url}`, { params }); }
  post<T>(url: string, body?: any){ return this.http.post<T>(`${this.base}${url}`, body); }
}
