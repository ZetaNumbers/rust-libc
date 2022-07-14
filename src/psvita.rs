pub type c_char = i8;
pub type wchar_t = u16;

pub type c_int = i32;
pub type c_uint = u32;
pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type c_long = i32;
pub type c_ulong = u32;

pub type c_double = f64;
pub type c_float = f32;

pub type intmax_t = i64;
pub type uintmax_t = u64;

pub type ptrdiff_t = isize;
pub type size_t = usize;
pub type ssize_t = isize;
pub type intptr_t = isize;
pub type uintptr_t = usize;

cfg_if! {
    if #[cfg(libc_core_cvoid)] {
        pub use ::ffi::c_void;
    } else {
        // Use repr(u8) as LLVM expects `void*` to be the same as `i8*` to help
        // enable more optimization opportunities around it recognizing things
        // like malloc/free.
        #[repr(u8)]
        #[allow(missing_copy_implementations)]
        #[allow(missing_debug_implementations)]
        pub enum c_void {
            // Two dummy variants so the #[repr] attribute can be used.
            #[doc(hidden)]
            __variant1,
            #[doc(hidden)]
            __variant2,
        }
    }
}

pub type SceChar8 = i8;
pub type SceUChar8 = u8;

pub type SceInt8 = i8;
pub type SceUInt8 = u8;

pub type SceShort16 = i16;
pub type SceUShort16 = u16;

pub type SceInt16 = i16;
pub type SceUInt16 = u16;

pub type SceInt32 = i32;
pub type SceUInt32 = u32;

pub type SceInt = i32;
pub type SceUInt = u32;

pub type SceInt64 = i64;
pub type SceUInt64 = u64;

pub type SceLong64 = i64;
pub type SceULong64 = u64;

pub type SceSize = usize;
pub type SceSSize = isize;

pub type SceBool = i32;
pub const SCE_FALSE: SceBool = 0;
pub const SCE_TRUE: SceBool = 1;

pub type SceFloat = f32;
pub type SceFloat32 = f32;

pub type SceDouble = f64;
pub type SceDouble64 = f64;

pub type SceSByte = i8;
pub type SceSByte8 = i8;

pub type SceByte = u8;
pub type SceByte8 = u8;

pub type SceWChar16 = u16;
pub type SceWChar32 = u32;

pub type SceVoid = c_void;
pub type ScePVoid = *mut c_void;

pub type SceIntPtr = isize;
pub type SceUIntPtr = usize;
pub type SceUIntVAddr = usize;

pub type SceOff = SceInt64;

pub type SceUID = i32;

pub type SceName = *mut c_char;

pub const SCE_UID_NAMELEN: usize = 31;

extern "C" {
    pub fn calloc(nobj: size_t, size: size_t) -> *mut c_void;
    pub fn malloc(size: size_t) -> *mut c_void;
    pub fn realloc(p: *mut c_void, size: size_t) -> *mut c_void;
    pub fn free(p: *mut c_void);
    pub fn memalign(align: size_t, size: size_t) -> *mut c_void;
}
