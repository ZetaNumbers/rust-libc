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

s_no_extra_traits! {
    #[allow(missing_debug_implementations)]
    pub union max_align_t {
        float: f64,
        int: i64,
    }
}

pub type errno_t = c_int;

pub const EXIT_SUCCESS: c_int = 0;
pub const EXIT_FAILURE: c_int = 1;

s! {
    pub struct FILE {
        data: [u64; 32],
    }

    pub struct fpos_t {
        data: [u64; 2],
    }
}

extern "C" {
    pub fn calloc(nobj: size_t, size: size_t) -> *mut c_void;
    pub fn malloc(size: size_t) -> *mut c_void;
    pub fn realloc(p: *mut c_void, size: size_t) -> *mut c_void;
    pub fn free(p: *mut c_void);
    pub fn memalign(align: size_t, size: size_t) -> *mut c_void;

    pub fn abort() -> !;
    pub fn exit(code: c_int) -> !;
    pub fn quick_exit(code: c_int) -> !;
    pub fn _Exit(code: c_int) -> !;
    pub fn atexit(cb: extern "C" fn()) -> c_int;
    pub fn at_quick_exit(cb: extern "C" fn()) -> c_int;

    #[link_name = "_Stderr"]
    pub static stderr: FILE;
    #[link_name = "_Stdin"]
    pub static stdin: FILE;
    #[link_name = "_Stdout"]
    pub static stdout: FILE;
    pub fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE;
    pub fn freopen(filename: *const c_char, mode: *const c_char, file: *mut FILE) -> *mut FILE;
    pub fn fflush(file: *mut FILE) -> c_int;
    pub fn fclose(file: *mut FILE) -> c_int;
    pub fn remove(filename: *const c_char) -> c_int;
    pub fn rename(oldname: *const c_char, newname: *const c_char) -> c_int;
    pub fn setvbuf(stream: *mut FILE, buffer: *mut c_char, mode: c_int, size: size_t) -> c_int;
    pub fn setbuf(stream: *mut FILE, buf: *mut c_char);
    pub fn getchar() -> c_int;
    pub fn putchar(c: c_int) -> c_int;
    pub fn fgetc(stream: *mut FILE) -> c_int;
    pub fn fgets(buf: *mut c_char, n: c_int, stream: *mut FILE) -> *mut c_char;
    pub fn fputc(c: c_int, stream: *mut FILE) -> c_int;
    pub fn fputs(s: *const c_char, stream: *mut FILE) -> c_int;
    pub fn puts(s: *const c_char) -> c_int;
    pub fn ungetc(c: c_int, stream: *mut FILE) -> c_int;
    pub fn fread(ptr: *mut c_void, size: size_t, nobj: size_t, stream: *mut FILE) -> size_t;
    pub fn fwrite(ptr: *const c_void, size: size_t, nobj: size_t, stream: *mut FILE) -> size_t;
    pub fn fseek(stream: *mut FILE, offset: c_long, whence: c_int) -> c_int;
    pub fn ftell(stream: *mut FILE) -> c_long;
    pub fn rewind(stream: *mut FILE);
    pub fn fgetpos(stream: *mut FILE, ptr: *mut fpos_t) -> c_int;
    pub fn fsetpos(stream: *mut FILE, ptr: *const fpos_t) -> c_int;
    pub fn feof(stream: *mut FILE) -> c_int;
    pub fn ferror(stream: *mut FILE) -> c_int;
    pub fn clearerr(stream: *mut FILE);
    pub fn perror(s: *const c_char);
    pub fn printf(format: *const c_char, ...) -> ::c_int;
    pub fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> ::c_int;
    pub fn fopen_s(
        steamptr: *mut *mut FILE,
        filename: *const c_char,
        mode: *const c_char,
    ) -> errno_t;
    pub fn freopen_s(
        newsteamptr: *mut *mut FILE,
        filename: *const c_char,
        mode: *const c_char,
        stream: *mut FILE,
    ) -> errno_t;
    pub fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    pub fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    pub fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    pub fn scanf(format: *const c_char, ...) -> c_int;
    pub fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;

    pub fn _sceLibcErrnoLoc() -> *mut errno_t;

    #[link_name = "_Thrd_yield"]
    pub fn thrd_yield();
}
