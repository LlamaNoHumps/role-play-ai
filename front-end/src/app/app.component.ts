import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { MatToolbarModule } from '@angular/material/toolbar';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatMenuModule } from '@angular/material/menu';
import { AuthService } from './core/services/auth.service';
import { Router } from '@angular/router';
import { RouterLink } from '@angular/router';
@Component({
  standalone: true,
  selector: 'app-root',
  imports: [RouterOutlet, MatButtonModule, MatIconModule, MatMenuModule, MatToolbarModule, RouterLink],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss'
})
export class AppComponent {
  constructor(public auth: AuthService, private router: Router) {}
  logout() { this.auth.logout(); this.router.navigateByUrl('/auth/login'); }
  title = 'front-end';

}
