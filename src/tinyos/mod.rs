use crate::prelude::*;

pub type size_t = usize;
pub type ssize_t = isize;

pub type off_t = i64;

pub type c_uint8_t = u8;
pub type c_uint16_t = u16;
pub type c_uint32_t = u32;
pub type c_uint64_t = u64;

pub type c_int8_t = i8;
pub type c_int16_t = i16;
pub type c_int32_t = i32;
pub type c_int64_t = i64;

pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type ptrdiff_t = isize;

pub type time_t = c_long;

pub type clockid_t = c_int;

extern "C" {}
