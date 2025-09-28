import { Component } from '@angular/core';
import { FormBuilder, ReactiveFormsModule, Validators } from '@angular/forms';

import { RoleService } from '../../../core/services/role.service';
import { Router } from '@angular/router';
@Component({
  selector: 'app-role-generator',
  standalone: true,
  imports: [ReactiveFormsModule],
  templateUrl: './role-generator.component.html',
  styleUrl: './role-generator.component.scss'
})
export class RoleGeneratorComponent {
  form: any
  loading=false;
  msg='';
  constructor(private fb: FormBuilder,
              private roleSvc: RoleService,
              private router: Router){
                  this.form = this.fb.group({
    name: ['', Validators.required],
    tone: ['温柔'],
    domain: ['治愈'],
    traits: ['理性;同理心;幽默']
  });
              }



  submit(){
    if (this.form.invalid) return;
    this.loading = true; this.msg='';
    this.roleSvc.generate(this.form.value as any).subscribe({
      next: _ => { this.loading=false; this.router.navigateByUrl('/roles'); },
      error: _ => { this.msg='生成失败'; this.loading=false; }
    });
  }
}
