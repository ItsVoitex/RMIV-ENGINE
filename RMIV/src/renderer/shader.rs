use std::ffi::CString;
use std::fs;


pub struct Shader {
    pub id:u32
}

impl  Shader{

    pub fn create() -> Shader
    {
        const VERTEX_SOURCE:&str = r#"
        #version 330 core
        layout (location = 0) in vec3 aPos;
        layout (location = 1) in vec2 aTexCoord;
        out vec2 texCoord;
        uniform vec3 offset;
        void main()
        {

            vec3 position = aPos + offset;
            position.x = position.x / 1920.0 * 2 - 1;
            position.y = 1 - (position.y/1080.0) * 2;
            gl_Position = vec4(position, 1.0);
            texCoord = aTexCoord;
        }
        "#;

        const FRAGMENT_SOURCE:&str = r#"
        #version 330 core
        out vec4 FragColor;
        in vec2 texCoord;
        uniform sampler2D texture1;
        void main()
        {
            FragColor = texture(texture1 ,texCoord);
        }
        "#;

        let vertex_shader_source = CString::new(VERTEX_SOURCE).unwrap();
        let fragment_shader_source = CString::new(FRAGMENT_SOURCE).unwrap();

        let vertex_shader;
        let fragment_shader;
        let shader_program;
        unsafe {
            vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex_shader, 1, &vertex_shader_source.as_ptr(), std::ptr::null());
            gl::CompileShader(vertex_shader);

            let mut success = 0;
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0
            {
                println!("failed to compile vertex shader");
            }

            fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment_shader, 1, &fragment_shader_source.as_ptr(), std::ptr::null());
            gl::CompileShader(fragment_shader);

            let mut success = 0;
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            let mut info_log:[i8;512] = [0;512];
            if success == 0
            {
                gl::GetShaderInfoLog(vertex_shader, 512,std::ptr::null_mut(),info_log.as_mut_ptr());

                println!("failed to compile fragment shader");
            }

            shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader); 
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);

            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
            if success == 0
            {
                println!("failed to link shaders");
            }

            gl::DeleteShader(fragment_shader);
            gl::DeleteShader(vertex_shader); 
            Shader {
                id:shader_program
            }
        }
    }
    pub fn new(vertex_path:&str,fragment_path:&str) -> Shader
    {
        let vertex_shader_source = CString::new(fs::read_to_string(vertex_path).unwrap()).unwrap();
        let fragment_shader_source = CString::new(fs::read_to_string(fragment_path).unwrap()).unwrap();

        let vertex_shader;
        let fragment_shader;
        let shader_program;
        unsafe {
            vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex_shader, 1, &vertex_shader_source.as_ptr(), std::ptr::null());
            gl::CompileShader(vertex_shader);

            let mut success = 0;
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0
            {
                println!("failed to compile vertex shader");
            }

            fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment_shader, 1, &fragment_shader_source.as_ptr(), std::ptr::null());
            gl::CompileShader(fragment_shader);

            let mut success = 0;
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            let mut info_log:[i8;512] = [0;512];
            if success == 0
            {
                gl::GetShaderInfoLog(vertex_shader, 512,std::ptr::null_mut(),info_log.as_mut_ptr());

                println!("failed to compile fragment shader");
            }

            shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader); 
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);

            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
            if success == 0
            {
                println!("failed to link shaders");
            }

            gl::DeleteShader(fragment_shader);
            gl::DeleteShader(vertex_shader); 
            Shader {
                id:shader_program
            }
        }
    }

    pub fn use_program(&self)
    {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}
/*impl Drop for Shader
{
    fn drop(&mut self)
    {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
    */