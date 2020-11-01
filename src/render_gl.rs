use gl;
use std;

pub mod shader {
    use std::ptr::null;
    use std::ffi::CString;

    pub struct Kind {
        gl_value: gl::types::GLuint
    }

    pub const VERTEX_SHADER: Kind = Kind { gl_value: gl::VERTEX_SHADER };
    pub const FRAGMENT_SHADER: Kind = Kind { gl_value: gl::FRAGMENT_SHADER };

    pub struct Shader {
        id: gl::types::GLuint,
    }

    pub fn new(kind: Kind) -> ShaderBuilder {
        ShaderBuilder {
            kind: Some(kind.gl_value),
            code: None
        }
    }

    // impl Drop for Shader {
    //     fn drop(&mut self) {
    //         unsafe {
    //             gl::DeleteShader(self.id);
    //         }
    //     }
    // }

    pub struct ShaderBuilder {
        kind: Option<gl::types::GLenum>,
        code: Option<&'static str>
    }

    impl ShaderBuilder {

        pub fn from_string(&mut self, value: &'static str) -> &mut ShaderBuilder {
            self.code = Some(value);
            self
        }

        pub fn build(&mut self) -> Result<Shader, String> {

            let id = unsafe { gl::CreateShader(self.kind.unwrap()) };
            let c_str = std::ffi::CString::new(self.code.unwrap()).unwrap();

            unsafe {
                gl::ShaderSource(id, 1, &c_str.as_ptr(), std::ptr::null());
                gl::CompileShader(id);
            }

            let mut success: gl::types::GLint = 1;
            unsafe {
                gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
            }

            if success == gl::FALSE.into() {

                let mut len: gl::types::GLint = 0;

                unsafe {
                    gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
                }

                let error = create_whitespace_cstring_with_len(len as usize);

                unsafe {
                    gl::GetShaderInfoLog(
                        id,
                        len,
                        std::ptr::null_mut(),
                        error.as_ptr() as *mut gl::types::GLchar
                    );
                }

                return Err(error.to_string_lossy().into_owned());
            }

            return Ok(Shader {
                id
            })
        }
    }

    fn create_whitespace_cstring_with_len(len: usize) -> CString {
        // allocate buffer of correct size
        let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
        // fill it with len spaces
        buffer.extend([b' '].iter().cycle().take(len));
        // convert buffer to CString
        unsafe { CString::from_vec_unchecked(buffer) }
    }
}
