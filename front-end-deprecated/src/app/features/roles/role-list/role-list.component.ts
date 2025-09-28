import { Component,OnInit } from '@angular/core';
import { RoleService } from '../../../core/services/role.service';
import { Role } from '../../../core/models/role';
import { RouterLink } from '@angular/router';
import { CommonModule } from '@angular/common';

@Component({
  standalone: true,
  selector: 'app-role-list',
  imports: [CommonModule, RouterLink],
  templateUrl: './role-list.component.html',
  styleUrl: './role-list.component.scss'
})
export class RoleListComponent implements OnInit {
  roles: Role[] = [];
  constructor(private roleSvc: RoleService){}
  ngOnInit(){ this.roleSvc.list().subscribe(r => this.roles = r); }
}
