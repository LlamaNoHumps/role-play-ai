import { Component} from '@angular/core';
import { Router } from '@angular/router';
import { AuthService } from '../../../core/services/auth.service';
import { FormGroup, FormBuilder, Validators, ReactiveFormsModule } from '@angular/forms';  // Reactive Forms
import { MatButtonModule } from '@angular/material/button';   // 导入 material 按钮模块
import { MatFormFieldModule } from '@angular/material/form-field'; // 导入 material 表单模块
import { MatInputModule } from '@angular/material/input';   // 导入 material 输入框模块
import { MatCardModule } from '@angular/material/card';   // 导入 material 卡片模块


@Component({
  selector: 'app-register',
  templateUrl: './register.component.html',
  styleUrls: ['./register.component.scss'],
  standalone: true,
  imports: [MatButtonModule, MatFormFieldModule, MatInputModule, MatCardModule, ReactiveFormsModule]  // 添加模块导入
})
export class RegisterComponent {
  registerForm: FormGroup;
  loading = false;
  msg = '';

  constructor(private fb: FormBuilder, private auth: AuthService, private router: Router) {
    // 使用 FormBuilder 创建表单并进行验证
    this.registerForm = this.fb.group({
      email: ['', [Validators.required, Validators.email]],
      displayName: ['', Validators.required],
      password: ['', [Validators.required, Validators.minLength(6)]],
    });
  }

  // 提交注册表单
  submit() {
    if (this.registerForm.invalid) {
      this.msg = '请正确填写表单';  // 如果表单无效，显示错误提示
      return;
    }

    this.loading = true;
    this.msg = '';

    const { email, password, displayName } = this.registerForm.value;

    this.auth.register(email, password, displayName).subscribe({
      next: () => {
        this.loading = false;
        this.router.navigateByUrl('/roles');  // 注册成功后跳转到角色页面
      },
      error: (err) => {
        this.loading = false;
        this.msg = '注册失败：' + err.message || '发生未知错误';  // 显示错误信息
      }
    });
  }

  // 获取表单字段的验证状态
  get email() { return this.registerForm.get('email'); }
  get displayName() { return this.registerForm.get('displayName'); }
  get password() { return this.registerForm.get('password'); }
}

