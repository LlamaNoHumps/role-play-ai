import { Routes } from '@angular/router';
import { RoleGeneratorComponent } from './role-generator/role-generator.component';
import { RoleListComponent } from './role-list/role-list.component';

// export const ROLES_ROUTES: Routes = [
//   { path: '', component: RoleListComponent },
//   { path: 'generate', component: RoleGeneratorComponent }
// ];

// export default ROLES_ROUTES;

const routes: Routes = [
  { path: '', component: RoleListComponent },
  { path: 'generate', component: RoleGeneratorComponent },
];

export default routes;
