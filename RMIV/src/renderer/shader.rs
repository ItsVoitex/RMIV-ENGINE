use std::ffi::CString;
use std::fs;


pub struct Shader (pub u32);

impl  Shader{

    pub fn create_default() -> Shader
    {
        //may be possible to do these calculations on the cpu
        const VERTEX_SOURCE:&str = r#"
        #version 330 core
        layout (location = 0) in vec3 aPos;
        layout (location = 1) in vec2 aTexCoord;
        layout (location = 2) in vec3 aColor;
        out vec2 texCoord;
        out vec3 Color;
        uniform vec3 offset;
        uniform vec3 rotation;
        uniform vec3 scale;

        uniform mat4 model;
        uniform mat4 view;
        uniform mat4 proj;

        void main()
        {
            float angle = radians(rotation.x);
            float s = sin(angle);
            float c = cos(angle);
            vec3 position = aPos * scale;
            position.xy = mat2(c,-s,s,c) *position.xy;
            position = position + offset;
            //position.xy /= 970;
            
            gl_Position = proj * view * model vec4(position, 1.0);
            texCoord = aTexCoord;
            Color = aColor; 
        }
        "#;
        const FRAGMENT_SOURCE:&str = r#"
        #version 330 core
        out vec4 FragColor;
        in vec2 texCoord;
        in vec3 Color;
        uniform bool has_texture;
        uniform sampler2D texture1;
        void main()
        {
            if (has_texture)
            {
                FragColor = texture(texture1 ,texCoord) * vec4(Color,1.0); 
            }
            else
            {
                FragColor = vec4(Color,1.0); 
            }   
             
        }
        "#;

        let vertex_shader_source = CString::new(VERTEX_SOURCE).unwrap();
        let fragment_shader_source = CString::new(FRAGMENT_SOURCE).unwrap();

        let vertex_shader;
        let fragment_shader;
        let shader_program;
        unsafe {
            let mut info_log:[i8;512] = [0;512];
            vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex_shader, 1, &vertex_shader_source.as_ptr(), std::ptr::null());
            gl::CompileShader(vertex_shader);

            let mut success = 0;
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0
            {
                gl::GetShaderInfoLog(vertex_shader, 512,std::ptr::null_mut(),info_log.as_mut_ptr());
                let chars:Vec<char> = info_log.iter().map(|&byte| (byte as u8) as char).collect();
                for i in chars.iter()
                {
                    print!("{}",i);
                }
                println!("failed to compile vertex shader");
            }

            fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment_shader, 1, &fragment_shader_source.as_ptr(), std::ptr::null());
            gl::CompileShader(fragment_shader);

            let mut success = 0;
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            
            
            //need to add proper error reporting
            if success == 0
            {
                gl::GetShaderInfoLog(fragment_shader, 512,std::ptr::null_mut(),info_log.as_mut_ptr());
                let chars:Vec<char> = info_log.iter().map(|&byte| (byte as u8) as char).collect();
                for i in chars.iter()
                {
                    print!("{}",i);
                }
                println!("failed to compile fragment shader ");
            }

            shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader); 
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);

            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
            //need to add proper error reporting
            if success == 0
            {
                println!("failed to link shaders");
            }

            gl::DeleteShader(fragment_shader);
            gl::DeleteShader(vertex_shader); 
            Shader (
                shader_program
            )
        }
    }
    pub fn create(vertex_path:&str,fragment_path:&str) -> Shader
    {
        let vertex_shader_source = CString::new(fs::read_to_string(vertex_path).unwrap()).unwrap();
        let fragment_shader_source = CString::new(fs::read_to_string(fragment_path).unwrap()).unwrap();

        let vertex_shader;
        let fragment_shader;
        let shader_program;
        unsafe {
            let mut info_log:[i8;512] = [0;512];
            vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex_shader, 1, &vertex_shader_source.as_ptr(), std::ptr::null());
            gl::CompileShader(vertex_shader);

            let mut success = 0;
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0
            {
                gl::GetShaderInfoLog(vertex_shader, 512,std::ptr::null_mut(),info_log.as_mut_ptr());
                let chars:Vec<char> = info_log.iter().map(|&byte| (byte as u8) as char).collect();
                for i in chars.iter()
                {
                    print!("{}",i);
                }
                println!("failed to compile vertex shader");
            }

            fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment_shader, 1, &fragment_shader_source.as_ptr(), std::ptr::null());
            gl::CompileShader(fragment_shader);

            let mut success = 0;
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            
            
            //need to add proper error reporting
            if success == 0
            {
                gl::GetShaderInfoLog(fragment_shader, 512,std::ptr::null_mut(),info_log.as_mut_ptr());
                let chars:Vec<char> = info_log.iter().map(|&byte| (byte as u8) as char).collect();
                for i in chars.iter()
                {
                    print!("{}",i);
                }
                println!("failed to compile fragment shader ");
            }

            shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader); 
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);

            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
            //need to add proper error reporting
            if success == 0
            {
                println!("failed to link shaders");
            }

            gl::DeleteShader(fragment_shader);
            gl::DeleteShader(vertex_shader); 
            Shader (
                shader_program
            )
        
        }
    }

    pub fn use_program(&self)
    {
        unsafe {
            gl::UseProgram(self.0);
        }
    }
}
 
impl Drop for Shader
{
    fn drop(&mut self)
    {
        unsafe {
            gl::DeleteProgram(self.0);
        }
    }
}
