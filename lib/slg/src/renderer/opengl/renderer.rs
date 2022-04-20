use std::{sync::{Arc, RwLock, atomic::{AtomicBool, Ordering}}, num::NonZeroU32, mem::{size_of_val, size_of}, lazy::Lazy, ops::Deref, os::raw::c_char, intrinsics::transmute};
use glutin::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, dpi::LogicalSize, ContextBuilder, event::{Event, WindowEvent, ElementState}, GlRequest, Api, platform::run_return::EventLoopExtRunReturn};
use gl33::{global_loader::{load_global_gl, glCreateShader, glShaderSource, glCompileShader, glGetShaderiv, glGetShaderInfoLog, glCreateProgram, glAttachShader, glGetProgramiv, glLinkProgram, glGetProgramInfoLog, glDetachShader, glValidateProgram, glGenVertexArrays, glBindVertexArray, glGenBuffers, glBindBuffer, glBufferData, glVertexAttribPointer, glEnableVertexAttribArray, glClear, glEnable, glBlendFunc}, GL_COMPILE_STATUS, GL_LINK_STATUS, ProgramPropertyARB, GL_VALIDATE_STATUS, GL_ARRAY_BUFFER, GL_STATIC_DRAW, GL_FLOAT, ShaderType, GL_VERTEX_SHADER, GL_FRAGMENT_SHADER, GL_DEPTH_BUFFER_BIT, GL_COLOR_BUFFER_BIT, GL_BLEND, GL_ONE};
use crate::{Renderer, Threadly, RenderInstance, generics::KeyboardKey};
use super::{GlInstance, GlShader, GlUniform};

pub struct OpenGl {
    pub el: RwLock<EventLoop<()>>,
    pub insts: RwLock<Vec<Threadly<GlInstance>>>
}

impl Renderer for OpenGl {
    type Instance = GlInstance;
    type Shader = GlShader;
    type Uniform = GlUniform;

    #[inline(always)]
    fn new() -> Result<Self, String> {
        Ok(Self {
            el: RwLock::new(EventLoop::new()),
            insts: RwLock::new(Vec::with_capacity(1))
        })
    }

    fn create_instance (self: &Arc<Self>, title: impl Into<String>, width: impl Into<u32>, height: impl Into<u32>) -> Result<Threadly<Self::Instance>, String> {
        let title = title.into();

        let builder = WindowBuilder::new()
            .with_title(title.clone())
            .with_inner_size(LogicalSize::new(width.into(), height.into()));

        let context = ContextBuilder::new()
            .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
            .with_vsync(true)
            .build_windowed(builder, self.el.read().unwrap().deref())
            .map_err(|e| e.to_string())?;

        let current = unsafe { context
            .make_current()
            .map_err(|e| e.1.to_string())?
        };

        unsafe {
            load_global_gl(&|ptr| {
                let c_str = std::ffi::CStr::from_ptr(ptr as *const c_char);
                let r_str = c_str.to_str().unwrap();
                current.get_proc_address(r_str) as _
            });

            glEnable(GL_BLEND);
            glBlendFunc(GL_ONE, GL_ONE)
        }

        let instance = Arc::new(RwLock::new(GlInstance::new(title, current)?));
        let mut lock = self.insts.write().map_err(|e| e.to_string())?;
        lock.push(instance.clone());
        Ok(instance)
    }

    fn create_shader(code: &str) -> Result<Arc<Self::Shader>, String> {
        // PROGRAM
        let program = NonZeroU32::try_from(glCreateProgram())
            .map_err(|e| e.to_string())?;

        // SHADER
        glBindVertexArray(OpenGl::FILLER_MESH.get());
        let vertex = Self::create_shader(program, GL_VERTEX_SHADER, include_str!("glsl/vertex.vert"))?;
        let fragment = Self::create_shader(program, GL_FRAGMENT_SHADER, code)?;

        // LINKING
        glLinkProgram(program.into());
        Self::check_program(program.into(), GL_LINK_STATUS)?;

        unsafe {
            glValidateProgram(program.into());
            glDetachShader(program.into(), vertex.into());
            glDetachShader(program.into(), fragment.into());
        }

        Self::check_program(program.into(), GL_VALIDATE_STATUS)?;
        GlShader::new(program, vertex, fragment)
    }

    fn listen_events(&self) -> Result<(), String> {
        let mut el = self.el.write().map_err(|e| e.to_string())?;
        el.run_return(|ev, _, cf| {
            *cf = ControlFlow::Wait;
    
            match ev {
                Event::LoopDestroyed => return,
                Event::WindowEvent { window_id, event, .. } => {
                    let insts = self.insts.read().unwrap();
                    let window = if insts.len() == 1 { &insts[0] } else { insts.iter().find(|x| x.read().unwrap().context.window().id() == window_id).unwrap() };
                    let window = window.read().unwrap();

                    match event {
                        WindowEvent::Resized(physical_size) => window.context.resize(physical_size),
                        WindowEvent::KeyboardInput { input, .. } => if let Some(x) = input.virtual_keycode {
                            let key = KEYBOARD_MAPPING[x as usize];
                            window.keyboard[usize::from(key)].store(input.state == ElementState::Pressed, std::sync::atomic::Ordering::Relaxed)
                        },

                        WindowEvent::CloseRequested => *cf = ControlFlow::Exit,
                        _ => (),
                    }
                },

                Event::RedrawRequested(id) => {
                    let insts = self.insts.read().expect("Instance poisoned");
                    let window = if insts.len() == 1 { &insts[0] } else { insts.iter().find(|x| x.read().unwrap().context.window().id() == id).unwrap() };
                    let window = window.read().unwrap();

                    unsafe { glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT); }
                    window.get_children().into_iter()
                        .for_each(|x| {
                            match x.read().map(|x| x.render()) {
                                Err(x) => { eprintln!("{x}"); *cf = ControlFlow::Exit; },
                                Ok(_) => {}
                            }
                        });

                    match window.context.swap_buffers() {
                        Err(x) => { eprintln!("{}", x.to_string()); *cf = ControlFlow::Exit; },
                        Ok(_) => {}
                    }
                },

                Event::MainEventsCleared => {
                    let insts = self.insts.read().unwrap();
                    insts.iter().for_each(|x| {
                        let lock = x.read().unwrap();
                        let context = &lock.context;
                        if context.is_current() { context.window().request_redraw(); }
                    });
                }
                _ => (),
            }
        });

        Ok(())
    }
}

impl OpenGl {
    pub const FILLER_MESH : Lazy<NonZeroU32> = Lazy::new(|| {
        match Self::create_filler_mesh() {
            Err(e) => panic!("{e}"),
            Ok(x) => x
        }
    });

    const VERTICES : [f32; 12] = [
        -1.,  1.,
        -1., -1.,
        1.,  1.,
        1.,  1.,
        -1., -1.,
        1., -1.
    ];

    fn create_filler_mesh () -> Result<NonZeroU32, String> {
        let mut vao = 0;
        unsafe { glGenVertexArrays(1, &mut vao); }

        let vao = NonZeroU32::try_from(vao)
            .map_err(|e| e.to_string())?;

        glBindVertexArray(vao.into());

        let mut vbo = 0;
        unsafe { 
            glGenBuffers(1, &mut vbo);
            glBindBuffer(GL_ARRAY_BUFFER, vbo);

            glBufferData(
                GL_ARRAY_BUFFER, size_of_val(&Self::VERTICES) as isize, 
                Self::VERTICES.as_ptr().cast(), GL_STATIC_DRAW
            );

            glVertexAttribPointer(
                0, 2, GL_FLOAT, 
                0, (2 * size_of::<f32>()) as i32, 
                std::ptr::null()
            );

            glEnableVertexAttribArray(0);
        }

        Ok(vao)
    }

    fn create_shader (program: NonZeroU32, typ: ShaderType, code: &str) -> Result<NonZeroU32, String> {
        let shader = NonZeroU32::try_from(glCreateShader(typ))
            .map_err(|e| e.to_string())?;

        unsafe { glShaderSource(shader.into(), 1, &code.as_ptr().cast(), &(code.len() as i32)) }
        glCompileShader(shader.into());

        let mut successs = 0;
        unsafe { glGetShaderiv(shader.into(), GL_COMPILE_STATUS, &mut successs) }

        if successs == 0 {
            let mut buffer = Vec::<u8>::with_capacity(1024);
            let mut size = 0;
            unsafe { 
                glGetShaderInfoLog(shader.into(), buffer.capacity() as i32, &mut size, buffer.as_mut_ptr());
                buffer.set_len(size as usize);
                return Err(String::from_utf8_unchecked(buffer))
            }
        }

        glAttachShader(program.into(), shader.into());
        Ok(shader)
    }

    fn check_program (program: u32, check: ProgramPropertyARB) -> Result<(), String> {
        let mut successs = 0;
        unsafe { glGetProgramiv(program, check, &mut successs) }

        if successs == 0 {
            let mut buffer = Vec::<u8>::with_capacity(1024);
            let mut size = 0;
            unsafe { 
                glGetProgramInfoLog(program, buffer.capacity() as i32, &mut size, buffer.as_mut_ptr());
                buffer.set_len(size as usize);
                return Err(String::from_utf8_unchecked(buffer))
            }
        }

        Ok(())
    }
}

const KEYBOARD_MAPPING : [KeyboardKey; 161] = [
    KeyboardKey::ONE,
    KeyboardKey::TWO,
    KeyboardKey::THREE,
    KeyboardKey::FOUR,
    KeyboardKey::FIVE,
    KeyboardKey::SIX,
    KeyboardKey::SEVEN,
    KeyboardKey::EIGHT,
    KeyboardKey::NINE,
    KeyboardKey::ZERO,

    KeyboardKey::A,
    KeyboardKey::B,
    KeyboardKey::C,
    KeyboardKey::D,
    KeyboardKey::E,
    KeyboardKey::F,
    KeyboardKey::G,
    KeyboardKey::H,
    KeyboardKey::I,
    KeyboardKey::J,
    KeyboardKey::K,
    KeyboardKey::L,
    KeyboardKey::M,
    KeyboardKey::N,
    KeyboardKey::O,
    KeyboardKey::P,
    KeyboardKey::Q,
    KeyboardKey::R,
    KeyboardKey::S,
    KeyboardKey::T,
    KeyboardKey::U,
    KeyboardKey::V,
    KeyboardKey::W,
    KeyboardKey::X,
    KeyboardKey::Y,
    KeyboardKey::Z,

    KeyboardKey::ESCAPE,

    KeyboardKey::F1,
    KeyboardKey::F2,
    KeyboardKey::F3,
    KeyboardKey::F4,
    KeyboardKey::F5,
    KeyboardKey::F6,
    KeyboardKey::F7,
    KeyboardKey::F8,
    KeyboardKey::F9,
    KeyboardKey::F10,
    KeyboardKey::F12,
    KeyboardKey::F12,
    KeyboardKey::F13,
    KeyboardKey::F14,
    KeyboardKey::F15,
    KeyboardKey::F16,
    KeyboardKey::F17,
    KeyboardKey::F18,
    KeyboardKey::F19,
    KeyboardKey::F20,
    KeyboardKey::F21,
    KeyboardKey::F22,
    KeyboardKey::F23,
    KeyboardKey::F24,

    KeyboardKey::PRINT_SCREEN,
    KeyboardKey::SCROLL_LOCK,
    KeyboardKey::PAUSE,

    KeyboardKey::INSERT,
    KeyboardKey::HOME,
    KeyboardKey::DELETE,
    KeyboardKey::END,
    KeyboardKey::PAGE_DOWN,
    KeyboardKey::PAGE_UP,

    KeyboardKey::LEFT,
    KeyboardKey::UP,
    KeyboardKey::RIGHT,
    KeyboardKey::DOWN,

    KeyboardKey::BACKSPACE,
    KeyboardKey::ENTER,
    KeyboardKey::SPACE,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,

    KeyboardKey::NUM_LOCK,
    KeyboardKey::KP0,
    KeyboardKey::KP1,
    KeyboardKey::KP2,
    KeyboardKey::KP3,
    KeyboardKey::KP4,
    KeyboardKey::KP5,
    KeyboardKey::KP6,
    KeyboardKey::KP7,
    KeyboardKey::KP8,
    KeyboardKey::KP9,

    KeyboardKey::KP_ADD,
    KeyboardKey::KP_DIVIDE,
    KeyboardKey::KP_DECIMAL,
    KeyboardKey::KP_DECIMAL,
    KeyboardKey::KP_ENTER,
    KeyboardKey::KP_EQUAL,
    KeyboardKey::KP_MULTIPLY,
    KeyboardKey::KP_SUBTRACT,

    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::APOSTROPHE,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    
    KeyboardKey::BACKSLASH,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::COMMA,
    KeyboardKey::UNKNOWN,
    KeyboardKey::EQUAL,
    KeyboardKey::GRAVE_ACCENT,
    KeyboardKey::UNKNOWN,
    KeyboardKey::LEFT_ALT,
    KeyboardKey::LEFT_BRACKET,
    KeyboardKey::LEFT_CONTROL,
    KeyboardKey::LEFT_SHIFT,
    KeyboardKey::UNKNOWN,
    
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::MINUS,
    KeyboardKey::UNKNOWN,

    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,

    KeyboardKey::PERIOD,
    KeyboardKey::UNKNOWN, // PLAY-PAUSE
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,

    KeyboardKey::RIGHT_ALT,
    KeyboardKey::RIGHT_BRACKET,
    KeyboardKey::RIGHT_CONTROL,
    KeyboardKey::RIGHT_SHIFT,
    KeyboardKey::UNKNOWN,

    KeyboardKey::SEMICOLON,
    KeyboardKey::SLASH,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,

    KeyboardKey::TAB,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,

    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN,
    KeyboardKey::UNKNOWN
];