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

pub const SCE_KERNEL_THREAD_CPU_AFFINITY_MASK_DEFAULT: u32 = 0;

pub type SceKernelThreadEntry = extern "C" fn(args: SceSize, argp: *mut c_void) -> c_int;
pub type SceKernelCallbackFunction = extern "C" fn(
    notifyId: c_int,
    notifyCount: c_int,
    notifyArg: c_int,
    userData: *mut c_void,
) -> c_int;

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

pub type SceThreadStatus = c_uint;
pub const SCE_THREAD_RUNNING: SceThreadStatus = 1;
pub const SCE_THREAD_READY: SceThreadStatus = 2;
pub const SCE_THREAD_STANDBY: SceThreadStatus = 4;
pub const SCE_THREAD_WAITING: SceThreadStatus = 8;
pub const SCE_THREAD_SUSPEND: SceThreadStatus = 8;
pub const SCE_THREAD_DORMANT: SceThreadStatus = 16;
pub const SCE_THREAD_STOPPED: SceThreadStatus = 16;
pub const SCE_THREAD_DELETED: SceThreadStatus = 32;
pub const SCE_THREAD_KILLED: SceThreadStatus = 32;
pub const SCE_THREAD_DEAD: SceThreadStatus = 64;
pub const SCE_THREAD_STAGNANT: SceThreadStatus = 128;
pub const SCE_THREAD_SUSPENDED: SceThreadStatus = 256;

pub type SceKernelWaitableAttribute = c_uint;
pub const SCE_KERNEL_ATTR_THREAD_FIFO: SceKernelWaitableAttribute = 0;
pub const SCE_KERNEL_ATTR_THREAD_PRIO: SceKernelWaitableAttribute = 8192;
pub const SCE_KERNEL_ATTR_OPENABLE: SceKernelWaitableAttribute = 128;

pub type SceEventFlagAttributes = c_uint;
pub const SCE_EVENT_THREAD_FIFO: SceEventFlagAttributes = 0;
pub const SCE_EVENT_THREAD_PRIO: SceEventFlagAttributes = 8192;
pub const SCE_EVENT_WAITSINGLE: SceEventFlagAttributes = 0;
pub const SCE_EVENT_WAITMULTIPLE: SceEventFlagAttributes = 4096;
pub const SCE_EVENT_OPENABLE: SceEventFlagAttributes = 128;

pub type SceKernelMutexAttribute = c_uint;
pub const SCE_KERNEL_MUTEX_ATTR_RECURSIVE: SceKernelMutexAttribute = 2;
pub const SCE_KERNEL_MUTEX_ATTR_CEILING: SceKernelMutexAttribute = 4;

pub type SceEventFlagWaitTypes = c_uint;
pub const SCE_EVENT_WAITAND: SceEventFlagWaitTypes = 0;
pub const SCE_EVENT_WAITOR: SceEventFlagWaitTypes = 1;
pub const SCE_EVENT_WAITCLEAR: SceEventFlagWaitTypes = 2;
pub const SCE_EVENT_WAITCLEAR_PAT: SceEventFlagWaitTypes = 4;

pub type SceKernelIdListType = c_uint;
pub const SCE_KERNEL_TMID_Thread: SceKernelIdListType = 1;
pub const SCE_KERNEL_TMID_Semaphore: SceKernelIdListType = 2;
pub const SCE_KERNEL_TMID_EventFlag: SceKernelIdListType = 3;
pub const SCE_KERNEL_TMID_Mbox: SceKernelIdListType = 4;
pub const SCE_KERNEL_TMID_Vpl: SceKernelIdListType = 5;
pub const SCE_KERNEL_TMID_Fpl: SceKernelIdListType = 6;
pub const SCE_KERNEL_TMID_Mpipe: SceKernelIdListType = 7;
pub const SCE_KERNEL_TMID_Callback: SceKernelIdListType = 8;
pub const SCE_KERNEL_TMID_ThreadEventHandler: SceKernelIdListType = 9;
pub const SCE_KERNEL_TMID_Alarm: SceKernelIdListType = 10;
pub const SCE_KERNEL_TMID_VTimer: SceKernelIdListType = 11;
pub const SCE_KERNEL_TMID_SleepThread: SceKernelIdListType = 64;
pub const SCE_KERNEL_TMID_DelayThread: SceKernelIdListType = 65;
pub const SCE_KERNEL_TMID_SuspendThread: SceKernelIdListType = 66;
pub const SCE_KERNEL_TMID_DormantThread: SceKernelIdListType = 67;

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

    pub struct SceKernelThreadOptParam {
        pub size: SceSize,
        pub attr: SceUInt32,
    }

    pub struct SceKernelThreadInfo {
        pub size: SceSize,
        pub processId: SceUID,
        pub name: [c_char; 32],
        pub attr: SceUInt32,
        pub status: SceUInt32,
        pub entry: SceKernelThreadEntry,
        pub stack: *mut c_void,
        pub stackSize: SceInt32,
        pub initPriority: SceInt32,
        pub currentPriority: SceInt32,
        pub initCpuAffinityMask: SceInt32,
        pub currentCpuAffinityMask: SceInt32,
        pub currentCpuId: SceInt32,
        pub lastExecutedCpuId: SceInt32,
        pub waitType: SceUInt32,
        pub waitId: SceUID,
        pub exitStatus: SceInt32,
        pub runClocks: SceKernelSysClock,
        pub intrPreemptCount: SceUInt32,
        pub threadPreemptCount: SceUInt32,
        pub threadReleaseCount: SceUInt32,
        pub changeCpuCount: SceInt32,
        pub fNotifyCallback: SceInt32,
        pub reserved: SceInt32,
    }

    pub struct SceKernelThreadRunStatus {
        pub size: SceSize,
        pub cpuInfo: [SceKernelThreadRunStatusCpuInfo; 4],
    }

    pub struct SceKernelThreadRunStatusCpuInfo {
        pub processId: SceUID,
        pub threadId: SceUID,
        pub priority: c_int,
    }

    pub struct SceKernelSemaOptParam {
        pub size: SceSize,
    }

    pub struct SceKernelSemaInfo {
        pub size: SceSize,
        pub semaId: SceUID,
        pub name: [c_char; 32],
        pub attr: SceUInt,
        pub initCount: c_int,
        pub currentCount: c_int,
        pub maxCount: c_int,
        pub numWaitThreads: c_int,
    }

    pub struct SceKernelMutexOptParam {
        pub size: SceSize,
        pub ceilingPriority: c_int,
    }

    pub struct SceKernelMutexInfo {
        pub size: SceSize,
        pub mutexId: SceUID,
        pub name: [c_char; 32],
        pub attr: SceUInt,
        pub initCount: c_int,
        pub currentCount: c_int,
        pub currentOwnerId: SceUID,
        pub numWaitThreads: c_int,
    }

    pub struct SceKernelEventFlagInfo {
        pub size: SceSize,
        pub evfId: SceUID,
        pub name: [c_char; 32],
        pub attr: SceUInt,
        pub initPattern: SceUInt,
        pub currentPattern: SceUInt,
        pub numWaitThreads: c_int,
    }

    pub struct SceKernelEventFlagOptParam {
        pub size: SceSize,
    }

    pub struct SceKernelCondOptParam {
        pub size: SceSize,
    }

    pub struct SceKernelCondInfo {
        pub size: SceSize,
        pub condId: SceUID,
        pub name: [c_char; 32],
        pub attr: SceUInt,
        pub mutexId: SceUID,
        pub numWaitThreads: c_int,
    }

    pub struct SceKernelCallbackInfo {
        pub size: SceSize,
        pub callbackId: SceUID,
        pub name: [c_char; 32],
        pub threadId: SceUID,
        pub callback: SceKernelCallbackFunction,
        pub common: *mut c_void,
        pub notifyCount: c_int,
        pub notifyArg: c_int,
    }

    pub struct SceKernelMppInfo {
        pub size: SceSize,
        pub mppId: SceUID,
        pub name: [c_char; 32],
        pub attr: SceUInt,
        pub bufSize: c_int,
        pub freeSize: c_int,
        pub numSendWaitThreads: c_int,
        pub numReceiveWaitThreads: c_int,
    }

    pub struct SceKernelSystemInfo {
        pub size: SceSize,
        pub activeCpuMask: SceUInt32,
        pub cpuInfo: [SceKernelSystemInfoCpuInfo; 4],
    }

    pub struct SceKernelSystemInfoCpuInfo {
        pub idleClock: SceKernelSysClock,
        pub comesOutOfIdleCount: SceUInt32,
        pub threadSwitchCount: SceUInt32,
    }

    pub struct SceKernelLwMutexWork {
        pub data: [SceInt64; 4],
    }

    pub struct SceKernelLwMutexOptParam {
        pub size: SceSize,
    }

    pub struct SceKernelLwCondWork {
        pub data: [SceInt32; 4],
    }

    pub struct SceKernelLwCondOptParam {
        pub size: SceSize,
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

    pub fn sceKernelCreateThread(
        name: *const c_char,
        entry: SceKernelThreadEntry,
        initPriority: c_int,
        stackSize: SceSize,
        attr: SceUInt,
        cpuAffinityMask: c_int,
        option: *const SceKernelThreadOptParam,
    ) -> SceUID;
    pub fn sceKernelDeleteThread(thid: SceUID) -> c_int;
    pub fn sceKernelStartThread(thid: SceUID, arglen: SceSize, argp: *mut c_void) -> c_int;
    pub fn sceKernelExitThread(status: c_int) -> c_int;
    pub fn sceKernelExitDeleteThread(status: c_int) -> c_int;
    pub fn sceKernelWaitThreadEnd(thid: SceUID, stat: *mut c_int, timeout: *mut SceUInt) -> c_int;
    pub fn sceKernelWaitThreadEndCB(thid: SceUID, stat: *mut c_int, timeout: *mut SceUInt)
        -> c_int;
    pub fn sceKernelDelayThread(usecs: SceUInt) -> c_int;
    pub fn sceKernelDelayThreadCB(delay: SceUInt) -> c_int;
    pub fn sceKernelChangeCurrentThreadAttr(clearAttr: SceUInt, setAttr: SceUInt) -> c_int;
    pub fn sceKernelChangeThreadPriority(thid: SceUID, priority: c_int) -> c_int;
    pub fn sceKernelGetThreadId() -> c_int;
    pub fn sceKernelGetProcessId() -> SceUID;
    pub fn sceKernelGetThreadCurrentPriority() -> c_int;
    pub fn sceKernelGetThreadExitStatus(thid: SceUID, status: *mut c_int) -> c_int;
    pub fn sceKernelCheckThreadStack() -> c_int;
    pub fn sceKernelGetThreadStackFreeSize(thid: SceUID) -> c_int;
    pub fn sceKernelGetThreadInfo(thid: SceUID, info: *mut SceKernelThreadInfo) -> c_int;
    pub fn sceKernelGetThreadRunStatus(
        thid: SceUID,
        status: *mut SceKernelThreadRunStatus,
    ) -> c_int;
    pub fn sceKernelGetThreadCpuAffinityMask(thid: SceUID) -> c_int;
    pub fn sceKernelChangeThreadCpuAffinityMask(thid: SceUID, mask: c_int) -> c_int;
    pub fn sceKernelCreateSema(
        name: *const c_char,
        attr: SceUInt,
        initVal: c_int,
        maxVal: c_int,
        option: *mut SceKernelSemaOptParam,
    ) -> SceUID;
    pub fn sceKernelDeleteSema(semaid: SceUID) -> c_int;
    pub fn sceKernelSignalSema(semaid: SceUID, signal: c_int) -> c_int;
    pub fn sceKernelWaitSema(semaid: SceUID, signal: c_int, timeout: *mut SceUInt) -> c_int;
    pub fn sceKernelWaitSemaCB(semaid: SceUID, signal: c_int, timeout: *mut SceUInt) -> c_int;
    pub fn sceKernelPollSema(semaid: SceUID, signal: c_int) -> c_int;
    pub fn sceKernelCancelSema(
        semaid: SceUID,
        setCount: c_int,
        numWaitThreads: *mut c_int,
    ) -> c_int;
    pub fn sceKernelGetSemaInfo(semaid: SceUID, info: *mut SceKernelSemaInfo) -> c_int;
    pub fn sceKernelOpenSema(name: *const c_char) -> SceUID;
    pub fn sceKernelCloseSema(semaid: SceUID) -> c_int;
    pub fn sceKernelCreateMutex(
        name: *const c_char,
        attr: SceUInt,
        initCount: c_int,
        option: *mut SceKernelMutexOptParam,
    ) -> SceUID;
    pub fn sceKernelDeleteMutex(mutexid: SceUID) -> c_int;
    pub fn sceKernelOpenMutex(name: *const c_char) -> c_int;
    pub fn sceKernelCloseMutex(mutexid: SceUID) -> c_int;
    pub fn sceKernelLockMutex(mutexid: SceUID, lockCount: c_int, timeout: *mut c_uint) -> c_int;
    pub fn sceKernelLockMutexCB(mutexid: SceUID, lockCount: c_int, timeout: *mut c_uint) -> c_int;
    pub fn sceKernelTryLockMutex(mutexid: SceUID, lockCount: c_int) -> c_int;
    pub fn sceKernelUnlockMutex(mutexid: SceUID, unlockCount: c_int) -> c_int;
    pub fn sceKernelCancelMutex(
        mutexid: SceUID,
        newCount: c_int,
        numWaitThreads: *mut c_int,
    ) -> c_int;
    pub fn sceKernelGetMutexInfo(mutexid: SceUID, info: *mut SceKernelMutexInfo) -> c_int;
    pub fn sceKernelCreateEventFlag(
        name: *const c_char,
        attr: c_int,
        bits: c_int,
        opt: *mut SceKernelEventFlagOptParam,
    ) -> SceUID;
    pub fn sceKernelSetEventFlag(evid: SceUID, bits: c_uint) -> c_int;
    pub fn sceKernelClearEventFlag(evid: SceUID, bits: c_uint) -> c_int;
    pub fn sceKernelPollEventFlag(
        evid: c_int,
        bits: c_uint,
        wait: c_uint,
        outBits: *mut c_uint,
    ) -> c_int;
    pub fn sceKernelWaitEventFlag(
        evid: c_int,
        bits: c_uint,
        wait: c_uint,
        outBits: *mut c_uint,
        timeout: *mut SceUInt,
    ) -> c_int;
    pub fn sceKernelWaitEventFlagCB(
        evid: c_int,
        bits: c_uint,
        wait: c_uint,
        outBits: *mut c_uint,
        timeout: *mut SceUInt,
    ) -> c_int;
    pub fn sceKernelDeleteEventFlag(evid: c_int) -> c_int;
    pub fn sceKernelGetEventFlagInfo(event: SceUID, info: *mut SceKernelEventFlagInfo) -> c_int;
    pub fn sceKernelCreateCond(
        name: *const c_char,
        attr: SceUInt,
        mutexId: SceUID,
        option: *const SceKernelCondOptParam,
    ) -> SceUID;
    pub fn sceKernelDeleteCond(condId: SceUID) -> c_int;
    pub fn sceKernelOpenCond(name: *const c_char) -> c_int;
    pub fn sceKernelCloseCond(condId: SceUID) -> c_int;
    pub fn sceKernelWaitCond(condId: SceUID, timeout: *mut c_uint) -> c_int;
    pub fn sceKernelWaitCondCB(condId: SceUID, timeout: *mut c_uint) -> c_int;
    pub fn sceKernelSignalCond(condId: SceUID) -> c_int;
    pub fn sceKernelSignalCondAll(condId: SceUID) -> c_int;
    pub fn sceKernelSignalCondTo(condId: SceUID, threadId: SceUID) -> c_int;
    pub fn sceKernelCreateCallback(
        name: *const c_char,
        attr: c_uint,
        func: SceKernelCallbackFunction,
        userData: *mut c_void,
    ) -> c_int;
    pub fn sceKernelGetCallbackInfo(cb: SceUID, infop: *mut SceKernelCallbackInfo) -> c_int;
    pub fn sceKernelDeleteCallback(cb: SceUID) -> c_int;
    pub fn sceKernelNotifyCallback(cb: SceUID, arg2: c_int) -> c_int;
    pub fn sceKernelCancelCallback(cb: SceUID) -> c_int;
    pub fn sceKernelGetCallbackCount(cb: SceUID) -> c_int;
    pub fn sceKernelCheckCallback() -> c_int;
    pub fn sceKernelCreateMsgPipe(
        name: *const c_char,
        type_: c_int,
        attr: c_int,
        bufSize: c_uint,
        opt: *mut c_void,
    ) -> SceUID;
    pub fn sceKernelDeleteMsgPipe(uid: SceUID) -> c_int;
    pub fn sceKernelSendMsgPipe(
        uid: SceUID,
        message: *mut c_void,
        size: c_uint,
        unk1: c_int,
        unk2: *mut c_void,
        timeout: *mut c_uint,
    ) -> c_int;
    pub fn sceKernelSendMsgPipeCB(
        uid: SceUID,
        message: *mut c_void,
        size: c_uint,
        unk1: c_int,
        unk2: *mut c_void,
        timeout: *mut c_uint,
    ) -> c_int;
    pub fn sceKernelTrySendMsgPipe(
        uid: SceUID,
        message: *mut c_void,
        size: SceSize,
        unk1: c_int,
        unk2: *mut c_void,
    ) -> c_int;
    pub fn sceKernelReceiveMsgPipe(
        uid: SceUID,
        message: *mut c_void,
        size: SceSize,
        unk1: c_int,
        unk2: *mut c_void,
        timeout: *mut c_uint,
    ) -> c_int;
    pub fn sceKernelReceiveMsgPipeCB(
        uid: SceUID,
        message: *mut c_void,
        size: SceSize,
        unk1: c_int,
        unk2: *mut c_void,
        timeout: *mut c_uint,
    ) -> c_int;
    pub fn sceKernelTryReceiveMsgPipe(
        uid: SceUID,
        message: *mut c_void,
        size: SceSize,
        unk1: c_int,
        unk2: *mut c_void,
    ) -> c_int;
    pub fn sceKernelCancelMsgPipe(uid: SceUID, psend: *mut c_int, precv: *mut c_int) -> c_int;
    pub fn sceKernelGetMsgPipeInfo(uid: SceUID, info: *mut SceKernelMppInfo) -> c_int;
    pub fn sceKernelGetSystemInfo(info: *mut SceKernelSystemInfo) -> c_int;
    pub fn sceKernelGetThreadmgrUIDClass(uid: SceUID) -> SceKernelIdListType;
    pub fn sceKernelCreateLwMutex(
        pWork: *mut SceKernelLwMutexWork,
        pName: *const c_char,
        attr: c_uint,
        initCount: c_int,
        pOptParam: *const SceKernelLwMutexOptParam,
    ) -> c_int;
    pub fn sceKernelDeleteLwMutex(pWork: *mut SceKernelLwMutexWork) -> c_int;
    pub fn sceKernelLockLwMutex(
        pWork: *mut SceKernelLwMutexWork,
        lockCount: c_int,
        pTimeout: *mut c_uint,
    ) -> c_int;
    pub fn sceKernelTryLockLwMutex(pWork: *mut SceKernelLwMutexWork, lockCount: c_int) -> c_int;
    pub fn sceKernelUnlockLwMutex(pWork: *mut SceKernelLwMutexWork, unlockCount: c_int) -> c_int;
    pub fn sceKernelCreateLwCond(
        pWork: *mut SceKernelLwCondWork,
        pName: *const c_char,
        attr: c_uint,
        pLwMutex: *mut SceKernelLwMutexWork,
        pOptParam: *const SceKernelLwCondOptParam,
    ) -> c_int;
    pub fn sceKernelDeleteLwCond(pWork: *mut SceKernelLwCondWork) -> c_int;
    pub fn sceKernelSignalLwCond(pWork: *mut SceKernelLwCondWork) -> c_int;
    pub fn sceKernelWaitLwCond(pWork: *mut SceKernelLwCondWork, pTimeout: *mut c_uint) -> c_int;
    pub fn sceKernelWaitSignal(unk0: SceUInt32, delay: SceUInt32, timeout: *mut SceUInt32)
        -> c_int;
    pub fn sceKernelSendSignal(thid: SceUID) -> c_int;
    pub fn sceKernelGetSystemTimeWide() -> SceInt64;
    pub fn sceKernelGetThreadTLSAddr(thid: SceUID, key: c_int) -> *mut c_void;
    pub fn sceKernelGetTLSAddr(key: c_int) -> *mut c_void;
}
