use winit::event::*;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct FractalInfo {
    pos_x: f32,
    pos_y: f32,
    scale: f32,
    pad: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ScreenInfo {
    pub img_size_w: f32,
    pub img_size_h: f32,
    pad: f32,
    pad2: f32,
}

#[derive(Debug)]
pub struct InputController {
    pub fractal_info: FractalInfo,
    pub screen_info: ScreenInfo,
    is_mouse_clicked: bool,
}

impl InputController {
    pub fn new_empty() -> InputController {
        InputController {
            fractal_info: FractalInfo {
                pos_x: 0.0,
                pos_y: 0.0,
                scale: 1.0,
                pad: 0.0,
            },
            screen_info: ScreenInfo {
                img_size_w: 100.0,
                img_size_h: 100.0,
                pad: 0.0,
                pad2: 0.0,
            },
            is_mouse_clicked: false,
        }
    }

    pub fn process_mouse_events(&mut self, event: &DeviceEvent) -> bool {
        match event {
            DeviceEvent::MouseMotion { delta } if self.is_mouse_clicked == true => {
                self.fractal_info.pos_x -=
                    (6. / self.fractal_info.scale) * (delta.0 as f32 / self.screen_info.img_size_w);
                self.fractal_info.pos_y -=
                    (6. / self.fractal_info.scale) * (delta.1 as f32 / self.screen_info.img_size_h);
                true
            }
            _ => false,
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::MouseWheel { delta, .. } => match delta {
                MouseScrollDelta::LineDelta(_, y) => {
                    self.fractal_info.scale *= if *y >= 0.0 { 1.1 } else { 0.9 };
                    true
                }
                MouseScrollDelta::PixelDelta(lpos) => {
                    self.fractal_info.scale *= if lpos.y >= 0.0 { 1.1 } else { 0.9 };
                    true
                }
            },
            WindowEvent::MouseInput { button, state, .. } => match button {
                MouseButton::Left => match state {
                    ElementState::Pressed => {
                        self.is_mouse_clicked = true;
                        true
                    }
                    ElementState::Released => {
                        self.is_mouse_clicked = false;
                        true
                    }
                },
                _ => false,
            },
            _ => false,
        }
    }
}
