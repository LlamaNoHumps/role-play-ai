import { Component } from '@angular/core';
import { Router } from '@angular/router';
import { AuthService } from '../../../core/services/auth.service';
import { FormsModule } from '@angular/forms'; // 你需要在模块中导入
import { MatCardModule } from '@angular/material/card';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatButtonModule } from '@angular/material/button';
import { NgIf } from '@angular/common';

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.scss'],
  standalone: true,
  imports: [
    FormsModule,
    MatCardModule,
    MatFormFieldModule,
    MatInputModule,
    MatButtonModule,
    NgIf,
    // 注意：RouterLink 会自动可用，只要 AppModule 或 Lazy Module 导入了 RouterModule
  ]
})
export class LoginComponent {
  // 初始化表单数据
  email = 'demo@demo.com';
  password = '123456';

  // 状态控制
  loading = false;
  errorMsg = '';

  constructor(private auth: AuthService, private router: Router) {}

  /**
   * 处理登录表单提交
   */
  onSubmit(): void {
    // 前端基础校验
    if (!this.email || !this.password) {
      this.errorMsg = '请输入邮箱和密码';
      return;
    }

    if (!this.isValidEmail(this.email)) {
      this.errorMsg = '请输入有效的邮箱地址';
      return;
    }

    this.loading = true;
    this.errorMsg = '';

    this.auth.login(this.email, this.password).subscribe({
      next: () => {
        this.loading = false;
        // 登录成功，跳转到角色页
        this.router.navigate(['/roles']);
      },
      error: (err) => {
        this.loading = false;
        this.errorMsg = err.message || '登录失败，请检查账号密码';
        console.error('Login error:', err);
      }
    });
  }

  /**
   * 邮箱格式校验
   */
  private isValidEmail(email: string): boolean {
    const emailRegex = /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/;
    return emailRegex.test(email);
  }
}
