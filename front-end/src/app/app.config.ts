import { ApplicationConfig, provideZoneChangeDetection, importProvidersFrom } from '@angular/core';
import { provideRouter } from '@angular/router';

import { routes } from './app.routes';
import { provideClientHydration, withEventReplay } from '@angular/platform-browser';
import { provideAnimations } from '@angular/platform-browser/animations';
import { provideHttpClient, withInterceptors } from '@angular/common/http';
import { authInterceptor } from './core/interceptors/auth.interceptor';
import { errorInterceptor } from './core/interceptors/error.interceptor';
// 本地 Mock 后端（开发时启用）
// import { HttpClientInMemoryWebApiModule } from 'angular-in-memory-web-api';
// import { MockBackendService } from '../mocks/mock-backend.service';
export const appConfig: ApplicationConfig = {
  providers: [
    provideZoneChangeDetection({ eventCoalescing: true }),
    provideRouter(routes),
    provideClientHydration(withEventReplay()),
    provideAnimations(),
    provideHttpClient(withInterceptors([authInterceptor, errorInterceptor]))]
    // importProvidersFrom(HttpClientInMemoryWebApiModule.forRoot(MockBackendService, { delay: 300 }))]
};
