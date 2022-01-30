use std::{sync::{Arc, Mutex}, num::NonZeroU32, mem::{size_of_val, size_of}, lazy::Lazy};
use glutin::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, dpi::LogicalSize, ContextBuilder, event::{Event, WindowEvent}, GlRequest, Api, platform::run_return::EventLoopExtRunReturn};
use gl33::{global_loader::{load_global_gl, glCreateShader, glShaderSource, glCompileShader, glGetShaderiv, glGetShaderInfoLog, glCreateProgram, glAttachShader, glGetProgramiv, glLinkProgram, glGetProgramInfoLog, glDetachShader, glValidateProgram, glGenVertexArrays, glBindVertexArray, glGenBuffers, glBindBuffer, glBufferData, glVertexAttribPointer, glEnableVertexAttribArray, glClear, glEnable, glBlendFunc}, GL_COMPILE_STATUS, GL_LINK_STATUS, ProgramPropertyARB, GL_VALIDATE_STATUS, GL_ARRAY_BUFFER, GL_STATIC_DRAW, GL_FLOAT, ShaderType, GL_VERTEX_SHADER, GL_FRAGMENT_SHADER, GL_DEPTH_BUFFER_BIT, GL_COLOR_BUFFER_BIT, GL_BLEND, GL_ONE};
use crate::{Renderer, RenderInstance};
use super::{GlInstance, GlShader, GlUniform};

pub struct OpenGl {
    pub(super) el: Mutex<EventLoop<()>>,
    insts: Mutex<Vec<Arc<Mutex<GlInstance>>>>,
}

impl Renderer for OpenGl {
    type Error = String;
    type Instance = GlInstance;
    type Shader = GlShader;
    type Uniform = GlUniform;

    #[inline(always)]
    fn new() -> Result<Self, Self::Error> {
        Ok(Self {
            el: Mutex::new(EventLoop::new()),
            insts: Mutex::new(Vec::with_capacity(1))
        })
    }

    fn create_instance (self: &Arc<Self>, title: impl Into<String>, width: impl Into<u32>, height: impl Into<u32>) -> Result<Arc<Mutex<Self::Instance>>, Self::Error> {
        let title = title.into();

        let builder = WindowBuilder::new()
            .with_title(title.clone())
            .with_inner_size(LogicalSize::new(width.into(), height.into()));

        let context = ContextBuilder::new()
            .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
            .with_vsync(true)
            .build_windowed(builder, &*self.el.lock().unwrap())
            .map_err(|e| e.to_string())?;

        let current = unsafe { context
            .make_current()
            .map_err(|e| e.1.to_string())?
        };

        unsafe {
            load_global_gl(&|ptr| {
                let c_str = std::ffi::CStr::from_ptr(ptr as *const i8);
                let r_str = c_str.to_str().unwrap();
                current.get_proc_address(r_str) as _
            });

            glEnable(GL_BLEND);
            glBlendFunc(GL_ONE, GL_ONE)
        }

        let instance = Arc::new(Mutex::new(GlInstance::new(self.clone(), title, current)?));
        let mut lock = self.insts.lock().map_err(|e| e.to_string())?;
        lock.push(instance.clone());
        Ok(instance)
    }

    fn create_shader(&self, code: &str) -> Result<Arc<Self::Shader>, Self::Error> {
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

    fn listen_events(&self) -> Result<(), Self::Error> {
        let mut el = self.el.lock().map_err(|e| e.to_string())?;
        el.run_return(|ev, _, cf| {
            *cf = ControlFlow::Wait;
    
            match ev {
                Event::LoopDestroyed => return,
                Event::WindowEvent { window_id, event, .. } => {
                    let insts = self.insts.lock().unwrap();
                    let window = if insts.len() == 1 { &insts[0] } else { insts.iter().find(|x| x.lock().unwrap().context.window().id() == window_id).unwrap() };
                    let window = window.lock().unwrap();

                    match event {
                        WindowEvent::Resized(physical_size) => window.context.resize(physical_size),
                        WindowEvent::CloseRequested => *cf = ControlFlow::Exit,
                        _ => (),
                    }
                },

                Event::RedrawRequested(id) => {
                    let insts = self.insts.lock().expect("Instance poisoned");
                    let window = if insts.len() == 1 { &insts[0] } else { insts.iter().find(|x| x.lock().unwrap().context.window().id() == id).unwrap() };
                    let window = window.lock().unwrap();

                    unsafe { glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT); }

                    window.get_children().into_iter()
                        .for_each(|x| {
                            match x.lock().map(|x| x.render()) {
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
                    let insts = self.insts.lock().unwrap();
                    insts.iter().for_each(|x| {
                        let lock = x.lock().unwrap();
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