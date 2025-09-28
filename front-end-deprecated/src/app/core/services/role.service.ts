import { Injectable } from '@angular/core';
import { ApiService } from './api.service';
import { Role } from '../models/role';

@Injectable({ providedIn: 'root' })
export class RoleService {
  constructor(private api: ApiService) {}
  list(){ return this.api.get<Role[]>('/roles'); }
  generate(payload: {name:string; tone:string; domain:string; traits:string;}){
    return this.api.post<Role>('/roles/generate', payload);
  }
}
