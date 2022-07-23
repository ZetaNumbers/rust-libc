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

pub const SCE_OK: c_int = 0;
pub const SCE_UID_NAMELEN: usize = 31;

pub const SCE_KERNEL_THREAD_CPU_AFFINITY_MASK_DEFAULT: c_int = 0;

pub type SceKernelThreadEntry = unsafe extern "C" fn(args: SceSize, argp: *mut c_void) -> c_int;
pub type SceKernelCallbackFunction = unsafe extern "C" fn(
    notifyId: c_int,
    notifyCount: c_int,
    notifyArg: c_int,
    userData: *mut c_void,
) -> c_int;

pub type SceKernelMemBlockType = SceUInt32;
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

pub type SceKernelAllocMemBlockAttr = c_uint;
pub const SCE_KERNEL_ALLOC_MEMBLOCK_ATTR_HAS_ALIGNMENT: SceKernelAllocMemBlockAttr = 4;

pub type SceKernelModel = c_uint;
pub const SCE_KERNEL_MODEL_VITA: SceKernelModel = 65536;
pub const SCE_KERNEL_MODEL_VITATV: SceKernelModel = 131072;

pub type SceKernelMemoryAccessType = c_uint;
pub const SCE_KERNEL_MEMORY_ACCESS_X: SceKernelMemoryAccessType = 1;
pub const SCE_KERNEL_MEMORY_ACCESS_W: SceKernelMemoryAccessType = 2;
pub const SCE_KERNEL_MEMORY_ACCESS_R: SceKernelMemoryAccessType = 4;

pub type SceKernelMemoryType = c_uint;
pub const SCE_KERNEL_MEMORY_TYPE_NORMAL_NC: SceKernelMemoryType = 128;
pub const SCE_KERNEL_MEMORY_TYPE_NORMAL: SceKernelMemoryType = 208;

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

pub type SceDisplayErrorCode = c_int;
pub const SCE_DISPLAY_ERROR_OK: SceDisplayErrorCode = 0;
pub const SCE_DISPLAY_ERROR_INVALID_HEAD: SceDisplayErrorCode = 2150170624;
pub const SCE_DISPLAY_ERROR_INVALID_VALUE: SceDisplayErrorCode = 2150170625;
pub const SCE_DISPLAY_ERROR_INVALID_ADDR: SceDisplayErrorCode = 2150170626;
pub const SCE_DISPLAY_ERROR_INVALID_PIXELFORMAT: SceDisplayErrorCode = 2150170627;
pub const SCE_DISPLAY_ERROR_INVALID_PITCH: SceDisplayErrorCode = 2150170628;
pub const SCE_DISPLAY_ERROR_INVALID_RESOLUTION: SceDisplayErrorCode = 2150170629;
pub const SCE_DISPLAY_ERROR_INVALID_UPDATETIMING: SceDisplayErrorCode = 2150170630;
pub const SCE_DISPLAY_ERROR_NO_FRAME_BUFFER: SceDisplayErrorCode = 2150170631;
pub const SCE_DISPLAY_ERROR_NO_PIXEL_DATA: SceDisplayErrorCode = 2150170632;
pub const SCE_DISPLAY_ERROR_NO_OUTPUT_SIGNAL: SceDisplayErrorCode = 2150170633;

pub type SceDisplayPixelFormat = c_uint;
pub const SCE_DISPLAY_PIXELFORMAT_A8B8G8R8: SceDisplayPixelFormat = 0;

pub type SceDisplaySetBufSync = c_uint;
pub const SCE_DISPLAY_SETBUF_IMMEDIATE: SceDisplaySetBufSync = 0;
pub const SCE_DISPLAY_SETBUF_NEXTFRAME: SceDisplaySetBufSync = 1;

pub type SceKernelErrorCode = c_int;
pub const SCE_KERNEL_OK: SceKernelErrorCode = 0;
pub const SCE_KERNEL_ERROR_ERROR: SceKernelErrorCode = 2147614721;
pub const SCE_KERNEL_ERROR_NOT_IMPLEMENTED: SceKernelErrorCode = 2147614722;
pub const SCE_KERNEL_ERROR_NOSYS: SceKernelErrorCode = 2147614723;
pub const SCE_KERNEL_ERROR_UNSUP: SceKernelErrorCode = 2147614724;
pub const SCE_KERNEL_ERROR_INVALID_ARGUMENT: SceKernelErrorCode = 2147614725;
pub const SCE_KERNEL_ERROR_ILLEGAL_ADDR: SceKernelErrorCode = 2147614726;
pub const SCE_KERNEL_ERROR_ILLEGAL_ALIGNMENT: SceKernelErrorCode = 2147614727;
pub const SCE_KERNEL_ERROR_ILLEGAL_PERMISSION: SceKernelErrorCode = 2147614728;
pub const SCE_KERNEL_ERROR_INVALID_ARGUMENT_SIZE: SceKernelErrorCode = 2147614729;
pub const SCE_KERNEL_ERROR_INVALID_FLAGS: SceKernelErrorCode = 2147614730;
pub const SCE_KERNEL_ERROR_ILLEGAL_SIZE: SceKernelErrorCode = 2147614731;
pub const SCE_KERNEL_ERROR_ILLEGAL_TYPE: SceKernelErrorCode = 2147614732;
pub const SCE_KERNEL_ERROR_ILLEGAL_PATTERN: SceKernelErrorCode = 2147614733;
pub const SCE_KERNEL_ERROR_ILLEGAL_ATTR: SceKernelErrorCode = 2147614734;
pub const SCE_KERNEL_ERROR_ILLEGAL_COUNT: SceKernelErrorCode = 2147614735;
pub const SCE_KERNEL_ERROR_ILLEGAL_MODE: SceKernelErrorCode = 2147614736;
pub const SCE_KERNEL_ERROR_ILLEGAL_OPEN_LIMIT: SceKernelErrorCode = 2147614737;
pub const SCE_KERNEL_ERROR_ONLY_DEVELOPMENT_MODE: SceKernelErrorCode = 2147614738;
pub const SCE_KERNEL_ERROR_DEBUG_ERROR: SceKernelErrorCode = 2147618816;
pub const SCE_KERNEL_ERROR_ILLEGAL_DIPSW_NUMBER: SceKernelErrorCode = 2147618817;
pub const SCE_KERNEL_ERROR_PA_ERROR: SceKernelErrorCode = 2147619072;
pub const SCE_KERNEL_ERROR_PA_NOT_AVAILABLE: SceKernelErrorCode = 2147619073;
pub const SCE_KERNEL_ERROR_PA_INVALID_KEY: SceKernelErrorCode = 2147619074;
pub const SCE_KERNEL_ERROR_PA_KEY_IS_NOT_SHARED: SceKernelErrorCode = 2147619075;
pub const SCE_KERNEL_ERROR_PA_INVALID_SIGNATURE: SceKernelErrorCode = 2147619076;
pub const SCE_KERNEL_ERROR_CPU_ERROR: SceKernelErrorCode = 2147622912;
pub const SCE_KERNEL_ERROR_MMU_ILLEGAL_L1_TYPE: SceKernelErrorCode = 2147622913;
pub const SCE_KERNEL_ERROR_MMU_L2_INDEX_OVERFLOW: SceKernelErrorCode = 2147622914;
pub const SCE_KERNEL_ERROR_MMU_L2_SIZE_OVERFLOW: SceKernelErrorCode = 2147622915;
pub const SCE_KERNEL_ERROR_INVALID_CPU_AFFINITY: SceKernelErrorCode = 2147622916;
pub const SCE_KERNEL_ERROR_INVALID_MEMORY_ACCESS: SceKernelErrorCode = 2147622917;
pub const SCE_KERNEL_ERROR_INVALID_MEMORY_ACCESS_PERMISSION: SceKernelErrorCode = 2147622918;
pub const SCE_KERNEL_ERROR_VA2PA_FAULT: SceKernelErrorCode = 2147622919;
pub const SCE_KERNEL_ERROR_VA2PA_MAPPED: SceKernelErrorCode = 2147622920;
pub const SCE_KERNEL_ERROR_VALIDATION_CHECK_FAILED: SceKernelErrorCode = 2147622921;
pub const SCE_KERNEL_ERROR_SYSMEM_ERROR: SceKernelErrorCode = 2147631104;
pub const SCE_KERNEL_ERROR_INVALID_PROCESS_CONTEXT: SceKernelErrorCode = 2147631105;
pub const SCE_KERNEL_ERROR_UID_NAME_TOO_LONG: SceKernelErrorCode = 2147631106;
pub const SCE_KERNEL_ERROR_VARANGE_IS_NOT_PHYSICAL_CONTINUOUS: SceKernelErrorCode = 2147631107;
pub const SCE_KERNEL_ERROR_PHYADDR_ERROR: SceKernelErrorCode = 2147631360;
pub const SCE_KERNEL_ERROR_NO_PHYADDR: SceKernelErrorCode = 2147631361;
pub const SCE_KERNEL_ERROR_PHYADDR_USED: SceKernelErrorCode = 2147631362;
pub const SCE_KERNEL_ERROR_PHYADDR_NOT_USED: SceKernelErrorCode = 2147631363;
pub const SCE_KERNEL_ERROR_NO_IOADDR: SceKernelErrorCode = 2147631364;
pub const SCE_KERNEL_ERROR_PHYMEM_ERROR: SceKernelErrorCode = 2147631872;
pub const SCE_KERNEL_ERROR_ILLEGAL_PHYPAGE_STATUS: SceKernelErrorCode = 2147631873;
pub const SCE_KERNEL_ERROR_NO_FREE_PHYSICAL_PAGE: SceKernelErrorCode = 2147631874;
pub const SCE_KERNEL_ERROR_NO_FREE_PHYSICAL_PAGE_UNIT: SceKernelErrorCode = 2147631875;
pub const SCE_KERNEL_ERROR_PHYMEMPART_NOT_EMPTY: SceKernelErrorCode = 2147631876;
pub const SCE_KERNEL_ERROR_NO_PHYMEMPART_LPDDR2: SceKernelErrorCode = 2147631877;
pub const SCE_KERNEL_ERROR_NO_PHYMEMPART_CDRAM: SceKernelErrorCode = 2147631878;
pub const SCE_KERNEL_ERROR_PHYMEMPART_OUT_OF_INDEX: SceKernelErrorCode = 2147631879;
pub const SCE_KERNEL_ERROR_CANNOT_GROW_PHYMEMPART: SceKernelErrorCode = 2147631880;
pub const SCE_KERNEL_ERROR_NO_FREE_PHYSICAL_PAGE_CDRAM: SceKernelErrorCode = 2147631881;
pub const SCE_KERNEL_ERROR_INVALID_SUBBUDGET_ID: SceKernelErrorCode = 2147631882;
pub const SCE_KERNEL_ERROR_FIXEDHEAP_ERROR: SceKernelErrorCode = 2147632128;
pub const SCE_KERNEL_ERROR_FIXEDHEAP_ILLEGAL_SIZE: SceKernelErrorCode = 2147632129;
pub const SCE_KERNEL_ERROR_FIXEDHEAP_ILLEGAL_INDEX: SceKernelErrorCode = 2147632130;
pub const SCE_KERNEL_ERROR_FIXEDHEAP_INDEX_OVERFLOW: SceKernelErrorCode = 2147632131;
pub const SCE_KERNEL_ERROR_FIXEDHEAP_NO_CHUNK: SceKernelErrorCode = 2147632132;
pub const SCE_KERNEL_ERROR_UID_ERROR: SceKernelErrorCode = 2147632384;
pub const SCE_KERNEL_ERROR_INVALID_UID: SceKernelErrorCode = 2147632385;
pub const SCE_KERNEL_ERROR_SYSMEM_UID_INVALID_ARGUMENT: SceKernelErrorCode = 2147632386;
pub const SCE_KERNEL_ERROR_SYSMEM_INVALID_UID_RANGE: SceKernelErrorCode = 2147632387;
pub const SCE_KERNEL_ERROR_SYSMEM_NO_VALID_UID: SceKernelErrorCode = 2147632388;
pub const SCE_KERNEL_ERROR_SYSMEM_CANNOT_ALLOCATE_UIDENTRY: SceKernelErrorCode = 2147632389;
pub const SCE_KERNEL_ERROR_NOT_PROCESS_UID: SceKernelErrorCode = 2147632390;
pub const SCE_KERNEL_ERROR_NOT_KERNEL_UID: SceKernelErrorCode = 2147632391;
pub const SCE_KERNEL_ERROR_INVALID_UID_CLASS: SceKernelErrorCode = 2147632392;
pub const SCE_KERNEL_ERROR_INVALID_UID_SUBCLASS: SceKernelErrorCode = 2147632393;
pub const SCE_KERNEL_ERROR_UID_CANNOT_FIND_BY_NAME: SceKernelErrorCode = 2147632394;
pub const SCE_KERNEL_ERROR_UID_NOT_VISIBLE: SceKernelErrorCode = 2147632395;
pub const SCE_KERNEL_ERROR_UID_MAX_OPEN: SceKernelErrorCode = 2147632396;
pub const SCE_KERNEL_ERROR_UID_RL_OVERFLOW: SceKernelErrorCode = 2147632397;
pub const SCE_KERNEL_ERROR_VIRPAGE_ERROR: SceKernelErrorCode = 2147632640;
pub const SCE_KERNEL_ERROR_ILLEGAL_VIRPAGE_TYPE: SceKernelErrorCode = 2147632641;
pub const SCE_KERNEL_ERROR_BLOCK_ERROR: SceKernelErrorCode = 2147632896;
pub const SCE_KERNEL_ERROR_ILLEGAL_BLOCK_ID: SceKernelErrorCode = 2147632897;
pub const SCE_KERNEL_ERROR_ILLEGAL_BLOCK_TYPE: SceKernelErrorCode = 2147632898;
pub const SCE_KERNEL_ERROR_BLOCK_IN_USE: SceKernelErrorCode = 2147632899;
pub const SCE_KERNEL_ERROR_PARTITION_ERROR: SceKernelErrorCode = 2147633152;
pub const SCE_KERNEL_ERROR_ILLEGAL_PARTITION_ID: SceKernelErrorCode = 2147633153;
pub const SCE_KERNEL_ERROR_ILLEGAL_PARTITION_INDEX: SceKernelErrorCode = 2147633154;
pub const SCE_KERNEL_ERROR_NO_L2PAGETABLE: SceKernelErrorCode = 2147633155;
pub const SCE_KERNEL_ERROR_HEAPLIB_ERROR: SceKernelErrorCode = 2147633408;
pub const SCE_KERNEL_ERROR_ILLEGAL_HEAP_ID: SceKernelErrorCode = 2147633409;
pub const SCE_KERNEL_ERROR_OUT_OF_RANG: SceKernelErrorCode = 2147633410;
pub const SCE_KERNEL_ERROR_HEAPLIB_NOMEM: SceKernelErrorCode = 2147633411;
pub const SCE_KERNEL_ERROR_HEAPLIB_VERIFY_ERROR: SceKernelErrorCode = 2147633412;
pub const SCE_KERNEL_ERROR_SYSMEM_ADDRESS_SPACE_ERROR: SceKernelErrorCode = 2147633664;
pub const SCE_KERNEL_ERROR_INVALID_ADDRESS_SPACE_ID: SceKernelErrorCode = 2147633665;
pub const SCE_KERNEL_ERROR_INVALID_PARTITION_INDEX: SceKernelErrorCode = 2147633666;
pub const SCE_KERNEL_ERROR_ADDRESS_SPACE_CANNOT_FIND_PARTITION_BY_ADDR: SceKernelErrorCode =
    2147633667;
pub const SCE_KERNEL_ERROR_SYSMEM_MEMBLOCK_ERROR: SceKernelErrorCode = 2147633920;
pub const SCE_KERNEL_ERROR_ILLEGAL_MEMBLOCK_TYPE: SceKernelErrorCode = 2147633921;
pub const SCE_KERNEL_ERROR_ILLEGAL_MEMBLOCK_REMAP_TYPE: SceKernelErrorCode = 2147633922;
pub const SCE_KERNEL_ERROR_NOT_PHY_CONT_MEMBLOCK: SceKernelErrorCode = 2147633923;
pub const SCE_KERNEL_ERROR_ILLEGAL_MEMBLOCK_CODE: SceKernelErrorCode = 2147633924;
pub const SCE_KERNEL_ERROR_ILLEGAL_MEMBLOCK_SIZE: SceKernelErrorCode = 2147633925;
pub const SCE_KERNEL_ERROR_ILLEGAL_USERMAP_SIZE: SceKernelErrorCode = 2147633926;
pub const SCE_KERNEL_ERROR_MEMBLOCK_TYPE_FOR_KERNEL_PROCESS: SceKernelErrorCode = 2147633927;
pub const SCE_KERNEL_ERROR_PROCESS_CANNOT_REMAP_MEMBLOCK: SceKernelErrorCode = 2147633928;
pub const SCE_KERNEL_ERROR_MEMBLOCK_RANGE_ERROR: SceKernelErrorCode = 2147633929;
pub const SCE_KERNEL_ERROR_MEMBLOCK_TYPE_FOR_UPDATER_OR_SAFEMODE: SceKernelErrorCode = 2147633930;
pub const SCE_KERNEL_ERROR_MEMBLOCK_OVERFLOW: SceKernelErrorCode = 2147633931;
pub const SCE_KERNEL_ERROR_SYSMEM_PHYMEMLOW_ERROR: SceKernelErrorCode = 2147634176;
pub const SCE_KERNEL_ERROR_CANNOT_ALLOC_PHYMEMLOW: SceKernelErrorCode = 2147634177;
pub const SCE_KERNEL_ERROR_UNKNOWN_PHYMEMLOW_TYPE: SceKernelErrorCode = 2147634178;
pub const SCE_KERNEL_ERROR_SYSMEM_BITHEAP_ERROR: SceKernelErrorCode = 2147634432;
pub const SCE_KERNEL_ERROR_CANNOT_ALLOC_BITHEAP: SceKernelErrorCode = 2147634433;
pub const SCE_KERNEL_ERROR_SYSMEM_NAMEHEAP_ERROR: SceKernelErrorCode = 2147634688;
pub const SCE_KERNEL_ERROR_NO_SUCH_NAME: SceKernelErrorCode = 2147634689;
pub const SCE_KERNEL_ERROR_DUPLICATE_NAME: SceKernelErrorCode = 2147634690;
pub const SCE_KERNEL_ERROR_LOADCORE_ERROR: SceKernelErrorCode = 2147635200;
pub const SCE_KERNEL_ERROR_ILLEGAL_ELF_HEADER: SceKernelErrorCode = 2147635201;
pub const SCE_KERNEL_ERROR_ILLEGAL_SELF_HEADER: SceKernelErrorCode = 2147635202;
pub const SCE_KERNEL_ERROR_EXCPMGR_ERROR: SceKernelErrorCode = 2147643392;
pub const SCE_KERNEL_ERROR_ILLEGAL_EXCPCODE: SceKernelErrorCode = 2147643393;
pub const SCE_KERNEL_ERROR_ILLEGAL_EXCPHANDLER: SceKernelErrorCode = 2147643394;
pub const SCE_KERNEL_ERROR_NOTFOUND_EXCPHANDLER: SceKernelErrorCode = 2147643395;
pub const SCE_KERNEL_ERROR_CANNOT_RELEASE_EXCPHANDLER: SceKernelErrorCode = 2147643396;
pub const SCE_KERNEL_ERROR_INTRMGR_ERROR: SceKernelErrorCode = 2147643648;
pub const SCE_KERNEL_ERROR_ILLEGAL_CONTEXT: SceKernelErrorCode = 2147643649;
pub const SCE_KERNEL_ERROR_ILLEGAL_INTRCODE: SceKernelErrorCode = 2147643650;
pub const SCE_KERNEL_ERROR_ILLEGAL_INTRPARAM: SceKernelErrorCode = 2147643651;
pub const SCE_KERNEL_ERROR_ILLEGAL_INTRPRIORITY: SceKernelErrorCode = 2147643652;
pub const SCE_KERNEL_ERROR_ILLEGAL_TARGET_CPU: SceKernelErrorCode = 2147643653;
pub const SCE_KERNEL_ERROR_ILLEGAL_INTRFILTER: SceKernelErrorCode = 2147643654;
pub const SCE_KERNEL_ERROR_ILLEGAL_INTRTYPE: SceKernelErrorCode = 2147643655;
pub const SCE_KERNEL_ERROR_ILLEGAL_HANDLER: SceKernelErrorCode = 2147643656;
pub const SCE_KERNEL_ERROR_FOUND_HANDLER: SceKernelErrorCode = 2147643657;
pub const SCE_KERNEL_ERROR_NOTFOUND_HANDLER: SceKernelErrorCode = 2147643658;
pub const SCE_KERNEL_ERROR_NO_MEMORY: SceKernelErrorCode = 2147643659;
pub const SCE_KERNEL_ERROR_DMACMGR_ERROR: SceKernelErrorCode = 2147643904;
pub const SCE_KERNEL_ERROR_ALREADY_QUEUED: SceKernelErrorCode = 2147643905;
pub const SCE_KERNEL_ERROR_NOT_QUEUED: SceKernelErrorCode = 2147643906;
pub const SCE_KERNEL_ERROR_NOT_SETUP: SceKernelErrorCode = 2147643907;
pub const SCE_KERNEL_ERROR_ON_TRANSFERRING: SceKernelErrorCode = 2147643908;
pub const SCE_KERNEL_ERROR_NOT_INITIALIZED: SceKernelErrorCode = 2147643909;
pub const SCE_KERNEL_ERROR_TRANSFERRED: SceKernelErrorCode = 2147643910;
pub const SCE_KERNEL_ERROR_NOT_UNDER_CONTROL: SceKernelErrorCode = 2147643911;
pub const SCE_KERNEL_ERROR_CANCELING: SceKernelErrorCode = 2147643912;
pub const SCE_KERNEL_ERROR_SYSTIMER_ERROR: SceKernelErrorCode = 2147644160;
pub const SCE_KERNEL_ERROR_NO_FREE_TIMER: SceKernelErrorCode = 2147644161;
pub const SCE_KERNEL_ERROR_TIMER_NOT_ALLOCATED: SceKernelErrorCode = 2147644162;
pub const SCE_KERNEL_ERROR_TIMER_COUNTING: SceKernelErrorCode = 2147644163;
pub const SCE_KERNEL_ERROR_TIMER_STOPPED: SceKernelErrorCode = 2147644164;
pub const SCE_KERNEL_ERROR_THREADMGR_ERROR: SceKernelErrorCode = 2147647488;
pub const SCE_KERNEL_ERROR_UNKNOWN_UID: SceKernelErrorCode = 2147647489;
pub const SCE_KERNEL_ERROR_DIFFERENT_UID_CLASS: SceKernelErrorCode = 2147647490;
pub const SCE_KERNEL_ERROR_ALREADY_REGISTERED: SceKernelErrorCode = 2147647491;
pub const SCE_KERNEL_ERROR_CAN_NOT_WAIT: SceKernelErrorCode = 2147647492;
pub const SCE_KERNEL_ERROR_WAIT_TIMEOUT: SceKernelErrorCode = 2147647493;
pub const SCE_KERNEL_ERROR_WAIT_DELETE: SceKernelErrorCode = 2147647494;
pub const SCE_KERNEL_ERROR_WAIT_CANCEL: SceKernelErrorCode = 2147647495;
pub const SCE_KERNEL_ERROR_THREAD_ERROR: SceKernelErrorCode = 2147647520;
pub const SCE_KERNEL_ERROR_UNKNOWN_THREAD_ID: SceKernelErrorCode = 2147647521;
pub const SCE_KERNEL_ERROR_ILLEGAL_THREAD_ID: SceKernelErrorCode = 2147647522;
pub const SCE_KERNEL_ERROR_ILLEGAL_PRIORITY: SceKernelErrorCode = 2147647523;
pub const SCE_KERNEL_ERROR_ILLEGAL_STACK_SIZE: SceKernelErrorCode = 2147647524;
pub const SCE_KERNEL_ERROR_ILLEGAL_CPU_AFFINITY_MASK: SceKernelErrorCode = 2147647525;
pub const SCE_KERNEL_ERROR_ILLEGAL_THREAD_PARAM_COMBINATION: SceKernelErrorCode = 2147647526;
pub const SCE_KERNEL_ERROR_DORMANT: SceKernelErrorCode = 2147647527;
pub const SCE_KERNEL_ERROR_NOT_DORMANT: SceKernelErrorCode = 2147647528;
pub const SCE_KERNEL_ERROR_RUNNING: SceKernelErrorCode = 2147647529;
pub const SCE_KERNEL_ERROR_DELETED: SceKernelErrorCode = 2147647530;
pub const SCE_KERNEL_ERROR_CAN_NOT_SUSPEND: SceKernelErrorCode = 2147647531;
pub const SCE_KERNEL_ERROR_THREAD_STOPPED: SceKernelErrorCode = 2147647532;
pub const SCE_KERNEL_ERROR_THREAD_SUSPENDED: SceKernelErrorCode = 2147647533;
pub const SCE_KERNEL_ERROR_NOT_SUSPENDED: SceKernelErrorCode = 2147647534;
pub const SCE_KERNEL_ERROR_ALREADY_DEBUG_SUSPENDED: SceKernelErrorCode = 2147647535;
pub const SCE_KERNEL_ERROR_NOT_DEBUG_SUSPENDED: SceKernelErrorCode = 2147647536;
pub const SCE_KERNEL_ERROR_CAN_NOT_USE_VFP: SceKernelErrorCode = 2147647537;
pub const SCE_KERNEL_ERROR_THREAD_EVENT_ERROR: SceKernelErrorCode = 2147647584;
pub const SCE_KERNEL_ERROR_UNKNOWN_THREAD_EVENT_ID: SceKernelErrorCode = 2147647585;
pub const SCE_KERNEL_ERROR_KERNEL_TLS_ERROR: SceKernelErrorCode = 2147647616;
pub const SCE_KERNEL_ERROR_KERNEL_TLS_FULL: SceKernelErrorCode = 2147647617;
pub const SCE_KERNEL_ERROR_ILLEGAL_KERNEL_TLS_INDEX: SceKernelErrorCode = 2147647618;
pub const SCE_KERNEL_ERROR_KERNEL_TLS_BUSY: SceKernelErrorCode = 2147647619;
pub const SCE_KERNEL_ERROR_CALLBACK_ERROR: SceKernelErrorCode = 2147647648;
pub const SCE_KERNEL_ERROR_UNKNOWN_CALLBACK_ID: SceKernelErrorCode = 2147647649;
pub const SCE_KERNEL_ERROR_NOTIFY_CALLBACK: SceKernelErrorCode = 2147647650;
pub const SCE_KERNEL_ERROR_CALLBACK_NOT_REGISTERED: SceKernelErrorCode = 2147647651;
pub const SCE_KERNEL_ERROR_ALARM_ERROR: SceKernelErrorCode = 2147647680;
pub const SCE_KERNEL_ERROR_UNKNOWN_ALARM_ID: SceKernelErrorCode = 2147647681;
pub const SCE_KERNEL_ERROR_ALARM_CAN_NOT_CANCEL: SceKernelErrorCode = 2147647682;
pub const SCE_KERNEL_ERROR_EVF_ERROR: SceKernelErrorCode = 2147647712;
pub const SCE_KERNEL_ERROR_UNKNOWN_EVF_ID: SceKernelErrorCode = 2147647713;
pub const SCE_KERNEL_ERROR_EVF_MULTI: SceKernelErrorCode = 2147647714;
pub const SCE_KERNEL_ERROR_EVF_COND: SceKernelErrorCode = 2147647715;
pub const SCE_KERNEL_ERROR_SEMA_ERROR: SceKernelErrorCode = 2147647744;
pub const SCE_KERNEL_ERROR_UNKNOWN_SEMA_ID: SceKernelErrorCode = 2147647745;
pub const SCE_KERNEL_ERROR_SEMA_ZERO: SceKernelErrorCode = 2147647746;
pub const SCE_KERNEL_ERROR_SEMA_OVF: SceKernelErrorCode = 2147647747;
pub const SCE_KERNEL_ERROR_SIGNAL_ERROR: SceKernelErrorCode = 2147647776;
pub const SCE_KERNEL_ERROR_ALREADY_SENT: SceKernelErrorCode = 2147647777;
pub const SCE_KERNEL_ERROR_MUTEX_ERROR: SceKernelErrorCode = 2147647808;
pub const SCE_KERNEL_ERROR_UNKNOWN_MUTEX_ID: SceKernelErrorCode = 2147647809;
pub const SCE_KERNEL_ERROR_MUTEX_RECURSIVE: SceKernelErrorCode = 2147647810;
pub const SCE_KERNEL_ERROR_MUTEX_LOCK_OVF: SceKernelErrorCode = 2147647811;
pub const SCE_KERNEL_ERROR_MUTEX_UNLOCK_UDF: SceKernelErrorCode = 2147647812;
pub const SCE_KERNEL_ERROR_MUTEX_FAILED_TO_OWN: SceKernelErrorCode = 2147647813;
pub const SCE_KERNEL_ERROR_MUTEX_NOT_OWNED: SceKernelErrorCode = 2147647814;
pub const SCE_KERNEL_ERROR_FAST_MUTEX_ERROR: SceKernelErrorCode = 2147647840;
pub const SCE_KERNEL_ERROR_UNKNOWN_FAST_MUTEX_ID: SceKernelErrorCode = 2147647841;
pub const SCE_KERNEL_ERROR_FAST_MUTEX_RECURSIVE: SceKernelErrorCode = 2147647842;
pub const SCE_KERNEL_ERROR_FAST_MUTEX_LOCK_OVF: SceKernelErrorCode = 2147647843;
pub const SCE_KERNEL_ERROR_FAST_MUTEX_FAILED_TO_OWN: SceKernelErrorCode = 2147647844;
pub const SCE_KERNEL_ERROR_FAST_MUTEX_NOT_OWNED: SceKernelErrorCode = 2147647845;
pub const SCE_KERNEL_ERROR_FAST_MUTEX_OWNED: SceKernelErrorCode = 2147647846;
pub const SCE_KERNEL_ERROR_FAST_MUTEX_ALREADY_INITIALIZED: SceKernelErrorCode = 2147647847;
pub const SCE_KERNEL_ERROR_FAST_MUTEX_NOT_INITIALIZED: SceKernelErrorCode = 2147647848;
pub const SCE_KERNEL_ERROR_LW_MUTEX_ERROR: SceKernelErrorCode = 2147647872;
pub const SCE_KERNEL_ERROR_UNKNOWN_LW_MUTEX_ID: SceKernelErrorCode = 2147647873;
pub const SCE_KERNEL_ERROR_LW_MUTEX_RECURSIVE: SceKernelErrorCode = 2147647874;
pub const SCE_KERNEL_ERROR_LW_MUTEX_LOCK_OVF: SceKernelErrorCode = 2147647875;
pub const SCE_KERNEL_ERROR_LW_MUTEX_UNLOCK_UDF: SceKernelErrorCode = 2147647876;
pub const SCE_KERNEL_ERROR_LW_MUTEX_FAILED_TO_OWN: SceKernelErrorCode = 2147647877;
pub const SCE_KERNEL_ERROR_LW_MUTEX_NOT_OWNED: SceKernelErrorCode = 2147647878;
pub const SCE_KERNEL_ERROR_COND_ERROR: SceKernelErrorCode = 2147647904;
pub const SCE_KERNEL_ERROR_UNKNOWN_COND_ID: SceKernelErrorCode = 2147647905;
pub const SCE_KERNEL_ERROR_WAIT_DELETE_MUTEX: SceKernelErrorCode = 2147647906;
pub const SCE_KERNEL_ERROR_WAIT_CANCEL_MUTEX: SceKernelErrorCode = 2147647907;
pub const SCE_KERNEL_ERROR_WAIT_DELETE_COND: SceKernelErrorCode = 2147647908;
pub const SCE_KERNEL_ERROR_WAIT_CANCEL_COND: SceKernelErrorCode = 2147647909;
pub const SCE_KERNEL_ERROR_LW_COND_ERROR: SceKernelErrorCode = 2147647936;
pub const SCE_KERNEL_ERROR_UNKNOWN_LW_COND_ID: SceKernelErrorCode = 2147647937;
pub const SCE_KERNEL_ERROR_WAIT_DELETE_LW_MUTEX: SceKernelErrorCode = 2147647938;
pub const SCE_KERNEL_ERROR_WAIT_DELETE_LW_COND: SceKernelErrorCode = 2147647939;
pub const SCE_KERNEL_ERROR_RW_LOCK_ERROR: SceKernelErrorCode = 2147647968;
pub const SCE_KERNEL_ERROR_UNKNOWN_RW_LOCK_ID: SceKernelErrorCode = 2147647969;
pub const SCE_KERNEL_ERROR_RW_LOCK_RECURSIVE: SceKernelErrorCode = 2147647970;
pub const SCE_KERNEL_ERROR_RW_LOCK_LOCK_OVF: SceKernelErrorCode = 2147647971;
pub const SCE_KERNEL_ERROR_RW_LOCK_NOT_OWNED: SceKernelErrorCode = 2147647972;
pub const SCE_KERNEL_ERROR_RW_LOCK_UNLOCK_UDF: SceKernelErrorCode = 2147647973;
pub const SCE_KERNEL_ERROR_RW_LOCK_FAILED_TO_LOCK: SceKernelErrorCode = 2147647974;
pub const SCE_KERNEL_ERROR_RW_LOCK_FAILED_TO_UNLOCK: SceKernelErrorCode = 2147647975;
pub const SCE_KERNEL_ERROR_EVENT_ERROR: SceKernelErrorCode = 2147648000;
pub const SCE_KERNEL_ERROR_UNKNOWN_EVENT_ID: SceKernelErrorCode = 2147648001;
pub const SCE_KERNEL_ERROR_EVENT_COND: SceKernelErrorCode = 2147648002;
pub const SCE_KERNEL_ERROR_MSG_PIPE_ERROR: SceKernelErrorCode = 2147648032;
pub const SCE_KERNEL_ERROR_UNKNOWN_MSG_PIPE_ID: SceKernelErrorCode = 2147648033;
pub const SCE_KERNEL_ERROR_MSG_PIPE_FULL: SceKernelErrorCode = 2147648034;
pub const SCE_KERNEL_ERROR_MSG_PIPE_EMPTY: SceKernelErrorCode = 2147648035;
pub const SCE_KERNEL_ERROR_MSG_PIPE_DELETED: SceKernelErrorCode = 2147648036;
pub const SCE_KERNEL_ERROR_TIMER_ERROR: SceKernelErrorCode = 2147648064;
pub const SCE_KERNEL_ERROR_UNKNOWN_TIMER_ID: SceKernelErrorCode = 2147648065;
pub const SCE_KERNEL_ERROR_EVENT_NOT_SET: SceKernelErrorCode = 2147648066;
pub const SCE_KERNEL_ERROR_SIMPLE_EVENT_ERROR: SceKernelErrorCode = 2147648096;
pub const SCE_KERNEL_ERROR_UNKNOWN_SIMPLE_EVENT_ID: SceKernelErrorCode = 2147648097;
pub const SCE_KERNEL_ERROR_PMON_ERROR: SceKernelErrorCode = 2147648128;
pub const SCE_KERNEL_ERROR_PMON_NOT_THREAD_MODE: SceKernelErrorCode = 2147648129;
pub const SCE_KERNEL_ERROR_PMON_NOT_CPU_MODE: SceKernelErrorCode = 2147648130;
pub const SCE_KERNEL_ERROR_WORK_QUEUE: SceKernelErrorCode = 2147648256;
pub const SCE_KERNEL_ERROR_UNKNOWN_WORK_QUEUE_ID: SceKernelErrorCode = 2147648257;
pub const SCE_KERNEL_ERROR_UNKNOWN_WORK_TASK_ID: SceKernelErrorCode = 2147648258;
pub const SCE_KERNEL_ERROR_PROCESSMGR_ERROR: SceKernelErrorCode = 2147651584;
pub const SCE_KERNEL_ERROR_INVALID_PID: SceKernelErrorCode = 2147651585;
pub const SCE_KERNEL_ERROR_INVALID_PROCESS_TYPE: SceKernelErrorCode = 2147651586;
pub const SCE_KERNEL_ERROR_PLS_FULL: SceKernelErrorCode = 2147651587;
pub const SCE_KERNEL_ERROR_INVALID_PROCESS_STATUS: SceKernelErrorCode = 2147651588;
pub const SCE_KERNEL_ERROR_PROCESS_CALLBACK_NOTFOUND: SceKernelErrorCode = 2147651589;
pub const SCE_KERNEL_ERROR_INVALID_BUDGET_ID: SceKernelErrorCode = 2147651590;
pub const SCE_KERNEL_ERROR_INVALID_BUDGET_SIZE: SceKernelErrorCode = 2147651591;
pub const SCE_KERNEL_ERROR_CP14_DISABLED: SceKernelErrorCode = 2147651592;
pub const SCE_KERNEL_ERROR_EXCEEDED_MAX_PROCESSES: SceKernelErrorCode = 2147651593;
pub const SCE_KERNEL_ERROR_PROCESS_REMAINING: SceKernelErrorCode = 2147651594;
pub const SCE_KERNEL_ERROR_NO_PROCESS_DATA: SceKernelErrorCode = 2147651595;
pub const SCE_KERNEL_ERROR_PROCESS_EVENT_INHIBITED: SceKernelErrorCode = 2147651596;
pub const SCE_KERNEL_ERROR_IOFILEMGR_ERROR: SceKernelErrorCode = 2147655680;
pub const SCE_KERNEL_ERROR_IO_NAME_TOO_LONG: SceKernelErrorCode = 2147655681;
pub const SCE_KERNEL_ERROR_IO_REG_DEV: SceKernelErrorCode = 2147655682;
pub const SCE_KERNEL_ERROR_IO_ALIAS_USED: SceKernelErrorCode = 2147655683;
pub const SCE_KERNEL_ERROR_IO_DEL_DEV: SceKernelErrorCode = 2147655684;
pub const SCE_KERNEL_ERROR_IO_WOULD_BLOCK: SceKernelErrorCode = 2147655685;
pub const SCE_KERNEL_ERROR_MODULEMGR_START_FAILED: SceKernelErrorCode = 2147667968;
pub const SCE_KERNEL_ERROR_MODULEMGR_STOP_FAIL: SceKernelErrorCode = 2147667969;
pub const SCE_KERNEL_ERROR_MODULEMGR_IN_USE: SceKernelErrorCode = 2147667970;
pub const SCE_KERNEL_ERROR_MODULEMGR_NO_LIB: SceKernelErrorCode = 2147667971;
pub const SCE_KERNEL_ERROR_MODULEMGR_SYSCALL_REG: SceKernelErrorCode = 2147667972;
pub const SCE_KERNEL_ERROR_MODULEMGR_NOMEM_LIB: SceKernelErrorCode = 2147667973;
pub const SCE_KERNEL_ERROR_MODULEMGR_NOMEM_STUB: SceKernelErrorCode = 2147667974;
pub const SCE_KERNEL_ERROR_MODULEMGR_NOMEM_SELF: SceKernelErrorCode = 2147667975;
pub const SCE_KERNEL_ERROR_MODULEMGR_NOMEM: SceKernelErrorCode = 2147667976;
pub const SCE_KERNEL_ERROR_MODULEMGR_INVALID_LIB: SceKernelErrorCode = 2147667977;
pub const SCE_KERNEL_ERROR_MODULEMGR_INVALID_STUB: SceKernelErrorCode = 2147667978;
pub const SCE_KERNEL_ERROR_MODULEMGR_NO_FUNC_NID: SceKernelErrorCode = 2147667979;
pub const SCE_KERNEL_ERROR_MODULEMGR_NO_VAR_NID: SceKernelErrorCode = 2147667980;
pub const SCE_KERNEL_ERROR_MODULEMGR_INVALID_TYPE: SceKernelErrorCode = 2147667981;
pub const SCE_KERNEL_ERROR_MODULEMGR_NO_MOD_ENTRY: SceKernelErrorCode = 2147667982;
pub const SCE_KERNEL_ERROR_MODULEMGR_INVALID_PROC_PARAM: SceKernelErrorCode = 2147667983;
pub const SCE_KERNEL_ERROR_MODULEMGR_NO_MODOBJ: SceKernelErrorCode = 2147667984;
pub const SCE_KERNEL_ERROR_MODULEMGR_NO_MOD: SceKernelErrorCode = 2147667985;
pub const SCE_KERNEL_ERROR_MODULEMGR_NO_PROCESS: SceKernelErrorCode = 2147667986;
pub const SCE_KERNEL_ERROR_MODULEMGR_OLD_LIB: SceKernelErrorCode = 2147667987;
pub const SCE_KERNEL_ERROR_MODULEMGR_STARTED: SceKernelErrorCode = 2147667988;
pub const SCE_KERNEL_ERROR_MODULEMGR_NOT_STARTED: SceKernelErrorCode = 2147667989;
pub const SCE_KERNEL_ERROR_MODULEMGR_NOT_STOPPED: SceKernelErrorCode = 2147667990;
pub const SCE_KERNEL_ERROR_MODULEMGR_INVALID_PROCESS_UID: SceKernelErrorCode = 2147667991;
pub const SCE_KERNEL_ERROR_MODULEMGR_CANNOT_EXPORT_LIB_TO_SHARED: SceKernelErrorCode = 2147667992;
pub const SCE_KERNEL_ERROR_MODULEMGR_INVALID_REL_INFO: SceKernelErrorCode = 2147667993;
pub const SCE_KERNEL_ERROR_MODULEMGR_INVALID_REF_INFO: SceKernelErrorCode = 2147667994;
pub const SCE_KERNEL_ERROR_MODULEMGR_ELINK: SceKernelErrorCode = 2147667995;
pub const SCE_KERNEL_ERROR_MODULEMGR_NOENT: SceKernelErrorCode = 2147667996;
pub const SCE_KERNEL_ERROR_MODULEMGR_BUSY: SceKernelErrorCode = 2147667997;
pub const SCE_KERNEL_ERROR_MODULEMGR_NOEXEC: SceKernelErrorCode = 2147667998;
pub const SCE_KERNEL_ERROR_MODULEMGR_NAMETOOLONG: SceKernelErrorCode = 2147667999;
pub const SCE_KERNEL_ERROR_LIBRARYDB_NOENT: SceKernelErrorCode = 2147668096;
pub const SCE_KERNEL_ERROR_LIBRARYDB_NO_LIB: SceKernelErrorCode = 2147668097;
pub const SCE_KERNEL_ERROR_LIBRARYDB_NO_MOD: SceKernelErrorCode = 2147668098;
pub const SCE_KERNEL_ERROR_PRELOAD_FAILED: SceKernelErrorCode = 2147668208;
pub const SCE_KERNEL_ERROR_PRELOAD_LIBC_FAILED: SceKernelErrorCode = 2147668209;
pub const SCE_KERNEL_ERROR_PRELOAD_FIOS2_FAILED: SceKernelErrorCode = 2147668210;
pub const SCE_KERNEL_ERROR_AUTHFAIL: SceKernelErrorCode = 2147676160;
pub const SCE_KERNEL_ERROR_NO_AUTH: SceKernelErrorCode = 2147676161;

s! {
    pub struct FILE {
        data: [u64; 32],
    }

    pub struct fpos_t {
        data: [u64; 2],
    }

    pub struct SceKernelAllocMemBlockOpt {
        pub size: SceSize,
        pub attr: SceKernelAllocMemBlockAttr,
        pub alignment: SceSize,
        pub uidBaseBlock: SceUID,
        pub strBaseBlockName: *const c_char,
        /// Unknown flags 0x10 or 0x30 for ::sceKernelOpenMemBlock
        pub flags: c_int,
        pub reserved: [c_int; 10],
    }

    pub struct SceKernelMemBlockInfo {
        pub size: SceSize,
        pub mappedBase: *mut c_void,
        pub mappedSize: SceSize,
        pub memoryType: c_int,
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
        pub usec: i32,
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
        data: [SceInt64; 4],
    }

    pub struct SceKernelLwMutexOptParam {
        pub size: SceSize,
    }

    pub struct SceKernelLwCondWork {
        data: [SceInt32; 4],
    }

    pub struct SceKernelLwCondOptParam {
        pub size: SceSize,
    }

    pub struct SceDisplayFrameBuf {
        pub size: SceSize,
        pub base: *mut c_void,
        pub pitch: c_uint,
        pub pixelformat: SceDisplayPixelFormat,
        pub width: c_uint,
        pub height: c_uint,
    }
}

impl fpos_t {
    pub const fn zeroed() -> Self {
        fpos_t { data: [0; 2] }
    }
}

impl SceKernelLwMutexWork {
    pub const fn zeroed() -> Self {
        SceKernelLwMutexWork { data: [0; 4] }
    }
}

impl SceKernelLwCondWork {
    pub const fn zeroed() -> Self {
        SceKernelLwCondWork { data: [0; 4] }
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
    pub static mut stderr: FILE;
    #[link_name = "_Stdin"]
    pub static mut stdin: FILE;
    #[link_name = "_Stdout"]
    pub static mut stdout: FILE;
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
    pub fn printf(format: *const c_char, ...) -> c_int;
    pub fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
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
        attr: SceKernelMutexAttribute,
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

    pub fn sceDisplaySetFrameBuf(
        pParam: *const SceDisplayFrameBuf,
        sync: SceDisplaySetBufSync,
    ) -> c_int;
    pub fn sceDisplayGetFrameBuf(
        pParam: *mut SceDisplayFrameBuf,
        sync: SceDisplaySetBufSync,
    ) -> c_int;
    pub fn sceDisplayGetPrimaryHead() -> c_int;
    pub fn sceDisplayGetRefreshRate(pFps: *mut f32) -> c_int;
    pub fn sceDisplayGetMaximumFrameBufResolution(width: *mut c_int, height: *mut c_int) -> c_int;
    pub fn sceDisplayGetVcount() -> c_int;
    pub fn sceDisplayGetVcountInternal(display: c_int) -> c_int;
    pub fn sceDisplayWaitVblankStart() -> c_int;
    pub fn sceDisplayWaitVblankStartCB() -> c_int;
    pub fn sceDisplayWaitVblankStartMulti(vcount: c_uint) -> c_int;
    pub fn sceDisplayWaitVblankStartMultiCB(vcount: c_uint) -> c_int;
    pub fn sceDisplayWaitSetFrameBuf() -> c_int;
    pub fn sceDisplayWaitSetFrameBufCB() -> c_int;
    pub fn sceDisplayWaitSetFrameBufMulti(vcount: c_uint) -> c_int;
    pub fn sceDisplayWaitSetFrameBufMultiCB(vcount: c_uint) -> c_int;
    pub fn sceDisplayRegisterVblankStartCallback(uid: SceUID) -> c_int;
    pub fn sceDisplayUnregisterVblankStartCallback(uid: SceUID) -> c_int;
}
