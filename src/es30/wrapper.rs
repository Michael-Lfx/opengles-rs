use std;
use std::ffi::CStr;
use std::ffi::CString;
use std::mem;
use std::mem::size_of;
use std::ptr;
use std::slice;
use std::str::from_utf8;

use libc::c_char;

use super::data_struct::ProgramBinary;
use super::ffi;
use consts::*;
use enums::*;
use es20::wrapper::{Active, Error};
use types::*;

pub struct Wrapper {}

impl Wrapper {
    pub fn gl_read_buffer(&mut self, mode: ColorBufferMode) -> Result<(), Error> {
        unsafe {
            ffi::glReadBuffer(mode as GLenum);
        }
        Ok(())
    }

    pub fn gl_draw_buffers(&mut self, bufs: &[ColorBufferMode]) -> Result<(), Error> {
        unsafe {
            let n: GLsizei = bufs.len() as i32;
            ffi::glDrawBuffers(n, bufs.as_ptr() as *const GLenum);
        }
        Ok(())
    }

    pub fn gl_unmap_buffer(&mut self, target: BufferObjectTarget) -> Result<bool, Error> {
        unsafe {
            let result = ffi::glUnmapBuffer(target as GLenum);
            if result == GL_TRUE {
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }

    pub fn gl_copy_buffer_sub_data(
        &mut self,
        read_target: BufferObjectTarget,
        write_target: BufferObjectTarget,
        read_offset: GLintptr,
        write_offset: GLintptr,
        size: GLsizeiptr,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glCopyBufferSubData(
                read_target as GLenum,
                write_target as GLenum,
                read_offset,
                write_offset,
                size,
            );
        }
        Ok(())
    }

    //todo : *mut *mut GLvoid
    pub fn gl_get_buffer_pointerv<T>(
        &mut self,
        target: BufferObjectTarget,
        pname: BufferMapTarget,
        params: *mut *mut GLvoid,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glGetBufferPointerv(target as GLenum, pname as GLenum, params);
        }
        Ok(())
    }

    //todo : reference to program binary
    pub fn gl_map_buffer_range<'a, T>(
        &mut self,
        target: BufferObjectTarget,
        offset: GLintptr,
        length: GLsizeiptr,
        access: MappingBit,
    ) -> Result<&'a [T], Error> where T: std::fmt::Debug + Clone {
        unsafe {
            let ptr = ffi::glMapBufferRange(target as GLenum, offset, length, access as GLenum);

            let count = length as usize / std::mem::size_of::<T>();
            Ok(slice::from_raw_parts_mut(ptr as *mut T, count as usize))
        }
    }

    pub fn gl_flush_mapped_buffer_range(
        &mut self,
        target: BufferObjectTarget,
        offset: i32,
        length: i32,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glFlushMappedBufferRange(
                target as GLenum,
                offset as GLintptr,
                length as GLsizeiptr,
            );
        }
        Ok(())
    }

    // todo: target范围变小
    pub fn gl_bind_buffer_range(
        &mut self,
        target: BufferObjectTarget,
        index: u32,
        buffer: u32,
        offset: i32,
        size: i32,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glBindBufferRange(
                target as GLenum,
                index as GLuint,
                buffer as GLuint,
                offset as GLintptr,
                size as GLsizeiptr,
            );
        }
        Ok(())
    }

    // todo: target范围变小
    pub fn gl_bind_buffer_base(
        &mut self,
        target: BufferObjectTarget,
        index: u32,
        buffer: u32,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glBindBufferBase(target as GLenum, index as GLuint, buffer as GLuint);
        }
        Ok(())
    }

    // todo: [GLint]
    pub fn gl_clear_bufferiv(
        &mut self,
        buffer: GLenum,
        draw_buffer: i32,
        value: &[GLint],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glClearBufferiv(buffer, draw_buffer as GLint, value.as_ptr() as *const GLint);
        }
        Ok(())
    }

    pub fn gl_clear_bufferuiv(
        &mut self,
        buffer: GLenum,
        drawbuffer: i32,
        value: &[GLuint],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glClearBufferuiv(buffer, drawbuffer as i32, value.as_ptr() as *const GLuint);
        }
        Ok(())
    }

    pub fn gl_clear_bufferfv(
        &mut self,
        buffer: GLenum,
        drawbuffer: i32,
        value: &[GLfloat],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glClearBufferfv(buffer, drawbuffer as i32, value.as_ptr() as *const GLfloat);
        }
        Ok(())
    }

    pub fn gl_clear_bufferfi(
        &mut self,
        buffer: GLenum,
        drawbuffer: i32,
        depth: GLfloat,
        stencil: GLint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glClearBufferfi(buffer, drawbuffer as i32, depth, stencil);
        }
        Ok(())
    }

    pub fn gl_get_buffer_parameteri64v(
        &mut self,
        target: GLenum,
        pname: GLenum,
    ) -> Result<i64, Error> {
        unsafe {
            let mut params = 0 as GLint64;
            ffi::glGetBufferParameteri64v(target as GLenum, pname as GLenum, &mut params);
            Ok(params)
        }
    }

    pub fn gl_tex_image_3d(
        &mut self,
        target: TextureTarget,
        level: i32,
        internal_format: i32,
        width: i32,
        height: i32,
        depth: GLsizei,
        border: i32,
        format: PixelDataFormat,
        type_: GLenum,
        opt_data: Option<&[u8]>,
    ) -> Result<(), Error> {
        unsafe {
            let pdata = match opt_data {
                Some(data) => mem::transmute(data.as_ptr()),
                None => ptr::null(),
            };
            ffi::glTexImage3D(
                target as GLenum,
                level as GLint,
                internal_format as GLint,
                width as GLsizei,
                height as GLsizei,
                depth,
                border as GLint,
                format as GLenum,
                type_,
                pdata,
            );
        }
        Ok(())
    }

    pub fn gl_tex_sub_image_3d(
        &mut self,
        target: TextureTarget,
        level: GLint,
        x_offset: GLint,
        y_offset: GLint,
        z_offset: GLint,
        width: i32,
        height: i32,
        depth: GLsizei,
        format: PixelDataFormat,
        type_: GLenum,
        opt_data: Option<&[u8]>,
    ) -> Result<(), Error> {
        unsafe {
            let pdata = match opt_data {
                Some(data) => mem::transmute(data.as_ptr()),
                None => ptr::null(),
            };

            ffi::glTexSubImage3D(
                target as GLenum,
                level,
                x_offset,
                y_offset,
                z_offset,
                width as GLsizei,
                height as GLsizei,
                depth,
                format as GLenum,
                type_,
                pdata,
            );
        }
        Ok(())
    }

    pub fn gl_copy_tex_sub_image3d(
        &mut self,
        target: TextureTarget,
        level: GLint,
        x_offset: GLint,
        y_offset: GLint,
        z_offset: GLint,
        x: GLint,
        y: GLint,
        width: i32,
        height: i32,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glCopyTexSubImage3D(
                target as GLenum,
                level,
                x_offset,
                y_offset,
                z_offset,
                x,
                y,
                width as GLsizei,
                height as GLsizei,
            );
        }
        Ok(())
    }

    pub fn gl_compressed_tex_image3d<T>(
        &mut self,
        target: TextureTarget,
        level: GLint,
        internal_format: PixelDataFormat,
        width: i32,
        height: i32,
        depth: GLsizei,
        border: GLint,
        imageSize: GLsizei,
        data: &[T],
    ) -> Result<(), Error> where T: std::fmt::Debug + Clone {
        unsafe {
            ffi::glCompressedTexImage3D(
                target as GLenum,
                level,
                internal_format as GLenum,
                width as GLsizei,
                height as GLsizei,
                depth,
                border,
                imageSize,
                data.as_ptr() as *const GLvoid,
            );
            ;
        }
        Ok(())
    }

    pub fn gl_compressed_tex_sub_image3d<T>(
        &mut self,
        target: TextureTarget,
        level: GLint,
        x_offset: GLint,
        y_offset: GLint,
        z_offset: GLint,
        width: i32,
        height: i32,
        depth: GLsizei,
        format: PixelDataFormat,
        image_size: GLsizei,
        data: &[T],
    ) -> Result<(), Error> where T: std::fmt::Debug + Clone {
        unsafe {
            ffi::glCompressedTexSubImage3D(
                target as GLenum,
                level,
                x_offset,
                y_offset,
                z_offset,
                width as GLsizei,
                height as GLsizei,
                depth,
                format as GLenum,
                image_size,
                data.as_ptr() as *const GLvoid,
            );
        }
        Ok(())
    }

    pub fn gl_gen_queries(&mut self, size: i32) -> Result<Vec<GLuint>, Error> {
        unsafe {
            let mut ids: Vec<GLuint> = Vec::with_capacity(size as usize);
            ffi::glGenQueries(size as GLsizei, ids.as_ptr() as *mut GLuint);
            Ok(ids)
        }
    }

    pub fn gl_delete_queries(&mut self, ids: &mut [GLuint]) -> Result<(), Error> {
        unsafe {
            let n: GLsizei = ids.len() as i32;
            ffi::glDeleteQueries(n, ids.as_ptr() as *mut GLuint);
        }
        Ok(())
    }

    pub fn gl_is_query(&mut self, id: u32) -> Result<GLboolean, Error> {
        unsafe {
            let result = ffi::glIsQuery(id as GLuint);
            Ok(result)
        }
    }

    pub fn gl_begin_query(&mut self, target: GLenum, id: u32) -> Result<(), Error> {
        unsafe {
            ffi::glBeginQuery(target as GLenum, id as GLuint);
        }
        Ok(())
    }

    pub fn gl_end_query(&mut self, target: GLenum) -> Result<(), Error> {
        unsafe {
            ffi::glEndQuery(target);
        }
        Ok(())
    }

    pub fn gl_get_queryiv(
        &mut self,
        target: GLenum,
        pname: GLenum,
        params: &mut [GLint],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glGetQueryiv(
                target as GLenum,
                pname as GLenum,
                params.as_ptr() as *mut GLint,
            );
        }
        Ok(())
    }

    pub fn gl_get_query_objectuiv(
        &mut self,
        id: u32,
        pname: GLenum,
        params: &mut [GLuint],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glGetQueryObjectuiv(
                id as GLuint,
                pname as GLenum,
                params.as_mut_ptr() as *mut GLuint,
            );
        }
        Ok(())
    }

    pub fn gl_uniform_matrix2x3fv(
        &mut self,
        location: i32,
        count: i32,
        transpose: bool,
        value: &[GLfloat],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glUniformMatrix2x3fv(
                location as GLint,
                count as GLsizei,
                transpose as GLboolean,
                value.as_ptr() as *const GLfloat,
            );
        }
        Ok(())
    }

    pub fn gl_uniform_matrix3x2fv(
        &mut self,
        location: i32,
        count: i32,
        transpose: bool,
        value: &[GLfloat],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glUniformMatrix3x2fv(
                location as GLint,
                count as GLsizei,
                transpose as GLboolean,
                value.as_ptr() as *const GLfloat,
            );
        }
        Ok(())
    }

    pub fn gl_uniform_matrix2x4fv(
        &mut self,
        location: i32,
        count: i32,
        transpose: bool,
        value: &[GLfloat],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glUniformMatrix2x4fv(
                location as GLint,
                count as GLsizei,
                transpose as GLboolean,
                value.as_ptr() as *const GLfloat,
            );
        }
        Ok(())
    }

    pub fn gl_uniform_matrix4x2fv(
        &mut self,
        location: i32,
        count: i32,
        transpose: bool,
        value: &[GLfloat],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glUniformMatrix4x2fv(
                location as GLint,
                count as GLsizei,
                transpose as GLboolean,
                value.as_ptr() as *const GLfloat,
            );
        }
        Ok(())
    }

    pub fn gl_uniform_matrix3x4fv(
        &mut self,
        location: i32,
        count: i32,
        transpose: bool,
        value: &[GLfloat],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glUniformMatrix3x4fv(
                location as GLint,
                count as GLsizei,
                transpose as GLboolean,
                value.as_ptr() as *const GLfloat,
            );
        }
        Ok(())
    }

    pub fn gl_uniform_matrix4x3fv(
        &mut self,
        location: i32,
        count: i32,
        transpose: bool,
        value: &[GLfloat],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glUniformMatrix4x3fv(
                location as GLint,
                count as GLsizei,
                transpose as GLboolean,
                value.as_ptr() as *const GLfloat,
            );
        }
        Ok(())
    }

    pub fn gl_renderbuffer_storage_multisample(
        &mut self,
        target: GLenum,
        samples: GLsizei,
        internal_format: GLenum,
        width: i32,
        height: i32,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glRenderbufferStorageMultisample(
                target as GLenum,
                samples,
                internal_format as GLenum,
                width as GLsizei,
                height as GLsizei,
            );
        }
        Ok(())
    }

    pub fn gl_bind_vertex_array(&mut self, array: GLuint) -> Result<(), Error> {
        unsafe {
            ffi::glBindVertexArray(array);
        }
        Ok(())
    }

    pub fn gl_delete_vertex_arrays(&mut self, arrays: &[u32]) -> Result<(), Error> {
        unsafe {
            if arrays.len() > 0 {
                let n = arrays.len() as i32;
                ffi::glDeleteVertexArrays(n, arrays.as_ptr() as *const GLuint);
            }
        }
        Ok(())
    }

    pub fn gl_gen_vertex_arrays(&mut self, count: GLsizei) -> Result<Vec<GLuint>, Error> {
        unsafe {
            let mut vec: Vec<GLuint> = Vec::with_capacity(count as usize);
            ffi::glGenVertexArrays(count as GLsizei, vec.as_mut_ptr());
            vec.set_len(count as usize);
            Ok(vec)
        }
    }

    pub fn gl_is_vertex_array(&mut self, array: GLuint) -> Result<GLboolean, Error> {
        unsafe {
            let result = ffi::glIsVertexArray(array);
            if result == GL_TRUE {
                Ok(GL_TRUE)
            } else {
                Ok(GL_FALSE)
            }
        }
    }

    pub fn gl_get_integeri_v(&mut self, target: GLenum, index: GLuint) -> Result<GLint, Error> {
        unsafe {
            let mut value: GLint = 0;
            ffi::glGetIntegeri_v(target as GLenum, index, &mut value);
            Ok(value)
        }
    }

    /// Transform Feedback

    pub fn gl_end_transform_feedback(&mut self) -> Result<(), Error> {
        unsafe {
            ffi::glEndTransformFeedback();
        }
        Ok(())
    }

    //todo: count这么写是否对？
    pub fn gl_transform_feedback_varyings(
        &mut self,
        program: u32,
        count: i32,
        varyings: &Vec<String>,
        buffer_mode: TransformFeedbackMode,
    ) -> Result<(), Error> {
        unsafe {
            let mut names: Vec<CString> = Vec::with_capacity(count as usize);
            let mut index = 0 as usize;
            while index < count as usize {
                names.push(CString::new(&(varyings[index])[..]).unwrap());
                index = index + 1;
            }
            index = 0;
            let ptr = names[index].as_ptr();
            let mut names_ptr: Vec<usize> = Vec::with_capacity(count as usize);

            while index < count as usize {
                names_ptr.push(names[index].as_ptr() as usize);
                index = index + 1;
            }
            ffi::glTransformFeedbackVaryings(
                program as GLuint,
                count as GLsizei,
                names_ptr.as_ptr() as *const *const GLchar,
                buffer_mode as GLenum,
            );
        }
        Ok(())
    }

    pub fn gl_get_transform_feedback_varying(
        &mut self,
        program: u32,
        index: u32,
        buffer_size: GLsizei,
    ) -> Result<Active, Error> {
        unsafe {
            let mut length: GLsizei = 0;
            let mut size: i32 = 0;
            let mut type_: GLenum = GL_NONE;
            let mut name = String::with_capacity(256);

            ffi::glGetTransformFeedbackVarying(
                program as GLuint,
                index,
                buffer_size,
                &mut length,
                &mut size,
                &mut type_,
                name.as_mut_vec().as_mut_ptr() as *mut GLchar,
            );

            if length > 0 {
                name.as_mut_vec().set_len(length as usize);
                name.truncate(length as usize);

                Ok(Active {
                    name,
                    size,
                    type_: DataType::BOOL,
                    length,
                })
            } else {
                Err(Error {})
            }
        }
    }

    pub fn gl_bind_transform_feedback(
        &mut self,
        target: TransformFeedbackObjectTarget,
        id: u32,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glBindTransformFeedback(target as GLenum, id as GLuint);
        }
        Ok(())
    }

    pub fn gl_delete_transform_feedbacks(&mut self, ids: &[GLuint]) -> Result<(), Error> {
        unsafe {
            let n = ids.len() as i32;
            ffi::glDeleteTransformFeedbacks(n, ids.as_ptr() as *const GLuint);
        }
        Ok(())
    }

    pub fn gl_gen_transform_feedbacks(&mut self, size: i32) -> Result<Vec<GLuint>, Error> {
        unsafe {
            let mut ids: Vec<GLuint> = Vec::with_capacity(size as usize);
            ffi::glGenTransformFeedbacks(size as GLsizei, ids.as_mut_ptr() as *mut GLuint);
            Ok(ids)
        }
    }

    pub fn gl_is_transform_feedback(&mut self, id: u32) -> Result<bool, Error> {
        unsafe {
            let result = ffi::glIsTransformFeedback(id as GLuint);
            if result == GL_TRUE {
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }

    pub fn gl_pause_transform_feedback(&mut self) -> Result<(), Error> {
        unsafe {
            ffi::glPauseTransformFeedback();
        }
        Ok(())
    }

    pub fn gl_resume_transform_feedback(&mut self) -> Result<(), Error> {
        unsafe {
            ffi::glResumeTransformFeedback();
        }
        Ok(())
    }

    pub fn gl_vertex_attrib_ipointer<T>(
        &mut self,
        index: u32,
        size: GLint,
        type_: GLenum,
        stride: GLsizei,
        pointer: &[T],
    ) -> Result<(), Error> where T: std::fmt::Debug + Clone {
        unsafe {
            ffi::glVertexAttribIPointer(
                index,
                size as GLsizei,
                type_,
                stride,
                pointer.as_ptr() as *const GLvoid,
            );
        }
        Ok(())
    }

    pub fn gl_get_vertex_attrib_iiv(&mut self, index: u32, pname: GLenum) -> Result<GLint, Error> {
        unsafe {
            let mut params: GLint = 0;
            ffi::glGetVertexAttribIiv(index, pname as GLenum, &mut params);
            Ok(params)
        }
    }

    pub fn gl_get_vertex_attrib_iuiv(
        &mut self,
        index: u32,
        pname: GLenum,
    ) -> Result<GLuint, Error> {
        unsafe {
            let mut params: GLuint = 0;
            ffi::glGetVertexAttribIuiv(index, pname as GLenum, &mut params);
            Ok(params)
        }
    }

    pub fn gl_vertex_attrib_i4i(
        &mut self,
        index: u32,
        x: GLint,
        y: GLint,
        z: GLint,
        w: GLint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glVertexAttribI4i(index, x, y, z, w);
        }
        Ok(())
    }

    pub fn gl_vertex_attrib_i4ui(
        &mut self,
        index: u32,
        x: GLuint,
        y: GLuint,
        z: GLuint,
        w: GLuint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glVertexAttribI4ui(index, x, y, z, w);
        }
        Ok(())
    }

    pub fn gl_vertex_attrib_i4iv(&mut self, index: u32, v: &[GLint]) -> Result<(), Error> {
        unsafe {
            ffi::glVertexAttribI4iv(index, v.as_ptr() as *const GLint);
        }
        Ok(())
    }

    pub fn gl_vertex_attrib_i4uiv(&mut self, index: u32, v: &[GLint]) -> Result<(), Error> {
        unsafe {
            ffi::glVertexAttribI4uiv(index, v.as_ptr() as *const GLuint);
        }
        Ok(())
    }

    pub fn gl_get_uniformuiv(&mut self, program: u32, location: i32) -> Result<GLuint, Error> {
        unsafe {
            let mut value: GLuint = 0;
            ffi::glGetUniformuiv(program as GLuint, location as GLint, &mut value);
            Ok(value)
        }
    }

    pub fn gl_get_frag_data_location(&mut self, program: u32, name: &str) -> Result<GLint, Error> {
        unsafe {
            let c_str = CString::new(name).unwrap();
            let location = ffi::glGetFragDataLocation(program as GLuint, c_str.as_ptr() as *const GLchar);
            Ok(location)
        }
    }

    pub fn gl_uniform1ui(&mut self, location: i32, v0: GLuint) -> Result<(), Error> {
        unsafe {
            ffi::glUniform1ui(location as GLint, v0);
        }
        Ok(())
    }

    pub fn gl_uniform2ui(&mut self, location: i32, v0: u32, v1: GLuint) -> Result<(), Error> {
        unsafe {
            ffi::glUniform2ui(location as GLint, v0 as GLuint, v1);
        }
        Ok(())
    }

    pub fn gl_uniform3ui(
        &mut self,
        location: i32,
        v0: u32,
        v1: GLuint,
        v2: GLuint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glUniform3ui(location as GLint, v0 as GLuint, v1, v2);
        }
        Ok(())
    }

    pub fn gl_uniform4ui(
        &mut self,
        location: i32,
        v0: u32,
        v1: GLuint,
        v2: GLuint,
        v3: GLuint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glUniform4ui(location as GLint, v0 as GLuint, v1, v2, v3);
        }
        Ok(())
    }

    pub fn gl_uniform1uiv(
        &mut self,
        location: i32,
        count: i32,
        value: &[GLuint],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glUniform1uiv(
                location as GLint,
                count as GLsizei,
                value.as_ptr() as *const GLuint,
            );
        }
        Ok(())
    }

    pub fn gl_uniform2uiv(
        &mut self,
        location: i32,
        count: i32,
        value: &[GLuint],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glUniform2uiv(
                location as GLint,
                count as GLsizei,
                value.as_ptr() as *const GLuint,
            );
        }
        Ok(())
    }

    pub fn gl_uniform3uiv(
        &mut self,
        location: i32,
        count: i32,
        value: &[GLuint],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glUniform3uiv(
                location as GLint,
                count as GLsizei,
                value.as_ptr() as *const GLuint,
            );
        }
        Ok(())
    }

    pub fn gl_uniform4uiv(
        &mut self,
        location: i32,
        count: i32,
        value: &[GLuint],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glUniform4uiv(
                location as GLint,
                count as GLsizei,
                value.as_ptr() as *const GLuint,
            );
        }
        Ok(())
    }

    pub fn gl_get_stringi(&mut self, name: GLenum, index: GLuint) -> Result<String, Error> {
        unsafe {
            let c_str = ffi::glGetStringi(name, index);
            if !c_str.is_null() {
                match from_utf8(CStr::from_ptr(c_str as *const c_char).to_bytes()) {
                    Ok(s) => Ok(s.to_string()),
                    Err(_) => Err(Error {}),
                }
            } else {
                Err(Error {})
            }
        }
    }

    //todo:
    pub fn gl_get_uniform_indices(
        &mut self,
        program: u32,
        uniform_count: i32,
        uniform_names: &Vec<String>,
    ) -> Result<Vec<GLuint>, Error> {
        unsafe {
            let mut names: Vec<CString> = Vec::with_capacity(uniform_count as usize);
            let mut index = 0 as usize;
            while index < uniform_count as usize {
                names.push(CString::new(&(uniform_names[index])[..]).unwrap());
                index = index + 1;
            }
            index = 0;
            let ptr = names[index].as_ptr();
            let mut names_ptr: Vec<usize> = Vec::with_capacity(uniform_count as usize);

            while index < uniform_count as usize {
                names_ptr.push(names[index].as_ptr() as usize);
                index = index + 1;
            }

            let mut uniform_indices: Vec<GLuint> = Vec::with_capacity(uniform_count as usize);

            ffi::glGetUniformIndices(
                program as GLuint,
                uniform_count as GLsizei,
                names_ptr.as_ptr() as *const *const GLchar,
                uniform_indices.as_ptr() as *mut GLuint,
            );

            Ok(uniform_indices)
        }
    }

    pub fn gl_get_active_uniformsiv(
        &mut self,
        program: u32,
        uniform_count: i32,
        uniform_indices: &[GLuint],
        pname: GLenum,
        params: &mut [GLint],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glGetActiveUniformsiv(
                program as GLuint,
                uniform_count as GLsizei,
                uniform_indices.as_ptr() as *const GLuint,
                pname as GLenum,
                params.as_mut_ptr() as *mut GLint,
            );
        }
        Ok(())
    }

    pub fn gl_get_uniform_block_index(
        &mut self,
        program: u32,
        uniform_block_name: &str,
    ) -> Result<GLuint, Error> {
        unsafe {
            let c_str = CString::new(uniform_block_name).unwrap();
            let index = ffi::glGetUniformBlockIndex(program as GLuint, c_str.as_ptr() as *const GLchar);
            Ok(index)
        }
    }

    pub fn gl_get_active_uniform_blockiv(
        &mut self,
        program: u32,
        uniform_block_index: GLuint,
        pname: GLenum,
    ) -> Result<GLint, Error> {
        unsafe {
            let mut value = 0 as GLint;
            ffi::glGetActiveUniformBlockiv(
                program as GLuint,
                uniform_block_index,
                pname as GLenum,
                &mut value,
            );
            Ok(value)
        }
    }

    pub fn gl_uniform_block_binding(
        &mut self,
        program: u32,
        uniform_block_index: GLuint,
        uniform_block_binding: GLuint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glUniformBlockBinding(
                program as GLuint,
                uniform_block_index,
                uniform_block_binding,
            );
        }
        Ok(())
    }

    pub fn gl_draw_range_elements<T>(
        &mut self,
        mode: BeginMode,
        start: GLuint,
        end: GLuint,
        count: i32,
        type_: GLenum,
        indices: &[T],
    ) -> Result<(), Error> where T: std::fmt::Debug + Clone {
        unsafe {
            ffi::glDrawRangeElements(
                mode as GLenum,
                start,
                end,
                count as GLsizei,
                type_,
                indices.as_ptr() as *const GLvoid,
            );
        }
        Ok(())
    }

    pub fn gl_draw_arrays_instanced(
        &mut self,
        mode: BeginMode,
        first: GLint,
        count: i32,
        instance_count: i32,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glDrawArraysInstanced(
                mode as GLenum,
                first,
                count as GLsizei,
                instance_count as GLsizei,
            );
        }
        Ok(())
    }

    pub fn gl_draw_elements_instanced<T>(
        &mut self,
        mode: BeginMode,
        count: i32,
        type_: GLenum,
        indices: &[T],
        instance_count: i32,
    ) -> Result<(), Error>  where T: std::fmt::Debug + Clone {
        unsafe {
            ffi::glDrawElementsInstanced(
                mode as GLenum,
                count as GLsizei,
                type_,
                indices.as_ptr() as *const GLvoid,
                instance_count as GLsizei,
            );
        }
        Ok(())
    }

    pub fn gl_fence_sync(&mut self, condition: GLenum, flags: GLbitfield) -> Result<GLsync, Error> {
        unsafe {
            let sync = ffi::glFenceSync(condition, flags);
            Ok(sync)
        }
    }

    pub fn gl_is_sync(&mut self, sync: GLsync) -> Result<bool, Error> {
        unsafe {
            let result = ffi::glIsSync(sync);
            if result == GL_TRUE {
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }

    pub fn gl_delete_sync(&mut self, sync: GLsync) -> Result<(), Error> {
        unsafe {
            ffi::glDeleteSync(sync);
        }
        Ok(())
    }

    pub fn gl_client_wait_sync(
        &mut self,
        sync: GLsync,
        flags: GLbitfield,
        timeout: GLuint64,
    ) -> Result<GLenum, Error> {
        unsafe {
            let result = ffi::glClientWaitSync(sync, flags, timeout);
            Ok(result)
        }
    }

    pub fn gl_wait_sync(
        &mut self,
        sync: GLsync,
        flags: GLbitfield,
        timeout: GLuint64,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glWaitSync(sync, flags, timeout);
        }
        Ok(())
    }

    pub fn gl_get_integer64v(&mut self, pname: GLenum) -> Result<GLint64, Error> {
        unsafe {
            let mut value = 0 as GLint64;
            ffi::glGetInteger64v(pname as GLenum, &mut value);
            Ok(value)
        }
    }

    //todo: 返回两个，不封装结构体，不做处理。
    pub fn gl_get_synciv(
        &mut self,
        sync: GLsync,
        pname: GLenum,
        buffer_size: GLsizei,
        length: GLsizei,
    ) -> Result<Vec<GLint>, Error> {
        unsafe {
            let mut values: Vec<GLint> = Vec::with_capacity(buffer_size as usize);
            ffi::glGetSynciv(
                sync,
                pname as GLenum,
                buffer_size,
                length as *mut GLsizei,
                values.as_mut_ptr() as *mut GLint,
            );
            Ok(values)
        }
    }

    pub fn gl_get_integer64i_v(&mut self, target: GLenum, index: GLuint) -> Result<GLint64, Error> {
        unsafe {
            let mut value = 0 as GLint64;
            ffi::glGetInteger64i_v(target as GLenum, index, &mut value);
            Ok(value)
        }
    }

    /// Samplers

    pub fn gl_gen_samplers(&mut self, count: GLsizei) -> Result<Vec<GLuint>, Error> {
        unsafe {
            let mut sampler: Vec<GLuint> = Vec::with_capacity(count as usize);
            ffi::glGenSamplers(count as GLsizei, sampler.as_mut_ptr() as *mut GLuint);
            Ok(sampler)
        }
    }

    pub fn gl_delete_samplers(&mut self, samplers: &[GLuint]) -> Result<(), Error> {
        unsafe {
            let count = samplers.len() as i32;
            ffi::glDeleteSamplers(count as GLsizei, samplers.as_ptr() as *const GLuint);
        }
        Ok(())
    }

    pub fn gl_is_sampler(&mut self, sampler: GLuint) -> Result<bool, Error> {
        unsafe {
            let result = ffi::glIsSampler(sampler);
            if result == GL_TRUE {
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }

    pub fn gl_bind_sampler(&mut self, unit: GLuint, sampler: GLuint) -> Result<(), Error> {
        unsafe {
            ffi::glBindSampler(unit, sampler);
        }
        Ok(())
    }

    pub fn gl_sampler_parameteri(
        &mut self,
        sampler: u32,
        pname: SamplerParameter,
        param: GLint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glSamplerParameteri(sampler as GLuint, pname as GLenum, param);
        }
        Ok(())
    }

    pub fn gl_sampler_parameteriv(
        &mut self,
        sampler: u32,
        pname: SamplerParameter,
        param: &[GLint],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glSamplerParameteriv(
                sampler as GLuint,
                pname as GLenum,
                param.as_ptr() as *const GLint,
            );
        }
        Ok(())
    }

    pub fn gl_sampler_parameterf(
        &mut self,
        sampler: u32,
        pname: SamplerParameter,
        param: GLfloat,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glSamplerParameterf(sampler as GLuint, pname as GLenum, param);
        }
        Ok(())
    }

    pub fn gl_sampler_parameterfv(
        &mut self,
        sampler: u32,
        pname: SamplerParameter,
        param: &[GLfloat],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glSamplerParameterfv(
                sampler as GLuint,
                pname as GLenum,
                param.as_ptr() as *const GLfloat,
            );
        }
        Ok(())
    }

    //todo : 我怀疑是返回一个，这里需要用slice?
    pub fn gl_get_sampler_parameteriv(
        &mut self,
        sampler: u32,
        pname: SamplerParameter,
        params: &mut [GLint],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glGetSamplerParameteriv(
                sampler as GLuint,
                pname as GLenum,
                params.as_mut_ptr() as *mut GLint,
            );
        }
        Ok(())
    }

    //todo : 我怀疑是返回一个，这里需要用slice?
    pub fn gl_get_sampler_parameterfv(
        &mut self,
        sampler: u32,
        pname: SamplerParameter,
        params: &mut [GLfloat],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glGetSamplerParameterfv(
                sampler as GLuint,
                pname as GLenum,
                params.as_mut_ptr() as *mut GLfloat,
            );
        }
        Ok(())
    }

    pub fn gl_vertex_attrib_divisor(&mut self, index: u32, divisor: u32) -> Result<(), Error> {
        unsafe {
            ffi::glVertexAttribDivisor(index, divisor as GLuint);
        }
        Ok(())
    }

    /// Shaders

    //todo:
    pub fn gl_get_program_binary(
        &mut self,
        program: u32,
        buffer_size: GLsizei,
    ) -> Result<ProgramBinary, Error> {
        unsafe {
            let mut length = 0 as GLsizei;
            let mut binary_format = GL_NONE as GLenum;
            let mut binary: Vec<u8> = Vec::with_capacity(buffer_size as usize);

            ffi::glGetProgramBinary(
                program as GLuint,
                buffer_size,
                &mut length as *mut GLsizei,
                &mut binary_format as *mut GLenum,
                binary.as_mut_ptr() as *mut GLvoid,
            );

            if length == 0 {
                Err(Error {})
            } else {
                Ok(ProgramBinary {
                    length,
                    binary_format,
                    binary,
                })
            }
        }
    }

    pub fn gl_program_binary(
        &mut self,
        program: u32,
        binary_format: GLenum,
        binary: &[u8],
        length: GLsizei,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramBinary(
                program as GLuint,
                binary_format,
                binary.as_ptr() as *const GLvoid,
                length,
            );
        }
        Ok(())
    }

    pub fn gl_program_parameteri(
        &mut self,
        program: u32,
        pname: GLenum,
        value: GLint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramParameteri(program as GLuint, pname as GLenum, value);
        }
        Ok(())
    }

    /// Frame Buffers

    pub fn gl_invalidate_framebuffer(
        &mut self,
        target: FrameBufferTarget,
        num_attachments: GLsizei,
        attachments: &[AttachmentTarget],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glInvalidateFramebuffer(
                target as GLenum,
                num_attachments,
                attachments.as_ptr() as *const GLenum,
            );
        }
        Ok(())
    }

    pub fn gl_invalidate_sub_framebuffer(
        &mut self,
        target: FrameBufferTarget,
        num_attachments: GLsizei,
        attachments: &[AttachmentTarget],
        x: GLint,
        y: GLint,
        width: i32,
        height: i32,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glInvalidateSubFramebuffer(
                target as GLenum,
                num_attachments,
                attachments.as_ptr() as *const GLenum,
                x,
                y,
                width as GLsizei,
                height as GLsizei,
            );
        }
        Ok(())
    }

    pub fn gl_blit_framebuffer(
        &mut self,
        src_X0: GLint,
        src_Y0: GLint,
        src_X1: GLint,
        src_Y1: GLint,
        dst_X0: GLint,
        dst_Y0: GLint,
        dst_X1: GLint,
        dst_Y1: GLint,
        mask: BufferMask,
        filter: FilterMode,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glBlitFramebuffer(
                src_X0,
                src_Y0,
                src_X1,
                src_Y1,
                dst_X0,
                dst_Y0,
                dst_X1,
                dst_Y1,
                mask as GLenum,
                filter as GLenum,
            );
        }
        Ok(())
    }

    pub fn gl_framebuffer_texture_layer(
        &mut self,
        target: FramebufferTarget,
        attachment: AttachmentTarget,
        texture: GLuint,
        level: GLint,
        layer: GLint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glFramebufferTextureLayer(
                target as GLenum,
                attachment as GLenum,
                texture,
                level,
                layer,
            );
        }
        Ok(())
    }

    pub fn gl_tex_storage2d(
        &mut self,
        target: TextureTarget,
        levels: GLsizei,
        internal_format: TextureTarget,
        width: i32,
        height: i32,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glTexStorage2D(
                target as GLenum,
                levels,
                internal_format as GLenum,
                width as GLsizei,
                height as GLsizei,
            );
        }
        Ok(())
    }

    pub fn gl_tex_storage3d(
        &mut self,
        target: TextureTarget,
        levels: GLsizei,
        internal_format: TextureTarget,
        width: i32,
        height: i32,
        depth: GLsizei,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glTexStorage3D(
                target as GLenum,
                levels,
                internal_format as GLenum,
                width as GLsizei,
                height as GLsizei,
                depth,
            );
        }
        Ok(())
    }

    pub fn gl_get_internal_formativ(
        &mut self,
        target: GLenum,
        internal_format: TextureTarget,
        pname: GLenum,
        buffer_size: GLsizei,
    ) -> Result<Vec<GLint>, Error> {
        unsafe {
            let mut params: Vec<GLint> = Vec::with_capacity(buffer_size as usize);
            ffi::glGetInternalformativ(
                target as GLenum,
                internal_format as GLenum,
                pname as GLenum,
                buffer_size,
                params.as_mut_ptr() as *mut GLint,
            );
            Ok(params)
        }
    }
}
