import { Routes } from '@angular/router';
import { AuthGuard } from './core/guards/auth.guard';

export const routes: Routes = [
  { path: '', pathMatch: 'full', redirectTo: 'roles' },
  { path: 'auth', loadChildren: () => import('./features/auth/auth.routes').then(m => m.AUTH_ROUTES) },
  { path: 'roles', canActivate: [AuthGuard],
    loadChildren: () => import('./features/roles/roles.routes').then(m => m.ROLES_ROUTES) },
  { path: 'chat', canActivate: [AuthGuard],
    loadChildren: () => import('./features/chat/chat.routes').then(m => m.CHAT_ROUTES) },
  { path: '**', redirectTo: 'roles' }
];

