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

pub type SceKernelSysClock = SceUInt64;
pub type SceKernelClock = SceUInt64;
pub type SceKernelTime = SceUInt32;

pub const SCE_UID_NAMELEN: usize = 31;



pub type SceKernelMemBlockType = u32;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_CDRAM_L1WBWA_RW: SceKernelMemBlockType = 0x09404060;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_CDRAM_R: SceKernelMemBlockType = 0x09408040;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_CDRAM_RW: SceKernelMemBlockType = 0x09408060;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_DEVICE_RW: SceKernelMemBlockType = 0x0C200860;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_R: SceKernelMemBlockType = 0x0C20D040;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_RW: SceKernelMemBlockType = 0x0C20D060;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_NC_RW: SceKernelMemBlockType = 0x0C208060;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_GAME_RW: SceKernelMemBlockType = 0x0C50D060;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_PHYCONT_RW: SceKernelMemBlockType = 0x0C80D060;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_PHYCONT_NC_RW: SceKernelMemBlockType = 0x0D808060;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_CDIALOG_RW: SceKernelMemBlockType = 0x0CA0D060;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_CDIALOG_NC_RW: SceKernelMemBlockType = 0x0CA08060;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_TOOL_RW: SceKernelMemBlockType = 0x0CF0D060;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_TOOL_NC_RW: SceKernelMemBlockType = 0x0CF08060;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_CDIALOG_R: SceKernelMemBlockType = 0x0E20D040;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_CDIALOG_RW: SceKernelMemBlockType = 0x0E20D060;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_CDIALOG_NC_R: SceKernelMemBlockType = 0x0E208040;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_CDIALOG_NC_RW: SceKernelMemBlockType = 0x0E208060;

pub type SceKernelProcessPrioritySystem = c_uint;
pub const SCE_KERNEL_PROCESS_PRIORITY_SYSTEM_HIGH: SceKernelProcessPrioritySystem = 32;
pub const SCE_KERNEL_PROCESS_PRIORITY_SYSTEM_DEFAULT: SceKernelProcessPrioritySystem = 96;
pub const SCE_KERNEL_PROCESS_PRIORITY_SYSTEM_LOW: SceKernelProcessPrioritySystem = 159;

pub type SceKernelProcessPriorityUser = c_uint;
pub const SCE_KERNEL_PROCESS_PRIORITY_USER_HIGH: SceKernelProcessPriorityUser = 64;
pub const SCE_KERNEL_PROCESS_PRIORITY_USER_DEFAULT: SceKernelProcessPriorityUser = 96;
pub const SCE_KERNEL_PROCESS_PRIORITY_USER_LOW: SceKernelProcessPriorityUser = 127;

pub type SceKernelPowerTickType = c_uint;
pub const SCE_KERNEL_POWER_TICK_DEFAULT: SceKernelPowerTickType = 0;
pub const SCE_KERNEL_POWER_TICK_DISABLE_AUTO_SUSPEND: SceKernelPowerTickType = 1;
pub const SCE_KERNEL_POWER_TICK_DISABLE_OLED_OFF: SceKernelPowerTickType = 4;
pub const SCE_KERNEL_POWER_TICK_DISABLE_OLED_DIMMING: SceKernelPowerTickType = 6;

s! {
    pub struct FILE {
        data: [u64; 32],
    }

    pub struct fpos_t {
        data: [u64; 2],
    }

    pub struct SceKernelAllocMemBlockOpt {
        pub size: SceSize,
        pub attr: SceUInt32,
        pub alignment: SceSize,
        pub uidBaseBlock: SceUID,
        pub strBaseBlockName: *const c_char,
        /// Unknown flags 0x10 or 0x30 for ::sceKernelOpenMemBlock
        pub flags: c_int,
        pub reserved: [c_int; 10],
    }

    pub struct SceKernelMemBlockInfo {
        pub size: SceSize,
        pub mappedBase: *mut ::c_void,
        pub mappedSize: SceSize,
        pub memoryType: ::c_int,
        pub access: SceUInt32,
        pub type_: SceKernelMemBlockType,
    }

    pub struct SceKernelFreeMemorySizeInfo {
        pub size: c_int,
        pub size_user: c_int,
        pub size_cdram: c_int,
        pub size_phycont: c_int,
    }

    pub struct SceKernelTimeval {
        pub sec: i32,
        pub nsec: i32,
    }

    pub struct SceKernelTimezone {
        inner: u64,
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

    pub fn sceKernelGetRandomNumber(output: *mut c_void, size: SceSize) -> c_int;
    /// - `opt` can be null
    pub fn sceKernelAllocMemBlock(
        name: *const c_char,
        ty: SceKernelMemBlockType,
        size: SceSize,
        opt: *const SceKernelAllocMemBlockOpt,
    ) -> SceUID;
    pub fn sceKernelFreeMemBlock(uid: SceUID) -> c_int;
    pub fn sceKernelGetMemBlockBase(uid: SceUID, base: *mut *mut c_void) -> c_int;
    pub fn sceKernelFindMemBlockByAddr(addr: *const c_void, size: SceSize) -> SceUID;
    pub fn sceKernelGetMemBlockInfoByAddr(
        base: *mut c_void,
        info: *mut SceKernelMemBlockInfo,
    ) -> c_int;
    pub fn sceKernelGetMemBlockInfoByRange(
        base: *mut c_void,
        size: SceSize,
        info: *mut SceKernelMemBlockInfo,
    ) -> c_int;
    pub fn sceKernelAllocMemBlockForVM(name: *const char, size: SceSize) -> SceUID;
    pub fn sceKernelSyncVMDomain(uid: SceUID, data: *mut c_void, size: SceSize) -> c_int;
    pub fn sceKernelOpenVMDomain() -> c_int;
    pub fn sceKernelCloseVMDomain() -> c_int;
    pub fn sceKernelOpenMemBlock(name: *const char, flags: c_int) -> c_int;
    pub fn sceKernelCloseMemBlock(uid: SceUID) -> c_int;
    pub fn sceKernelGetModelForCDialog() -> c_int;
    pub fn sceKernelGetModel() -> c_int;
    pub fn sceKernelGetFreeMemorySize(info: *mut SceKernelFreeMemorySizeInfo) -> c_int;
    pub fn sceKernelIsPSVitaTV() -> SceBool;

    pub fn sceKernelExitProcess(res: c_int) -> c_int;
    pub fn sceKernelPowerTick(type_: SceKernelPowerTickType) -> c_int;
    pub fn sceKernelPowerLock(type_: SceKernelPowerTickType) -> c_int;
    pub fn sceKernelPowerUnlock(type_: SceKernelPowerTickType) -> c_int;
    pub fn sceKernelGetProcessTime(pSysClock: *mut SceKernelSysClock) -> c_int;
    pub fn sceKernelGetProcessTimeLow() -> SceUInt32;
    pub fn sceKernelGetProcessTimeWide() -> SceUInt64;
    pub fn sceKernelGetCurrentProcess() -> SceUID;
    pub fn sceKernelGetRemoteProcessTime(
        processId: SceUID,
        pClock: *mut SceKernelSysClock,
    ) -> SceInt32;
    pub fn sceKernelGetStderr() -> SceUID;
    pub fn sceKernelGetStdin() -> SceUID;
    pub fn sceKernelGetStdout() -> SceUID;
    pub fn sceKernelGetProcessParam() -> *const c_void;
    pub fn sceKernelLibcClock() -> SceKernelClock;
    pub fn sceKernelLibcTime(tloc: *mut SceKernelTime) -> SceKernelTime;
    pub fn sceKernelLibcGettimeofday(
        tv: *mut SceKernelTimeval,
        tz: *mut SceKernelTimezone,
    ) -> c_int;
}
