#![allow(
    non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
    missing_copy_implementations, unused_imports
)]

// -------------------------------------------------------------------------------------------------
// DEPENDENCIES
// -------------------------------------------------------------------------------------------------
extern crate libc;

// -------------------------------------------------------------------------------------------------
// DEPENDENCIES
// -------------------------------------------------------------------------------------------------

use std::ffi::CStr;
use std::ffi::CString;
use std::mem::size_of;
use std::str::from_utf8;

use libc::{c_char, c_int, c_short, c_uchar, c_uint, c_ushort, c_void};

// -------------------------------------------------------------------------------------------------
// LINKING
// -------------------------------------------------------------------------------------------------

#[cfg(target_os = "android")]
#[link(name = "GLESv2")]
#[link(name = "EGL")]
extern "C" {}

#[cfg(target_os = "ios")]
#[link(name = "OpenGLES")]
extern "C" {}

// -------------------------------------------------------------------------------------------------
// MODULES
// -------------------------------------------------------------------------------------------------

pub mod es20;
pub mod es30;
pub mod es31;
pub mod es32;

pub mod consts;
pub mod egl;
pub mod enums;
pub mod types;
