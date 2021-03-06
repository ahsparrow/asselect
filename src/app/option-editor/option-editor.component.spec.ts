import { ComponentFixture, TestBed } from '@angular/core/testing';

import { OptionEditorComponent } from './option-editor.component';

describe('OptionEditorComponent', () => {
  let component: OptionEditorComponent;
  let fixture: ComponentFixture<OptionEditorComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ OptionEditorComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(OptionEditorComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
