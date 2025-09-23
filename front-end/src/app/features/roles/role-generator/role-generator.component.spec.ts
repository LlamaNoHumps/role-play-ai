import { ComponentFixture, TestBed } from '@angular/core/testing';

import { RoleGeneratorComponent } from './role-generator.component';

describe('RoleGeneratorComponent', () => {
  let component: RoleGeneratorComponent;
  let fixture: ComponentFixture<RoleGeneratorComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [RoleGeneratorComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(RoleGeneratorComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
