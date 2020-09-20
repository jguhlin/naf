#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, const_raw_ptr_to_usize_cast, extern_types, main,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    /*= Decompression context
 *  When decompressing many times,
 *  it is recommended to allocate a context only once,
 *  and re-use it for each successive compression operation.
 *  This will make workload friendlier for system's memory.
 *  Use one context per thread for parallel execution. */
    pub type ZSTD_DCtx_s;
    #[no_mangle]
    static mut stdin: *mut FILE;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char)
     -> *mut FILE;
    #[no_mangle]
    fn freopen(__filename: *const libc::c_char, __modes: *const libc::c_char,
               __stream: *mut FILE) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::std::ffi::VaList)
     -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn strtoll(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char,
               __base: libc::c_int) -> libc::c_longlong;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    #[no_mangle]
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t,
             __stream: *mut FILE) -> size_t;
    #[no_mangle]
    fn fwrite(__ptr: *const libc::c_void, __size: size_t, __n: size_t,
              __s: *mut FILE) -> size_t;
    #[no_mangle]
    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fchown(__fd: libc::c_int, __owner: __uid_t, __group: __gid_t)
     -> libc::c_int;
    #[no_mangle]
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn __fxstat(__ver: libc::c_int, __fildes: libc::c_int,
                __stat_buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn fchmod(__fd: libc::c_int, __mode: __mode_t) -> libc::c_int;
    #[no_mangle]
    fn futimens(__fd: libc::c_int, __times: *const timespec) -> libc::c_int;
    /* *< to check runtime library version */
    #[no_mangle]
    fn ZSTD_versionString() -> *const libc::c_char;
    /* ! ZSTD_decompress() :
 *  `compressedSize` : must be the _exact_ size of some number of compressed and/or skippable frames.
 *  `dstCapacity` is an upper bound of originalSize to regenerate.
 *  If user cannot imply a maximum upper bound, it's better to use streaming mode to decompress data.
 *  @return : the number of bytes decompressed into `dst` (<= `dstCapacity`),
 *            or an errorCode if it fails (which can be tested using ZSTD_isError()). */
    #[no_mangle]
    fn ZSTD_decompress(dst: *mut libc::c_void, dstCapacity: size_t,
                       src: *const libc::c_void, compressedSize: size_t)
     -> size_t;
    /* !< maximum compressed size in worst case single-pass scenario */
    #[no_mangle]
    fn ZSTD_isError(code: size_t) -> libc::c_uint;
    /* !< tells if a `size_t` function result is an error code */
    #[no_mangle]
    fn ZSTD_getErrorName(code: size_t) -> *const libc::c_char;
    /* *< DCtx and DStream are now effectively same object (>= v1.3.0) */
                                 /* For compatibility with versions <= v1.2.0, prefer differentiating them. */
    /*===== ZSTD_DStream management functions =====*/
    #[no_mangle]
    fn ZSTD_createDStream() -> *mut ZSTD_DStream;
    /*===== Streaming decompression functions =====*/
    /* This function is redundant with the advanced API and equivalent to:
 *
 *     ZSTD_DCtx_reset(zds, ZSTD_reset_session_only);
 *     ZSTD_DCtx_refDDict(zds, NULL);
 */
    #[no_mangle]
    fn ZSTD_initDStream(zds: *mut ZSTD_DStream) -> size_t;
    #[no_mangle]
    fn ZSTD_decompressStream(zds: *mut ZSTD_DStream,
                             output: *mut ZSTD_outBuffer,
                             input: *mut ZSTD_inBuffer) -> size_t;
    #[no_mangle]
    fn ZSTD_DStreamInSize() -> size_t;
    /* !< recommended size for input buffer */
    #[no_mangle]
    fn ZSTD_DStreamOutSize() -> size_t;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __int32_t = libc::c_int;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type ZSTD_DCtx = ZSTD_DCtx_s;
/* ***************************
*  Streaming
****************************/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_inBuffer_s {
    pub src: *const libc::c_void,
    pub size: size_t,
    pub pos: size_t,
}
pub type ZSTD_inBuffer = ZSTD_inBuffer_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_outBuffer_s {
    pub dst: *mut libc::c_void,
    pub size: size_t,
    pub pos: size_t,
}
pub type ZSTD_outBuffer = ZSTD_outBuffer_s;
/* *< position where reading stopped. Will be updated. Necessarily 0 <= pos <= size */
/* *< position where writing stopped. Will be updated. Necessarily 0 <= pos <= size */
/*-***************************************************************************
*  Streaming decompression - HowTo
*
*  A ZSTD_DStream object is required to track streaming operations.
*  Use ZSTD_createDStream() and ZSTD_freeDStream() to create/release resources.
*  ZSTD_DStream objects can be re-used multiple times.
*
*  Use ZSTD_initDStream() to start a new decompression operation.
* @return : recommended first input size
*  Alternatively, use advanced API to set specific properties.
*
*  Use ZSTD_decompressStream() repetitively to consume your input.
*  The function will update both `pos` fields.
*  If `input.pos < input.size`, some input has not been consumed.
*  It's up to the caller to present again remaining data.
*  The function tries to flush all data decoded immediately, respecting output buffer size.
*  If `output.pos < output.size`, decoder has flushed everything it could.
*  But if `output.pos == output.size`, there might be some data left within internal buffers.,
*  In which case, call ZSTD_decompressStream() again to flush whatever remains in the buffer.
*  Note : with no additional input provided, amount of data flushed is necessarily <= ZSTD_BLOCKSIZE_MAX.
* @return : 0 when a frame is completely decoded and fully flushed,
*        or an error code, which can be tested using ZSTD_isError(),
*        or any other value > 0, which means there is still some decoding or flushing to do to complete current frame :
*                                the return value is a suggested next input size (just a hint for better latency)
*                                that will never request more than the remaining frame size.
* *******************************************************************************/
pub type ZSTD_DStream = ZSTD_DCtx;
/* ! Custom memory allocation :
 *  These prototypes make it possible to pass your own allocation/free functions.
 *  ZSTD_customMem is provided at creation time, using ZSTD_create*_advanced() variants listed below.
 *  All allocation/free operations will be completed using these custom variants instead of regular <stdlib.h> ones.
 */
pub type ZSTD_allocFunction
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: size_t)
               -> *mut libc::c_void>;
pub type ZSTD_freeFunction
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void)
               -> ()>;
pub type OUTPUT_TYPE = libc::c_uint;
pub const FASTQ: OUTPUT_TYPE = 22;
pub const UNMASKED_FASTA: OUTPUT_TYPE = 21;
pub const MASKED_FASTA: OUTPUT_TYPE = 20;
pub const FASTA: OUTPUT_TYPE = 19;
pub const CHARCOUNT: OUTPUT_TYPE = 18;
pub const SEQUENCES: OUTPUT_TYPE = 17;
pub const SEQ: OUTPUT_TYPE = 16;
pub const UNMASKED_DNA: OUTPUT_TYPE = 15;
pub const MASKED_DNA: OUTPUT_TYPE = 14;
pub const DNA: OUTPUT_TYPE = 13;
pub const FOUR_BIT: OUTPUT_TYPE = 12;
pub const TOTAL_MASK_LENGTH: OUTPUT_TYPE = 11;
pub const MASK: OUTPUT_TYPE = 10;
pub const TOTAL_LENGTH: OUTPUT_TYPE = 9;
pub const LENGTHS: OUTPUT_TYPE = 8;
pub const NAMES: OUTPUT_TYPE = 7;
pub const IDS: OUTPUT_TYPE = 6;
pub const TITLE: OUTPUT_TYPE = 5;
pub const NUMBER_OF_SEQUENCES: OUTPUT_TYPE = 4;
pub const PART_SIZES: OUTPUT_TYPE = 3;
pub const PART_LIST: OUTPUT_TYPE = 2;
pub const FORMAT_NAME: OUTPUT_TYPE = 1;
pub const UNDECIDED: OUTPUT_TYPE = 0;
pub type C2RustUnnamed = libc::c_uint;
pub const seq_type_text: C2RustUnnamed = 3;
pub const seq_type_protein: C2RustUnnamed = 2;
pub const seq_type_rna: C2RustUnnamed = 1;
pub const seq_type_dna: C2RustUnnamed = 0;
/*
 * NAF decompressor
 * Copyright (c) 2018-2020 Kirill Kryukov
 * See README.md and LICENSE files of this repository
 */
pub type C2RustUnnamed_0 = libc::c_uint;
pub const AHEAD_BUFFER_SIZE: C2RustUnnamed_0 = 16384;
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
               *(*__ctype_toupper_loc()).offset(__c as isize)
           } else { __c };
}
#[inline]
unsafe extern "C" fn fstat(mut __fd: libc::c_int, mut __statbuf: *mut stat)
 -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
/*
 * NAF decompressor
 * Copyright (c) 2018-2020 Kirill Kryukov
 * See README.md and LICENSE files of this repository
 */
static mut code_to_nuc: [libc::c_uchar; 16] =
    ['-' as i32 as libc::c_uchar, 'T' as i32 as libc::c_uchar,
     'G' as i32 as libc::c_uchar, 'K' as i32 as libc::c_uchar,
     'C' as i32 as libc::c_uchar, 'Y' as i32 as libc::c_uchar,
     'S' as i32 as libc::c_uchar, 'B' as i32 as libc::c_uchar,
     'A' as i32 as libc::c_uchar, 'W' as i32 as libc::c_uchar,
     'R' as i32 as libc::c_uchar, 'D' as i32 as libc::c_uchar,
     'M' as i32 as libc::c_uchar, 'H' as i32 as libc::c_uchar,
     'V' as i32 as libc::c_uchar, 'N' as i32 as libc::c_uchar];
static mut codes_to_nucs: [libc::c_ushort; 256] = [0; 256];
static mut out_type: OUTPUT_TYPE = UNDECIDED;
static mut in_seq_type: libc::c_int = seq_type_dna as libc::c_int;
static mut in_seq_type_name: *const libc::c_char =
    b"DNA\x00" as *const u8 as *const libc::c_char;
static mut verbose: bool = 0 as libc::c_int != 0;
static mut binary_stderr: bool = 0 as libc::c_int != 0;
static mut use_mask: bool = 1 as libc::c_int != 0;
static mut in_file_path: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut IN: *mut FILE = 0 as *const FILE as *mut FILE;
static mut input_stat: stat =
    stat{st_dev: 0,
         st_ino: 0,
         st_nlink: 0,
         st_mode: 0,
         st_uid: 0,
         st_gid: 0,
         __pad0: 0,
         st_rdev: 0,
         st_size: 0,
         st_blksize: 0,
         st_blocks: 0,
         st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
         st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
         st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
         __glibc_reserved: [0; 3],};
static mut have_input_stat: bool = 0 as libc::c_int != 0;
static mut out_file_path: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut out_file_path_auto: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut OUT: *mut FILE = 0 as *const FILE as *mut FILE;
static mut force_stdout: bool = 0 as libc::c_int != 0;
static mut binary_stdout: bool = 0 as libc::c_int != 0;
static mut created_output_file: bool = 0 as libc::c_int != 0;
static mut format_version: libc::c_uchar = 1 as libc::c_int as libc::c_uchar;
static mut name_separator: libc::c_uchar = ' ' as i32 as libc::c_uchar;
static mut has_title: libc::c_int = 0 as libc::c_int;
static mut has_ids: libc::c_int = 0 as libc::c_int;
static mut has_names: libc::c_int = 0 as libc::c_int;
static mut has_lengths: libc::c_int = 0 as libc::c_int;
static mut has_mask: libc::c_int = 0 as libc::c_int;
static mut has_data: libc::c_int = 0 as libc::c_int;
static mut has_quality: libc::c_int = 0 as libc::c_int;
static mut max_line_length: libc::c_ulonglong = 0;
static mut N: libc::c_ulonglong = 0;
static mut ids_buffer: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut compressed_ids_buffer: *mut libc::c_uchar =
    0 as *const libc::c_uchar as *mut libc::c_uchar;
static mut ids: *mut *mut libc::c_char =
    0 as *const *mut libc::c_char as *mut *mut libc::c_char;
static mut names_buffer: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut compressed_names_buffer: *mut libc::c_uchar =
    0 as *const libc::c_uchar as *mut libc::c_uchar;
static mut names: *mut *mut libc::c_char =
    0 as *const *mut libc::c_char as *mut *mut libc::c_char;
static mut lengths_buffer: *mut libc::c_uint =
    0 as *const libc::c_uint as *mut libc::c_uint;
static mut compressed_lengths_buffer: *mut libc::c_uchar =
    0 as *const libc::c_uchar as *mut libc::c_uchar;
static mut n_lengths: libc::c_ulonglong =
    0 as libc::c_int as libc::c_ulonglong;
static mut mask_size: libc::c_ulonglong =
    0 as libc::c_int as libc::c_ulonglong;
static mut mask_buffer: *mut libc::c_uchar =
    0 as *const libc::c_uchar as *mut libc::c_uchar;
static mut compressed_mask_buffer: *mut libc::c_uchar =
    0 as *const libc::c_uchar as *mut libc::c_uchar;
static mut total_seq_length: libc::c_ulonglong =
    0 as libc::c_int as libc::c_ulonglong;
static mut compressed_seq_size: libc::c_ulonglong =
    0 as libc::c_int as libc::c_ulonglong;
static mut compressed_seq_buffer: *mut libc::c_uchar =
    0 as *const libc::c_uchar as *mut libc::c_uchar;
static mut compressed_seq_pos: libc::c_ulonglong =
    0 as libc::c_int as libc::c_ulonglong;
static mut total_quality_length: libc::c_ulonglong =
    0 as libc::c_int as libc::c_ulonglong;
static mut compressed_quality_size: libc::c_ulonglong =
    0 as libc::c_int as libc::c_ulonglong;
static mut in_buffer_size: size_t = 0 as libc::c_int as size_t;
static mut in_buffer: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut out_buffer_size: size_t = 0 as libc::c_int as size_t;
static mut out_buffer: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut mem_out_buffer_size: size_t = 0 as libc::c_int as size_t;
static mut mem_out_buffer: *mut libc::c_uchar =
    0 as *const libc::c_uchar as *mut libc::c_uchar;
static mut out_print_buffer_size: size_t = 0 as libc::c_int as size_t;
static mut out_print_buffer: *mut libc::c_uchar =
    0 as *const libc::c_uchar as *mut libc::c_uchar;
static mut input_decompression_stream: *mut ZSTD_DStream =
    0 as *const ZSTD_DStream as *mut ZSTD_DStream;
static mut file_bytes_to_read: size_t = 0;
static mut zstd_file_in_buffer: ZSTD_inBuffer =
    ZSTD_inBuffer{src: 0 as *const libc::c_void, size: 0, pos: 0,};
static mut memory_decompression_stream: *mut ZSTD_DStream =
    0 as *const ZSTD_DStream as *mut ZSTD_DStream;
static mut memory_bytes_to_read: size_t = 0;
static mut zstd_mem_in_buffer: ZSTD_inBuffer =
    ZSTD_inBuffer{src: 0 as *const libc::c_void, size: 0, pos: 0,};
static mut cur_seq_index: libc::c_ulonglong =
    0 as libc::c_int as libc::c_ulonglong;
static mut dna_buffer: *mut libc::c_uchar =
    0 as *const libc::c_uchar as *mut libc::c_uchar;
static mut dna_buffer_size: size_t = 0 as libc::c_int as size_t;
static mut dna_buffer_flush_size: size_t = 0 as libc::c_int as size_t;
static mut dna_buffer_pos: libc::c_uint = 0 as libc::c_int as libc::c_uint;
static mut dna_buffer_filling_pos: libc::c_uint =
    0 as libc::c_int as libc::c_uint;
static mut dna_buffer_printing_pos: libc::c_uint =
    0 as libc::c_int as libc::c_uint;
static mut dna_buffer_remaining: libc::c_uint =
    0 as libc::c_int as libc::c_uint;
static mut quality_buffer: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut quality_buffer_size: size_t = 0 as libc::c_int as size_t;
static mut quality_buffer_flush_size: size_t = 0 as libc::c_int as size_t;
static mut quality_buffer_filling_pos: libc::c_uint =
    0 as libc::c_int as libc::c_uint;
static mut quality_buffer_printing_pos: libc::c_uint =
    0 as libc::c_int as libc::c_uint;
static mut quality_buffer_remaining: libc::c_uint =
    0 as libc::c_int as libc::c_uint;
static mut total_seq_n_bp_remaining: libc::c_ulonglong =
    0 as libc::c_int as libc::c_ulonglong;
static mut cur_seq_len_index: libc::c_ulonglong =
    0 as libc::c_int as libc::c_ulonglong;
static mut cur_seq_len_n_bp_remaining: libc::c_ulonglong =
    0 as libc::c_int as libc::c_ulonglong;
static mut cur_qual_len_index: libc::c_ulonglong =
    0 as libc::c_int as libc::c_ulonglong;
static mut cur_mask: libc::c_ulonglong =
    0 as libc::c_int as libc::c_ulonglong;
static mut cur_mask_remaining: libc::c_uint =
    0 as libc::c_int as libc::c_uint;
static mut mask_on: libc::c_int = 0 as libc::c_int;
static mut cur_line_n_bp_remaining: libc::c_ulonglong =
    0 as libc::c_int as libc::c_ulonglong;
static mut line_length_is_specified: bool = 0 as libc::c_int != 0;
static mut requested_line_length: libc::c_ulonglong = 0 as libc::c_ulonglong;
static mut success: bool = 0 as libc::c_int != 0;
/*
 * NAF decompressor
 * Copyright (c) 2018-2020 Kirill Kryukov
 * See README.md and LICENSE files of this repository
 */
//__attribute__ ((format (printf, 1, 2)))
unsafe extern "C" fn msg(mut format: *const libc::c_char, mut args: ...) {
    let mut argptr: ::std::ffi::VaListImpl;
    argptr = args.clone();
    vfprintf(stderr, format, argptr.as_va_list());
}
#[cold]
unsafe extern "C" fn err(mut format: *const libc::c_char, mut args: ...) {
    fputs(b"unnaf error: \x00" as *const u8 as *const libc::c_char, stderr);
    let mut argptr: ::std::ffi::VaListImpl;
    argptr = args.clone();
    vfprintf(stderr, format, argptr.as_va_list());
}
#[cold]
unsafe extern "C" fn die(mut format: *const libc::c_char, mut args: ...)
 -> ! {
    fputs(b"unnaf error: \x00" as *const u8 as *const libc::c_char, stderr);
    let mut argptr: ::std::ffi::VaListImpl;
    argptr = args.clone();
    vfprintf(stderr, format, argptr.as_va_list());
    exit(1 as libc::c_int);
}
#[cold]
#[inline]
unsafe extern "C" fn incomplete() -> ! {
    die(b"incomplete or truncated input\n\x00" as *const u8 as
            *const libc::c_char);
}
#[cold]
unsafe extern "C" fn out_of_memory(size: size_t) -> ! {
    die(b"can\'t allocate %zu bytes\n\x00" as *const u8 as
            *const libc::c_char, size);
}
unsafe extern "C" fn malloc_or_die(size: size_t) -> *mut libc::c_void {
    let mut buf: *mut libc::c_void = malloc(size);
    if buf.is_null() { out_of_memory(size); }
    return buf;
}
unsafe extern "C" fn init_tables() {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < 16 as libc::c_int as libc::c_uint {
        let mut j: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while j < 16 as libc::c_int as libc::c_uint {
            codes_to_nucs[(i << 4 as libc::c_int | j) as usize] =
                ((code_to_nuc[i as usize] as libc::c_ushort as libc::c_int) <<
                     8 as libc::c_int |
                     code_to_nuc[j as usize] as libc::c_int) as
                    libc::c_ushort;
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn fgetc_or_incomplete(mut F: *mut FILE) -> libc::c_uchar {
    let mut c: libc::c_int = fgetc(F);
    if c == -(1 as libc::c_int) { incomplete(); }
    return c as libc::c_uchar;
}
unsafe extern "C" fn fflush_or_die(mut F: *mut FILE) {
    let mut error: libc::c_int = fflush(F);
    if error != 0 as libc::c_int {
        die(b"can\'t write to file - disk full?\n\x00" as *const u8 as
                *const libc::c_char);
    };
}
unsafe extern "C" fn fclose_or_die(mut F: *mut FILE) {
    let mut error: libc::c_int = fclose(F);
    if error != 0 as libc::c_int {
        die(b"can\'t close file - disk full?\n\x00" as *const u8 as
                *const libc::c_char);
    };
}
/*
 * Reads a number in variable length encoding.
 */
unsafe extern "C" fn read_number(mut F: *mut FILE) -> libc::c_ulonglong {
    static mut overflow_msg: *const libc::c_char =
        b"invalid input: overflow reading a variable length encoded number\n\x00"
            as *const u8 as *const libc::c_char;
    let mut a: libc::c_ulonglong = 0 as libc::c_int as libc::c_ulonglong;
    let mut c: libc::c_uchar = 0;
    if fread(&mut c as *mut libc::c_uchar as *mut libc::c_void,
             1 as libc::c_int as size_t, 1 as libc::c_int as size_t, F) == 0 {
        incomplete();
    }
    if c as libc::c_int == 128 as libc::c_int {
        die(b"invalid input: error parsing variable length encoded number\n\x00"
                as *const u8 as *const libc::c_char);
    }
    while c as libc::c_int & 128 as libc::c_int != 0 {
        if a & (127 as libc::c_ulonglong) << 57 as libc::c_int != 0 {
            fputs(overflow_msg, stderr);
            exit(1 as libc::c_int);
        }
        a =
            a << 7 as libc::c_int |
                (c as libc::c_int & 127 as libc::c_int) as libc::c_ulonglong;
        if fread(&mut c as *mut libc::c_uchar as *mut libc::c_void,
                 1 as libc::c_int as size_t, 1 as libc::c_int as size_t, F) ==
               0 {
            incomplete();
        }
    }
    if a & (127 as libc::c_ulonglong) << 57 as libc::c_int != 0 {
        fputs(overflow_msg, stderr);
        exit(1 as libc::c_int);
    }
    a = a << 7 as libc::c_int | c as libc::c_ulonglong;
    return a;
}
#[inline]
unsafe extern "C" fn put_magic_number(mut buffer: *mut libc::c_uchar) {
    *buffer.offset(0 as libc::c_int as isize) =
        0x28 as libc::c_int as libc::c_uchar;
    *buffer.offset(1 as libc::c_int as isize) =
        0xb5 as libc::c_int as libc::c_uchar;
    *buffer.offset(2 as libc::c_int as isize) =
        0x2f as libc::c_int as libc::c_uchar;
    *buffer.offset(3 as libc::c_int as isize) =
        0xfd as libc::c_int as libc::c_uchar;
}
/*
 * NAF decompressor
 * Copyright (c) 2018-2020 Kirill Kryukov
 * See README.md and LICENSE files of this repository
 */
unsafe extern "C" fn open_input_file() {
    if in_file_path.is_null() {
        if freopen(0 as *const libc::c_char,
                   b"rb\x00" as *const u8 as *const libc::c_char,
                   stdin).is_null() {
            die(b"can\'t read input in binary mode\n\x00" as *const u8 as
                    *const libc::c_char);
        }
        IN = stdin
    } else {
        IN =
            fopen(in_file_path,
                  b"rb\x00" as *const u8 as *const libc::c_char);
        if IN.is_null() {
            die(b"can\'t open input file\n\x00" as *const u8 as
                    *const libc::c_char);
        }
    };
}
unsafe extern "C" fn change_stderr_to_binary() {
    if freopen(0 as *const libc::c_char,
               b"wb\x00" as *const u8 as *const libc::c_char,
               stderr).is_null() {
        die(b"can\'t set error stream to binary mode\n\x00" as *const u8 as
                *const libc::c_char);
    };
}
unsafe extern "C" fn open_output_file() {
    let mut extracting_to_original_format: bool =
        if has_quality != 0 {
            (out_type as libc::c_uint == FASTA as libc::c_int as libc::c_uint)
                as libc::c_int
        } else {
            (out_type as libc::c_uint == FASTQ as libc::c_int as libc::c_uint)
                as libc::c_int
        } != 0;
    let mut is_large_output: bool =
        out_type as libc::c_uint == IDS as libc::c_int as libc::c_uint ||
            out_type as libc::c_uint == NAMES as libc::c_int as libc::c_uint
            ||
            out_type as libc::c_uint == LENGTHS as libc::c_int as libc::c_uint
            || out_type as libc::c_uint == MASK as libc::c_int as libc::c_uint
            ||
            out_type as libc::c_uint ==
                FOUR_BIT as libc::c_int as libc::c_uint ||
            out_type as libc::c_uint == DNA as libc::c_int as libc::c_uint ||
            out_type as libc::c_uint ==
                MASKED_DNA as libc::c_int as libc::c_uint ||
            out_type as libc::c_uint ==
                UNMASKED_DNA as libc::c_int as libc::c_uint ||
            out_type as libc::c_uint == SEQ as libc::c_int as libc::c_uint ||
            out_type as libc::c_uint == FASTA as libc::c_int as libc::c_uint
            ||
            out_type as libc::c_uint ==
                MASKED_FASTA as libc::c_int as libc::c_uint ||
            out_type as libc::c_uint ==
                UNMASKED_FASTA as libc::c_int as libc::c_uint ||
            out_type as libc::c_uint == FASTQ as libc::c_int as libc::c_uint;
    if extracting_to_original_format as libc::c_int != 0 && !force_stdout &&
           !in_file_path.is_null() && out_file_path.is_null() &&
           isatty(fileno(stdout)) != 0 {
        let mut len: size_t = strlen(in_file_path);
        if len > 4 as libc::c_int as libc::c_ulong &&
               strcmp(in_file_path.offset(len as
                                              isize).offset(-(4 as libc::c_int
                                                                  as isize)),
                      b".naf\x00" as *const u8 as *const libc::c_char) ==
                   0 as libc::c_int &&
               *in_file_path.offset(len.wrapping_sub(5 as libc::c_int as
                                                         libc::c_ulong) as
                                        isize) as libc::c_int != '/' as i32 &&
               *in_file_path.offset(len.wrapping_sub(5 as libc::c_int as
                                                         libc::c_ulong) as
                                        isize) as libc::c_int != '\\' as i32 {
            out_file_path_auto =
                malloc_or_die(len.wrapping_sub(3 as libc::c_int as
                                                   libc::c_ulong)) as
                    *mut libc::c_char;
            memcpy(out_file_path_auto as *mut libc::c_void,
                   in_file_path as *const libc::c_void,
                   len.wrapping_sub(4 as libc::c_int as libc::c_ulong));
            *out_file_path_auto.offset(len.wrapping_sub(4 as libc::c_int as
                                                            libc::c_ulong) as
                                           isize) =
                0 as libc::c_int as libc::c_char;
            out_file_path = out_file_path_auto
        }
    }
    if !out_file_path.is_null() && !force_stdout {
        OUT =
            fopen(out_file_path,
                  b"wb\x00" as *const u8 as *const libc::c_char);
        if OUT.is_null() {
            die(b"can\'t create output file\n\x00" as *const u8 as
                    *const libc::c_char);
        }
        created_output_file = 1 as libc::c_int != 0
    } else { OUT = stdout }
    if binary_stdout as libc::c_int != 0 ||
           out_type as libc::c_uint == FOUR_BIT as libc::c_int as libc::c_uint
               && force_stdout as libc::c_int != 0 {
        if freopen(0 as *const libc::c_char,
                   b"wb\x00" as *const u8 as *const libc::c_char,
                   stdout).is_null() {
            die(b"can\'t set output stream to binary mode\n\x00" as *const u8
                    as *const libc::c_char);
        }
    }
    if is_large_output as libc::c_int != 0 && !force_stdout &&
           isatty(fileno(OUT)) != 0 {
        die(b"output file not specified - please either specify output file with \'-o\' or \'>\', or use \'-c\' option to force writing to console\n\x00"
                as *const u8 as *const libc::c_char);
    };
}
unsafe extern "C" fn close_input_file() {
    if !IN.is_null() && IN != stdin { fclose(IN); IN = 0 as *mut FILE };
}
unsafe extern "C" fn close_output_file() {
    if OUT.is_null() { return }
    fclose_or_die(OUT);
    OUT = 0 as *mut FILE;
}
unsafe extern "C" fn close_output_file_and_set_stat() {
    fflush_or_die(OUT);
    if fchmod(fileno(OUT),
              input_stat.st_mode &
                  (0o400 as libc::c_int | 0o200 as libc::c_int |
                       0o100 as libc::c_int |
                       (0o400 as libc::c_int | 0o200 as libc::c_int |
                            0o100 as libc::c_int) >> 3 as libc::c_int |
                       (0o400 as libc::c_int | 0o200 as libc::c_int |
                            0o100 as libc::c_int) >> 3 as libc::c_int >>
                           3 as libc::c_int) as libc::c_uint) !=
           0 as libc::c_int {
        err(b"can\'t transfer permissions from input to output file\n\x00" as
                *const u8 as *const libc::c_char);
    }
    if fchown(fileno(OUT), input_stat.st_uid, input_stat.st_gid) !=
           0 as libc::c_int {
        err(b"can\'t transfer ownership from input to output file\n\x00" as
                *const u8 as *const libc::c_char);
    }
    let mut input_timestamp: [timespec; 2] =
        [timespec{tv_sec: 0, tv_nsec: 0,}; 2];
    input_timestamp[0 as libc::c_int as usize].tv_sec =
        input_stat.st_atim.tv_sec;
    input_timestamp[1 as libc::c_int as usize].tv_sec =
        input_stat.st_mtim.tv_sec;
    input_timestamp[0 as libc::c_int as usize].tv_nsec =
        input_stat.st_atim.tv_nsec;
    input_timestamp[1 as libc::c_int as usize].tv_nsec =
        input_stat.st_mtim.tv_nsec;
    if futimens(fileno(OUT), input_timestamp.as_mut_ptr() as *const timespec)
           != 0 as libc::c_int {
        err(b"can\'t transfer timestamp from input to output file\n\x00" as
                *const u8 as *const libc::c_char);
    }
    //if (verbose) { msg("Changed output timestamp using futimens()\n"); }
    fclose_or_die(OUT);
    OUT = 0 as *mut FILE;
}
static mut ahead_buffer: [libc::c_uchar; 16384] = [0; 16384];
unsafe extern "C" fn skip_ahead(mut bytes: libc::c_ulonglong) {
    if IN == stdin {
        let mut remaining: libc::c_ulonglong = bytes;
        while remaining >
                  AHEAD_BUFFER_SIZE as libc::c_int as libc::c_ulonglong {
            if fread(ahead_buffer.as_mut_ptr() as *mut libc::c_void,
                     1 as libc::c_int as size_t,
                     AHEAD_BUFFER_SIZE as libc::c_int as size_t, IN) !=
                   AHEAD_BUFFER_SIZE as libc::c_int as libc::c_ulong {
                incomplete();
            }
            remaining =
                remaining.wrapping_sub(AHEAD_BUFFER_SIZE as libc::c_int as
                                           libc::c_ulonglong)
        }
        if fread(ahead_buffer.as_mut_ptr() as *mut libc::c_void,
                 1 as libc::c_int as size_t, remaining as size_t, IN) as
               libc::c_ulonglong != remaining {
            incomplete();
        }
    } else {
        let mut error: libc::c_int =
            fseek(IN, bytes as libc::c_long, 1 as libc::c_int);
        if error != 0 { incomplete(); }
    };
}
unsafe extern "C" fn read_header() {
    let mut first_bytes: [libc::c_uchar; 3] = [0; 3];
    let mut could_read: size_t =
        fread(&mut first_bytes as *mut [libc::c_uchar; 3] as
                  *mut libc::c_void, 1 as libc::c_int as size_t,
              3 as libc::c_int as size_t, IN);
    if could_read == 0 as libc::c_int as libc::c_ulong {
        die(b"empty input\x00" as *const u8 as *const libc::c_char);
    } else {
        if could_read != 3 as libc::c_int as libc::c_ulong { incomplete(); }
    }
    if first_bytes[0 as libc::c_int as usize] as libc::c_int !=
           0x1 as libc::c_int ||
           first_bytes[1 as libc::c_int as usize] as libc::c_int !=
               0xf9 as libc::c_int ||
           first_bytes[2 as libc::c_int as usize] as libc::c_int !=
               0xec as libc::c_int {
        die(b"not a NAF format\n\x00" as *const u8 as *const libc::c_char);
    }
    format_version = fgetc_or_incomplete(IN);
    if (format_version as libc::c_int) < 1 as libc::c_int ||
           format_version as libc::c_int > 2 as libc::c_int {
        die(b"unknown version (%d) of NAF format\n\x00" as *const u8 as
                *const libc::c_char, format_version as libc::c_int);
    }
    if format_version as libc::c_int > 1 as libc::c_int {
        let mut t: libc::c_uchar = fgetc_or_incomplete(IN);
        if t as libc::c_int == 1 as libc::c_int {
            in_seq_type = seq_type_rna as libc::c_int;
            in_seq_type_name = b"RNA\x00" as *const u8 as *const libc::c_char
        } else if t as libc::c_int == 2 as libc::c_int {
            in_seq_type = seq_type_protein as libc::c_int;
            in_seq_type_name =
                b"protein\x00" as *const u8 as *const libc::c_char
        } else if t as libc::c_int == 3 as libc::c_int {
            in_seq_type = seq_type_text as libc::c_int;
            in_seq_type_name = b"text\x00" as *const u8 as *const libc::c_char
        } else {
            die(b"unknown sequence type (%d) found in NAF file\n\x00" as
                    *const u8 as *const libc::c_char, t as libc::c_int);
        }
    }
    let mut flags: libc::c_uchar = fgetc_or_incomplete(IN);
    has_title = flags as libc::c_int >> 6 as libc::c_int & 1 as libc::c_int;
    has_ids = flags as libc::c_int >> 5 as libc::c_int & 1 as libc::c_int;
    has_names = flags as libc::c_int >> 4 as libc::c_int & 1 as libc::c_int;
    has_lengths = flags as libc::c_int >> 3 as libc::c_int & 1 as libc::c_int;
    has_mask = flags as libc::c_int >> 2 as libc::c_int & 1 as libc::c_int;
    has_data = flags as libc::c_int >> 1 as libc::c_int & 1 as libc::c_int;
    has_quality = flags as libc::c_int & 1 as libc::c_int;
    name_separator = fgetc_or_incomplete(IN);
    if (name_separator as libc::c_int) < 0x20 as libc::c_int ||
           name_separator as libc::c_int > 0x7e as libc::c_int {
        die(b"unsupported name separator character\n\x00" as *const u8 as
                *const libc::c_char);
    };
}
unsafe extern "C" fn skip_title() {
    if has_title != 0 {
        let mut title_size: libc::c_ulonglong = read_number(IN);
        skip_ahead(title_size);
    };
}
unsafe extern "C" fn skip_ids() {
    if has_ids != 0 {
        read_number(IN);
        let mut compressed_ids_size: libc::c_ulonglong = read_number(IN);
        skip_ahead(compressed_ids_size);
    };
}
unsafe extern "C" fn skip_names() {
    if has_names != 0 {
        read_number(IN);
        let mut compressed_names_size: libc::c_ulonglong = read_number(IN);
        skip_ahead(compressed_names_size);
    };
}
unsafe extern "C" fn skip_lengths() {
    if has_lengths != 0 {
        read_number(IN);
        let mut compressed_lengths_size: libc::c_ulonglong = read_number(IN);
        skip_ahead(compressed_lengths_size);
    };
}
unsafe extern "C" fn skip_mask() {
    if has_mask != 0 {
        read_number(IN);
        let mut compressed_mask_size: libc::c_ulonglong = read_number(IN);
        skip_ahead(compressed_mask_size);
    };
}
/*static void skip_data(void)
{
    if (has_data)
    {
        read_number(IN);
        unsigned long long compressed_data_size = read_number(IN);
        skip_ahead(compressed_data_size);
    }
}*/
unsafe extern "C" fn load_ids() {
    let mut ids_size: libc::c_ulonglong = read_number(IN);
    let mut compressed_ids_size: libc::c_ulonglong = read_number(IN);
    ids_buffer = malloc_or_die(ids_size as size_t) as *mut libc::c_char;
    compressed_ids_buffer =
        malloc_or_die(compressed_ids_size.wrapping_add(4 as libc::c_int as
                                                           libc::c_ulonglong)
                          as size_t) as *mut libc::c_uchar;
    put_magic_number(compressed_ids_buffer);
    if fread(compressed_ids_buffer.offset(4 as libc::c_int as isize) as
                 *mut libc::c_void, 1 as libc::c_int as size_t,
             compressed_ids_size as size_t, IN) as libc::c_ulonglong !=
           compressed_ids_size {
        incomplete();
    }
    let mut n_dec_bytes: size_t =
        ZSTD_decompress(ids_buffer as *mut libc::c_void, ids_size as size_t,
                        compressed_ids_buffer as *mut libc::c_void,
                        compressed_ids_size.wrapping_add(4 as libc::c_int as
                                                             libc::c_ulonglong)
                            as size_t);
    if n_dec_bytes as libc::c_ulonglong != ids_size {
        die(b"can\'t decompress ids\n\x00" as *const u8 as
                *const libc::c_char);
    }
    if *ids_buffer.offset(ids_size.wrapping_sub(1 as libc::c_int as
                                                    libc::c_ulonglong) as
                              isize) as libc::c_int != 0 as libc::c_int {
        die(b"corrupted ids - not 0-terminated\n\x00" as *const u8 as
                *const libc::c_char);
    }
    free(compressed_ids_buffer as *mut libc::c_void);
    compressed_ids_buffer = 0 as *mut libc::c_uchar;
    ids =
        malloc_or_die((::std::mem::size_of::<*mut libc::c_char>() as
                           libc::c_ulong as libc::c_ulonglong).wrapping_mul(N)
                          as size_t) as *mut *mut libc::c_char;
    let ref mut fresh0 = *ids.offset(0 as libc::c_int as isize);
    *fresh0 = ids_buffer;
    let mut i: libc::c_ulonglong = 1 as libc::c_int as libc::c_ulonglong;
    while i < N {
        let mut ep: *mut libc::c_char =
            strchr(*ids.offset(i.wrapping_sub(1 as libc::c_int as
                                                  libc::c_ulonglong) as
                                   isize), 0 as libc::c_int);
        if ep >=
               ids_buffer.offset(ids_size as
                                     isize).offset(-(1 as libc::c_int as
                                                         isize)) {
            die(b"currupted ids - can\'t read id %llu\n\x00" as *const u8 as
                    *const libc::c_char, i);
        }
        let ref mut fresh1 = *ids.offset(i as isize);
        *fresh1 = ep.offset(1 as libc::c_int as isize);
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn load_names() {
    let mut names_size: libc::c_ulonglong = read_number(IN);
    let mut compressed_names_size: libc::c_ulonglong = read_number(IN);
    names_buffer = malloc_or_die(names_size as size_t) as *mut libc::c_char;
    compressed_names_buffer =
        malloc_or_die(compressed_names_size.wrapping_add(4 as libc::c_int as
                                                             libc::c_ulonglong)
                          as size_t) as *mut libc::c_uchar;
    put_magic_number(compressed_names_buffer);
    if fread(compressed_names_buffer.offset(4 as libc::c_int as isize) as
                 *mut libc::c_void, 1 as libc::c_int as size_t,
             compressed_names_size as size_t, IN) as libc::c_ulonglong !=
           compressed_names_size {
        incomplete();
    }
    let mut n_dec_bytes: size_t =
        ZSTD_decompress(names_buffer as *mut libc::c_void,
                        names_size as size_t,
                        compressed_names_buffer as *mut libc::c_void,
                        compressed_names_size.wrapping_add(4 as libc::c_int as
                                                               libc::c_ulonglong)
                            as size_t);
    if n_dec_bytes as libc::c_ulonglong != names_size {
        die(b"can\'t decompress names\n\x00" as *const u8 as
                *const libc::c_char);
    }
    if *names_buffer.offset(names_size.wrapping_sub(1 as libc::c_int as
                                                        libc::c_ulonglong) as
                                isize) as libc::c_int != 0 as libc::c_int {
        die(b"corrupted names - not 0-terminated\n\x00" as *const u8 as
                *const libc::c_char);
    }
    free(compressed_names_buffer as *mut libc::c_void);
    compressed_names_buffer = 0 as *mut libc::c_uchar;
    names =
        malloc_or_die((::std::mem::size_of::<*mut libc::c_char>() as
                           libc::c_ulong as libc::c_ulonglong).wrapping_mul(N)
                          as size_t) as *mut *mut libc::c_char;
    let ref mut fresh2 = *names.offset(0 as libc::c_int as isize);
    *fresh2 = names_buffer;
    let mut i: libc::c_ulonglong = 1 as libc::c_int as libc::c_ulonglong;
    while i < N {
        let mut ep: *mut libc::c_char =
            strchr(*names.offset(i.wrapping_sub(1 as libc::c_int as
                                                    libc::c_ulonglong) as
                                     isize), 0 as libc::c_int);
        if ep >=
               names_buffer.offset(names_size as
                                       isize).offset(-(1 as libc::c_int as
                                                           isize)) {
            die(b"corrupted names - can\'t read name %llu\n\x00" as *const u8
                    as *const libc::c_char, i);
        }
        let ref mut fresh3 = *names.offset(i as isize);
        *fresh3 = ep.offset(1 as libc::c_int as isize);
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn load_lengths() {
    let mut lengths_size: libc::c_ulonglong = read_number(IN);
    let mut compressed_lengths_size: libc::c_ulonglong = read_number(IN);
    n_lengths =
        lengths_size.wrapping_div(4 as libc::c_int as libc::c_ulonglong);
    lengths_buffer =
        malloc_or_die(lengths_size as size_t) as *mut libc::c_uint;
    compressed_lengths_buffer =
        malloc_or_die(compressed_lengths_size.wrapping_add(4 as libc::c_int as
                                                               libc::c_ulonglong)
                          as size_t) as *mut libc::c_uchar;
    put_magic_number(compressed_lengths_buffer);
    if fread(compressed_lengths_buffer.offset(4 as libc::c_int as isize) as
                 *mut libc::c_void, 1 as libc::c_int as size_t,
             compressed_lengths_size as size_t, IN) as libc::c_ulonglong !=
           compressed_lengths_size {
        incomplete();
    }
    let mut n_dec_bytes: size_t =
        ZSTD_decompress(lengths_buffer as *mut libc::c_void,
                        lengths_size as size_t,
                        compressed_lengths_buffer as *mut libc::c_void,
                        compressed_lengths_size.wrapping_add(4 as libc::c_int
                                                                 as
                                                                 libc::c_ulonglong)
                            as size_t);
    if n_dec_bytes as libc::c_ulonglong != lengths_size {
        die(b"can\'t decompress lengths\n\x00" as *const u8 as
                *const libc::c_char);
    }
    free(compressed_lengths_buffer as *mut libc::c_void);
    compressed_lengths_buffer = 0 as *mut libc::c_uchar;
}
unsafe extern "C" fn load_mask() {
    mask_size = read_number(IN);
    let mut compressed_mask_size: libc::c_ulonglong = read_number(IN);
    mask_buffer = malloc_or_die(mask_size as size_t) as *mut libc::c_uchar;
    compressed_mask_buffer =
        malloc_or_die(compressed_mask_size.wrapping_add(4 as libc::c_int as
                                                            libc::c_ulonglong)
                          as size_t) as *mut libc::c_uchar;
    put_magic_number(compressed_mask_buffer);
    if fread(compressed_mask_buffer.offset(4 as libc::c_int as isize) as
                 *mut libc::c_void, 1 as libc::c_int as size_t,
             compressed_mask_size as size_t, IN) as libc::c_ulonglong !=
           compressed_mask_size {
        incomplete();
    }
    let mut n_dec_bytes: size_t =
        ZSTD_decompress(mask_buffer as *mut libc::c_void, mask_size as size_t,
                        compressed_mask_buffer as *mut libc::c_void,
                        compressed_mask_size.wrapping_add(4 as libc::c_int as
                                                              libc::c_ulonglong)
                            as size_t);
    if n_dec_bytes as libc::c_ulonglong != mask_size {
        die(b"can\'t decompress mask\n\x00" as *const u8 as
                *const libc::c_char);
    }
    free(compressed_mask_buffer as *mut libc::c_void);
    compressed_mask_buffer = 0 as *mut libc::c_uchar;
    if mask_size > 0 as libc::c_int as libc::c_ulonglong {
        if *mask_buffer.offset(0 as libc::c_int as isize) as libc::c_int ==
               0 as libc::c_int {
            mask_on = 1 as libc::c_int;
            cur_mask = 1 as libc::c_int as libc::c_ulonglong;
            if mask_size < 2 as libc::c_int as libc::c_ulonglong {
                die(b"corrupted mask\n\x00" as *const u8 as
                        *const libc::c_char);
            }
        }
        cur_mask_remaining =
            *mask_buffer.offset(cur_mask as isize) as libc::c_uint
    };
}
unsafe extern "C" fn load_compressed_sequence() {
    total_seq_length = read_number(IN);
    compressed_seq_size = read_number(IN);
    compressed_seq_buffer =
        malloc_or_die(compressed_seq_size.wrapping_add(4 as libc::c_int as
                                                           libc::c_ulonglong)
                          as size_t) as *mut libc::c_uchar;
    put_magic_number(compressed_seq_buffer);
    if fread(compressed_seq_buffer.offset(4 as libc::c_int as isize) as
                 *mut libc::c_void, 1 as libc::c_int as size_t,
             compressed_seq_size as size_t, IN) as libc::c_ulonglong !=
           compressed_seq_size {
        incomplete();
    };
}
unsafe extern "C" fn initialize_input_decompression() -> size_t {
    in_buffer_size = ZSTD_DStreamInSize();
    in_buffer = malloc_or_die(in_buffer_size) as *mut libc::c_char;
    out_buffer_size = ZSTD_DStreamOutSize();
    out_buffer = malloc_or_die(out_buffer_size) as *mut libc::c_char;
    input_decompression_stream = ZSTD_createDStream();
    if input_decompression_stream.is_null() {
        die(b"can\'t create input decompression stream\n\x00" as *const u8 as
                *const libc::c_char);
    }
    let mut bytes_to_read: size_t =
        ZSTD_initDStream(input_decompression_stream);
    if ZSTD_isError(bytes_to_read) != 0 {
        die(b"can\'t initialize input decompression stream: %s\n\x00" as
                *const u8 as *const libc::c_char,
            ZSTD_getErrorName(bytes_to_read));
    }
    if bytes_to_read < 5 as libc::c_int as libc::c_ulong {
        die(b"can\'t initialize decompression\n\x00" as *const u8 as
                *const libc::c_char);
    }
    put_magic_number(in_buffer as *mut libc::c_uchar);
    let mut could_read: size_t =
        fread(in_buffer.offset(4 as libc::c_int as isize) as
                  *mut libc::c_void, 1 as libc::c_int as size_t,
              bytes_to_read.wrapping_sub(4 as libc::c_int as libc::c_ulong),
              IN);
    if could_read !=
           bytes_to_read.wrapping_sub(4 as libc::c_int as libc::c_ulong) {
        incomplete();
    }
    let mut in_0: ZSTD_inBuffer =
        {
            let mut init =
                ZSTD_inBuffer_s{src: in_buffer as *const libc::c_void,
                                size: bytes_to_read,
                                pos: 0 as libc::c_int as size_t,};
            init
        };
    let mut out: ZSTD_outBuffer =
        {
            let mut init =
                ZSTD_outBuffer_s{dst: out_buffer as *mut libc::c_void,
                                 size: out_buffer_size,
                                 pos: 0 as libc::c_int as size_t,};
            init
        };
    bytes_to_read =
        ZSTD_decompressStream(input_decompression_stream, &mut out,
                              &mut in_0);
    if ZSTD_isError(bytes_to_read) != 0 {
        die(b"can\'t decompress: %s\n\x00" as *const u8 as
                *const libc::c_char, ZSTD_getErrorName(bytes_to_read));
    }
    if in_0.pos != in_0.size {
        die(b"can\'t decompress first block\n\x00" as *const u8 as
                *const libc::c_char);
    }
    if out.pos != 0 as libc::c_int as libc::c_ulong {
        die(b"can\'t decompress first block\n\x00" as *const u8 as
                *const libc::c_char);
    }
    return bytes_to_read;
}
unsafe extern "C" fn initialize_memory_decompression() {
    mem_out_buffer_size = ZSTD_DStreamOutSize();
    mem_out_buffer = malloc_or_die(mem_out_buffer_size) as *mut libc::c_uchar;
    memory_decompression_stream = ZSTD_createDStream();
    if memory_decompression_stream.is_null() {
        die(b"can\'t create memory decompression stream\n\x00" as *const u8 as
                *const libc::c_char);
    }
    memory_bytes_to_read = ZSTD_initDStream(memory_decompression_stream);
    if ZSTD_isError(memory_bytes_to_read) != 0 {
        die(b"can\'t initialize memory decompression stream: %s\n\x00" as
                *const u8 as *const libc::c_char,
            ZSTD_getErrorName(memory_bytes_to_read));
    };
}
unsafe extern "C" fn initialize_quality_file_decompression() {
    in_buffer_size = ZSTD_DStreamInSize();
    in_buffer = malloc_or_die(in_buffer_size) as *mut libc::c_char;
    input_decompression_stream = ZSTD_createDStream();
    if input_decompression_stream.is_null() {
        die(b"can\'t create input decompression stream\n\x00" as *const u8 as
                *const libc::c_char);
    }
    file_bytes_to_read = ZSTD_initDStream(input_decompression_stream);
    if ZSTD_isError(file_bytes_to_read) != 0 {
        die(b"can\'t initialize input decompression stream: %s\n\x00" as
                *const u8 as *const libc::c_char,
            ZSTD_getErrorName(file_bytes_to_read));
    }
    if file_bytes_to_read < 5 as libc::c_int as libc::c_ulong {
        die(b"can\'t initialize decompression\n\x00" as *const u8 as
                *const libc::c_char);
    }
    put_magic_number(in_buffer as *mut libc::c_uchar);
    let mut could_read: size_t =
        fread(in_buffer.offset(4 as libc::c_int as isize) as
                  *mut libc::c_void, 1 as libc::c_int as size_t,
              file_bytes_to_read.wrapping_sub(4 as libc::c_int as
                                                  libc::c_ulong), IN);
    if could_read !=
           file_bytes_to_read.wrapping_sub(4 as libc::c_int as libc::c_ulong)
       {
        incomplete();
    }
    zstd_file_in_buffer.src = in_buffer as *const libc::c_void;
    zstd_file_in_buffer.size = file_bytes_to_read;
    zstd_file_in_buffer.pos = 0 as libc::c_int as size_t;
    let mut out: ZSTD_outBuffer =
        {
            let mut init =
                ZSTD_outBuffer_s{dst: quality_buffer as *mut libc::c_void,
                                 size: quality_buffer_size,
                                 pos: 0 as libc::c_int as size_t,};
            init
        };
    file_bytes_to_read =
        ZSTD_decompressStream(input_decompression_stream, &mut out,
                              &mut zstd_file_in_buffer);
    if ZSTD_isError(file_bytes_to_read) != 0 {
        die(b"can\'t decompress first quality block: %s\n\x00" as *const u8 as
                *const libc::c_char, ZSTD_getErrorName(file_bytes_to_read));
    }
    if zstd_file_in_buffer.pos != zstd_file_in_buffer.size {
        die(b"can\'t decompress first block\n\x00" as *const u8 as
                *const libc::c_char);
    }
    if out.pos != 0 as libc::c_int as libc::c_ulong {
        die(b"can\'t decompress first block\n\x00" as *const u8 as
                *const libc::c_char);
    }
    quality_buffer_filling_pos = out.pos as libc::c_uint;
    quality_buffer_remaining = out.pos as libc::c_uint;
}
#[inline]
unsafe extern "C" fn read_next_chunk(mut buffer: *mut libc::c_void,
                                     mut size: size_t) -> size_t {
    let mut could_read: size_t =
        fread(buffer, 1 as libc::c_int as size_t, size, IN);
    if could_read != size { incomplete(); }
    return could_read;
}
unsafe extern "C" fn refill_dna_buffer_from_memory_4bit() {
    dna_buffer_filling_pos = 0 as libc::c_int as libc::c_uint;
    while (dna_buffer_filling_pos as libc::c_ulong) < dna_buffer_flush_size &&
              (compressed_seq_pos < compressed_seq_size ||
                   zstd_mem_in_buffer.pos < zstd_mem_in_buffer.size) {
        if zstd_mem_in_buffer.pos >= zstd_mem_in_buffer.size {
            zstd_mem_in_buffer.src =
                compressed_seq_buffer.offset(compressed_seq_pos as isize) as
                    *const libc::c_void;
            zstd_mem_in_buffer.size = memory_bytes_to_read;
            zstd_mem_in_buffer.pos = 0 as libc::c_int as size_t;
            compressed_seq_pos =
                compressed_seq_pos.wrapping_add(memory_bytes_to_read as
                                                    libc::c_ulonglong)
        }
        let mut out: ZSTD_outBuffer =
            {
                let mut init =
                    ZSTD_outBuffer_s{dst: mem_out_buffer as *mut libc::c_void,
                                     size: mem_out_buffer_size,
                                     pos: 0 as libc::c_int as size_t,};
                init
            };
        memory_bytes_to_read =
            ZSTD_decompressStream(memory_decompression_stream, &mut out,
                                  &mut zstd_mem_in_buffer);
        if ZSTD_isError(memory_bytes_to_read) != 0 {
            die(b"can\'t decompress sequence from memory: %s\n\x00" as
                    *const u8 as *const libc::c_char,
                ZSTD_getErrorName(memory_bytes_to_read));
        }
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < out.pos {
            let fresh4 = dna_buffer_filling_pos;
            dna_buffer_filling_pos = dna_buffer_filling_pos.wrapping_add(1);
            *dna_buffer.offset(fresh4 as isize) =
                code_to_nuc[(*mem_out_buffer.offset(i as isize) as libc::c_int
                                 & 15 as libc::c_int) as usize];
            let fresh5 = dna_buffer_filling_pos;
            dna_buffer_filling_pos = dna_buffer_filling_pos.wrapping_add(1);
            *dna_buffer.offset(fresh5 as isize) =
                code_to_nuc[(*mem_out_buffer.offset(i as isize) as libc::c_int
                                 >> 4 as libc::c_int) as usize];
            i = i.wrapping_add(1)
        }
    }
    dna_buffer_remaining = dna_buffer_filling_pos;
    dna_buffer_printing_pos = 0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn refill_dna_buffer_from_memory() {
    dna_buffer_filling_pos = 0 as libc::c_int as libc::c_uint;
    while (dna_buffer_filling_pos as libc::c_ulong) < dna_buffer_flush_size &&
              (compressed_seq_pos < compressed_seq_size ||
                   zstd_mem_in_buffer.pos < zstd_mem_in_buffer.size) {
        if zstd_mem_in_buffer.pos >= zstd_mem_in_buffer.size {
            zstd_mem_in_buffer.src =
                compressed_seq_buffer.offset(compressed_seq_pos as isize) as
                    *const libc::c_void;
            zstd_mem_in_buffer.size = memory_bytes_to_read;
            zstd_mem_in_buffer.pos = 0 as libc::c_int as size_t;
            compressed_seq_pos =
                compressed_seq_pos.wrapping_add(memory_bytes_to_read as
                                                    libc::c_ulonglong)
        }
        let mut out: ZSTD_outBuffer =
            {
                let mut init =
                    ZSTD_outBuffer_s{dst:
                                         dna_buffer.offset(dna_buffer_filling_pos
                                                               as isize) as
                                             *mut libc::c_void,
                                     size:
                                         dna_buffer_size.wrapping_sub(dna_buffer_filling_pos
                                                                          as
                                                                          libc::c_ulong),
                                     pos: 0 as libc::c_int as size_t,};
                init
            };
        memory_bytes_to_read =
            ZSTD_decompressStream(memory_decompression_stream, &mut out,
                                  &mut zstd_mem_in_buffer);
        if ZSTD_isError(memory_bytes_to_read) != 0 {
            die(b"can\'t decompress sequence from memory: %s\n\x00" as
                    *const u8 as *const libc::c_char,
                ZSTD_getErrorName(memory_bytes_to_read));
        }
        dna_buffer_filling_pos =
            dna_buffer_filling_pos.wrapping_add(out.pos as libc::c_uint)
    }
    dna_buffer_remaining = dna_buffer_filling_pos;
    dna_buffer_printing_pos = 0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn refill_quality_buffer_from_file() {
    quality_buffer_filling_pos = 0 as libc::c_int as libc::c_uint;
    while (quality_buffer_filling_pos as libc::c_ulong) <
              quality_buffer_flush_size &&
              (file_bytes_to_read != 0 ||
                   zstd_file_in_buffer.pos < zstd_file_in_buffer.size) {
        if zstd_file_in_buffer.pos >= zstd_file_in_buffer.size {
            let mut input_size: size_t =
                read_next_chunk(in_buffer as *mut libc::c_void,
                                file_bytes_to_read);
            if input_size != file_bytes_to_read { incomplete(); }
            zstd_file_in_buffer.src = in_buffer as *const libc::c_void;
            zstd_file_in_buffer.size = file_bytes_to_read;
            zstd_file_in_buffer.pos = 0 as libc::c_int as size_t
        }
        let mut out: ZSTD_outBuffer =
            {
                let mut init =
                    ZSTD_outBuffer_s{dst:
                                         quality_buffer.offset(quality_buffer_filling_pos
                                                                   as isize)
                                             as *mut libc::c_void,
                                     size:
                                         quality_buffer_size.wrapping_sub(quality_buffer_filling_pos
                                                                              as
                                                                              libc::c_ulong),
                                     pos: 0 as libc::c_int as size_t,};
                init
            };
        file_bytes_to_read =
            ZSTD_decompressStream(input_decompression_stream, &mut out,
                                  &mut zstd_file_in_buffer);
        if ZSTD_isError(file_bytes_to_read) != 0 {
            die(b"can\'t decompress quality: %s\n\x00" as *const u8 as
                    *const libc::c_char,
                ZSTD_getErrorName(file_bytes_to_read));
        }
        quality_buffer_filling_pos =
            quality_buffer_filling_pos.wrapping_add(out.pos as libc::c_uint)
    }
    quality_buffer_remaining = quality_buffer_filling_pos;
    quality_buffer_printing_pos = 0 as libc::c_int as libc::c_uint;
}
/*
 * NAF decompressor
 * Copyright (c) 2018-2020 Kirill Kryukov
 * See README.md and LICENSE files of this repository
 */
unsafe extern "C" fn print_list_of_parts() {
    let mut printed: libc::c_int = 0 as libc::c_int;
    if has_title != 0 {
        fprintf(OUT, b"Title\x00" as *const u8 as *const libc::c_char);
        printed += 1
    }
    if has_ids != 0 {
        fprintf(OUT, b"%sIDs\x00" as *const u8 as *const libc::c_char,
                if printed != 0 {
                    b", \x00" as *const u8 as *const libc::c_char
                } else { b"\x00" as *const u8 as *const libc::c_char });
        printed += 1
    }
    if has_names != 0 {
        fprintf(OUT, b"%sNames\x00" as *const u8 as *const libc::c_char,
                if printed != 0 {
                    b", \x00" as *const u8 as *const libc::c_char
                } else { b"\x00" as *const u8 as *const libc::c_char });
        printed += 1
    }
    if has_lengths != 0 {
        fprintf(OUT, b"%sLengths\x00" as *const u8 as *const libc::c_char,
                if printed != 0 {
                    b", \x00" as *const u8 as *const libc::c_char
                } else { b"\x00" as *const u8 as *const libc::c_char });
        printed += 1
    }
    if has_mask != 0 {
        fprintf(OUT, b"%sMask\x00" as *const u8 as *const libc::c_char,
                if printed != 0 {
                    b", \x00" as *const u8 as *const libc::c_char
                } else { b"\x00" as *const u8 as *const libc::c_char });
        printed += 1
    }
    if has_data != 0 {
        fprintf(OUT, b"%sData\x00" as *const u8 as *const libc::c_char,
                if printed != 0 {
                    b", \x00" as *const u8 as *const libc::c_char
                } else { b"\x00" as *const u8 as *const libc::c_char });
        printed += 1
    }
    if has_quality != 0 {
        fprintf(OUT, b"%sQuality\x00" as *const u8 as *const libc::c_char,
                if printed != 0 {
                    b", \x00" as *const u8 as *const libc::c_char
                } else { b"\x00" as *const u8 as *const libc::c_char });
        printed += 1
    }
    fprintf(OUT, b"\n\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn print_part_sizes() {
    if has_title != 0 {
        let mut title_size: libc::c_ulonglong = read_number(IN);
        fprintf(OUT, b"Title: %llu\n\x00" as *const u8 as *const libc::c_char,
                title_size);
        skip_ahead(title_size);
    }
    if has_ids != 0 {
        let mut ids_size: libc::c_ulonglong = read_number(IN);
        let mut compressed_ids_size: libc::c_ulonglong = read_number(IN);
        fprintf(OUT,
                b"IDs: %llu / %llu (%.3f%%)\n\x00" as *const u8 as
                    *const libc::c_char, compressed_ids_size, ids_size,
                compressed_ids_size as libc::c_double /
                    ids_size as libc::c_double *
                    100 as libc::c_int as libc::c_double);
        skip_ahead(compressed_ids_size);
    }
    if has_names != 0 {
        let mut names_size: libc::c_ulonglong = read_number(IN);
        let mut compressed_names_size: libc::c_ulonglong = read_number(IN);
        fprintf(OUT,
                b"Names: %llu / %llu (%.3f%%)\n\x00" as *const u8 as
                    *const libc::c_char, compressed_names_size, names_size,
                compressed_names_size as libc::c_double /
                    names_size as libc::c_double *
                    100 as libc::c_int as libc::c_double);
        skip_ahead(compressed_names_size);
    }
    if has_lengths != 0 {
        let mut lengths_size: libc::c_ulonglong = read_number(IN);
        let mut compressed_lengths_size: libc::c_ulonglong = read_number(IN);
        fprintf(OUT,
                b"Lengths: %llu / %llu (%.3f%%)\n\x00" as *const u8 as
                    *const libc::c_char, compressed_lengths_size,
                lengths_size,
                compressed_lengths_size as libc::c_double /
                    lengths_size as libc::c_double *
                    100 as libc::c_int as libc::c_double);
        skip_ahead(compressed_lengths_size);
    }
    if has_mask != 0 {
        let mut mask_size_1: libc::c_ulonglong = read_number(IN);
        let mut compressed_mask_size: libc::c_ulonglong = read_number(IN);
        fprintf(OUT,
                b"Mask: %llu / %llu (%.3f%%)\n\x00" as *const u8 as
                    *const libc::c_char, compressed_mask_size, mask_size_1,
                compressed_mask_size as libc::c_double /
                    mask_size_1 as libc::c_double *
                    100 as libc::c_int as libc::c_double);
        skip_ahead(compressed_mask_size);
    }
    if has_data != 0 {
        let mut data_size: libc::c_ulonglong = read_number(IN);
        compressed_seq_size = read_number(IN);
        fprintf(OUT,
                b"Data: %llu / %llu (%.3f%%)\n\x00" as *const u8 as
                    *const libc::c_char, compressed_seq_size, data_size,
                compressed_seq_size as libc::c_double /
                    data_size as libc::c_double *
                    100 as libc::c_int as libc::c_double);
        skip_ahead(compressed_seq_size);
    }
    if has_quality != 0 {
        let mut quality_size: libc::c_ulonglong = read_number(IN);
        compressed_quality_size = read_number(IN);
        fprintf(OUT,
                b"Quality: %llu / %llu (%.3f%%)\n\x00" as *const u8 as
                    *const libc::c_char, compressed_quality_size,
                quality_size,
                compressed_quality_size as libc::c_double /
                    quality_size as libc::c_double *
                    100 as libc::c_int as libc::c_double);
        skip_ahead(compressed_quality_size);
    };
}
unsafe extern "C" fn print_title() {
    if has_title != 0 {
        let mut title_size: libc::c_ulonglong = read_number(IN);
        let mut title: *mut libc::c_char =
            malloc_or_die(title_size.wrapping_add(1 as libc::c_int as
                                                      libc::c_ulonglong) as
                              size_t) as *mut libc::c_char;
        if fread(title as *mut libc::c_void, 1 as libc::c_int as size_t,
                 title_size as size_t, IN) as libc::c_ulonglong != title_size
           {
            incomplete();
        }
        *title.offset(title_size as isize) = 0 as libc::c_int as libc::c_char;
        fputs(title, OUT);
        free(title as *mut libc::c_void);
    }
    fputc('\n' as i32, OUT);
}
unsafe extern "C" fn print_ids() {
    if has_ids != 0 {
        load_ids();
        let mut i: libc::c_ulonglong = 0 as libc::c_int as libc::c_ulonglong;
        while i < N {
            fprintf(OUT, b"%s\n\x00" as *const u8 as *const libc::c_char,
                    *ids.offset(i as isize));
            i = i.wrapping_add(1)
        }
    };
}
#[inline]
unsafe extern "C" fn print_name(mut index: libc::c_ulonglong) {
    if has_ids != 0 && has_names == 0 {
        fputs(*ids.offset(index as isize), OUT);
    } else if has_ids == 0 && has_names != 0 {
        fputs(*names.offset(index as isize), OUT);
    } else if has_ids != 0 && has_names != 0 {
        fputs(*ids.offset(index as isize), OUT);
        if *(*names.offset(index as isize)).offset(0 as libc::c_int as isize)
               as libc::c_int != 0 as libc::c_int {
            fputc(name_separator as libc::c_int, OUT);
            fputs(*names.offset(index as isize), OUT);
        }
    };
}
#[inline]
unsafe extern "C" fn print_fasta_name(mut index: libc::c_ulonglong) {
    fputc('>' as i32, OUT);
    print_name(index);
    fputc('\n' as i32, OUT);
}
#[inline]
unsafe extern "C" fn print_fastq_name(mut index: libc::c_ulonglong) {
    fputc('@' as i32, OUT);
    print_name(index);
    fputc('\n' as i32, OUT);
}
unsafe extern "C" fn print_names() {
    if has_ids != 0 || has_names != 0 {
        if has_ids != 0 { load_ids(); } else { skip_ids(); }
        if has_names != 0 { load_names(); }
        if has_ids != 0 && has_names == 0 {
            let mut i: libc::c_ulonglong =
                0 as libc::c_int as libc::c_ulonglong;
            while i < N {
                fprintf(OUT, b"%s\n\x00" as *const u8 as *const libc::c_char,
                        *ids.offset(i as isize));
                i = i.wrapping_add(1)
            }
        } else if has_ids == 0 && has_names != 0 {
            let mut i_0: libc::c_ulonglong =
                0 as libc::c_int as libc::c_ulonglong;
            while i_0 < N {
                fprintf(OUT, b"%s\n\x00" as *const u8 as *const libc::c_char,
                        *names.offset(i_0 as isize));
                i_0 = i_0.wrapping_add(1)
            }
        } else if has_ids != 0 && has_names != 0 {
            let mut i_1: libc::c_ulonglong =
                0 as libc::c_int as libc::c_ulonglong;
            while i_1 < N {
                fputs(*ids.offset(i_1 as isize), OUT);
                if *(*names.offset(i_1 as
                                       isize)).offset(0 as libc::c_int as
                                                          isize) as
                       libc::c_int != 0 as libc::c_int {
                    fputc(name_separator as libc::c_int, OUT);
                    fputs(*names.offset(i_1 as isize), OUT);
                }
                fputc('\n' as i32, OUT);
                i_1 = i_1.wrapping_add(1)
            }
        }
    };
}
unsafe extern "C" fn print_lengths() {
    if has_lengths != 0 {
        skip_ids();
        skip_names();
        load_lengths();
        let mut i: libc::c_ulonglong = 0 as libc::c_int as libc::c_ulonglong;
        while i < n_lengths {
            let mut len: libc::c_ulonglong =
                0 as libc::c_int as libc::c_ulonglong;
            while i < n_lengths &&
                      *lengths_buffer.offset(i as isize) ==
                          4294967295 as libc::c_uint {
                len = len.wrapping_add(4294967295 as libc::c_ulonglong);
                i = i.wrapping_add(1)
            }
            if i < n_lengths {
                len =
                    len.wrapping_add(*lengths_buffer.offset(i as isize) as
                                         libc::c_ulonglong)
            }
            fprintf(OUT, b"%llu\n\x00" as *const u8 as *const libc::c_char,
                    len);
            i = i.wrapping_add(1)
        }
    };
}
unsafe extern "C" fn print_total_length() {
    if has_lengths != 0 {
        skip_ids();
        skip_names();
        skip_lengths();
        skip_mask();
        total_seq_length = read_number(IN);
        fprintf(OUT, b"%llu\n\x00" as *const u8 as *const libc::c_char,
                total_seq_length);
    };
}
unsafe extern "C" fn print_mask() {
    if has_mask != 0 {
        skip_ids();
        skip_names();
        skip_lengths();
        load_mask();
        let mut i: libc::c_ulonglong = 0 as libc::c_int as libc::c_ulonglong;
        while i < mask_size {
            let mut len: libc::c_ulonglong =
                0 as libc::c_int as libc::c_ulonglong;
            while i < mask_size &&
                      *mask_buffer.offset(i as isize) as libc::c_uint ==
                          255 as libc::c_uint {
                len = len.wrapping_add(255 as libc::c_ulonglong);
                i = i.wrapping_add(1)
            }
            if i < mask_size {
                len =
                    len.wrapping_add(*mask_buffer.offset(i as isize) as
                                         libc::c_ulonglong)
            }
            fprintf(OUT, b"%llu\n\x00" as *const u8 as *const libc::c_char,
                    len);
            i = i.wrapping_add(1)
        }
    };
}
unsafe extern "C" fn print_total_mask_length() {
    if has_mask != 0 {
        skip_ids();
        skip_names();
        skip_lengths();
        load_mask();
        let mut total_mask_length: libc::c_ulonglong =
            0 as libc::c_int as libc::c_ulonglong;
        let mut i: libc::c_ulonglong = 0 as libc::c_int as libc::c_ulonglong;
        while i < mask_size {
            total_mask_length =
                total_mask_length.wrapping_add(*mask_buffer.offset(i as isize)
                                                   as libc::c_ulonglong);
            i = i.wrapping_add(1)
        }
        fprintf(OUT, b"%llu\n\x00" as *const u8 as *const libc::c_char,
                total_mask_length);
    } else { fprintf(OUT, b"0\n\x00" as *const u8 as *const libc::c_char); };
}
unsafe extern "C" fn print_4bit() {
    if has_data != 0 {
        skip_ids();
        skip_names();
        skip_lengths();
        skip_mask();
        read_number(IN);
        read_number(IN);
        let mut bytes_to_read: size_t = initialize_input_decompression();
        let mut input_size: size_t = 0;
        loop  {
            input_size =
                read_next_chunk(in_buffer as *mut libc::c_void,
                                bytes_to_read);
            if !(input_size != 0) { break ; }
            let mut in_0: ZSTD_inBuffer =
                {
                    let mut init =
                        ZSTD_inBuffer_s{src: in_buffer as *const libc::c_void,
                                        size: input_size,
                                        pos: 0 as libc::c_int as size_t,};
                    init
                };
            while in_0.pos < in_0.size {
                let mut out: ZSTD_outBuffer =
                    {
                        let mut init =
                            ZSTD_outBuffer_s{dst:
                                                 out_buffer as
                                                     *mut libc::c_void,
                                             size: out_buffer_size,
                                             pos:
                                                 0 as libc::c_int as size_t,};
                        init
                    };
                bytes_to_read =
                    ZSTD_decompressStream(input_decompression_stream,
                                          &mut out, &mut in_0);
                if ZSTD_isError(bytes_to_read) != 0 {
                    die(b"can\'t decompress: %s\n\x00" as *const u8 as
                            *const libc::c_char,
                        ZSTD_getErrorName(bytes_to_read));
                }
                fwrite(out_buffer as *const libc::c_void,
                       1 as libc::c_int as size_t, out.pos, OUT);
            }
        }
    };
}
#[inline]
unsafe extern "C" fn mask_dna_buffer(mut buffer: *mut libc::c_uchar,
                                     mut size: libc::c_uint) {
    let mut pos: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while pos < size {
        let mut advance: libc::c_uint = cur_mask_remaining;
        if advance > size.wrapping_sub(pos) {
            advance = size.wrapping_sub(pos)
        }
        let mut end_pos: libc::c_uint = pos.wrapping_add(advance);
        if mask_on != 0 {
            let mut i: libc::c_uint = pos;
            while i < end_pos {
                *buffer.offset(i as isize) =
                    (*buffer.offset(i as isize) as libc::c_int +
                         32 as libc::c_int) as libc::c_uchar;
                i = i.wrapping_add(1)
            }
        }
        cur_mask_remaining = cur_mask_remaining.wrapping_sub(advance);
        if cur_mask_remaining == 0 as libc::c_int as libc::c_uint {
            if *mask_buffer.offset(cur_mask as isize) as libc::c_int !=
                   255 as libc::c_int {
                mask_on = 1 as libc::c_int - mask_on
            }
            cur_mask = cur_mask.wrapping_add(1);
            cur_mask_remaining =
                *mask_buffer.offset(cur_mask as isize) as libc::c_uint
        }
        pos = end_pos
    };
}
#[inline]
unsafe extern "C" fn print_dna_buffer(mut masking: libc::c_int) {
    let mut n_bp_to_print: libc::c_ulonglong =
        dna_buffer_pos as libc::c_ulonglong;
    if n_bp_to_print > total_seq_n_bp_remaining {
        n_bp_to_print = total_seq_n_bp_remaining
    }
    if masking != 0 {
        mask_dna_buffer(dna_buffer, n_bp_to_print as libc::c_uint);
    }
    fwrite(dna_buffer as *const libc::c_void, 1 as libc::c_int as size_t,
           n_bp_to_print as size_t, OUT);
    total_seq_n_bp_remaining =
        total_seq_n_bp_remaining.wrapping_sub(n_bp_to_print);
    dna_buffer_pos = 0 as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn print_dna_split_into_lines(mut buffer:
                                                    *mut libc::c_uchar,
                                                mut size: size_t) {
    let mut print_pos: *mut libc::c_uchar = out_print_buffer;
    let mut pos: *mut libc::c_uchar = buffer;
    let mut remaining_bp: size_t = size;
    while remaining_bp as libc::c_ulonglong > cur_line_n_bp_remaining {
        memcpy(print_pos as *mut libc::c_void, pos as *const libc::c_void,
               cur_line_n_bp_remaining as libc::c_ulong);
        print_pos = print_pos.offset(cur_line_n_bp_remaining as isize);
        *print_pos = '\n' as i32 as libc::c_uchar;
        print_pos = print_pos.offset(1);
        pos = pos.offset(cur_line_n_bp_remaining as isize);
        remaining_bp =
            (remaining_bp as
                 libc::c_ulonglong).wrapping_sub(cur_line_n_bp_remaining) as
                size_t as size_t;
        cur_line_n_bp_remaining = max_line_length
    }
    memcpy(print_pos as *mut libc::c_void, pos as *const libc::c_void,
           remaining_bp);
    print_pos = print_pos.offset(remaining_bp as isize);
    cur_line_n_bp_remaining =
        cur_line_n_bp_remaining.wrapping_sub(remaining_bp as
                                                 libc::c_ulonglong);
    fwrite(out_print_buffer as *const libc::c_void,
           1 as libc::c_int as size_t,
           print_pos.wrapping_offset_from(out_print_buffer) as libc::c_long as
               size_t, OUT);
}
#[inline]
unsafe extern "C" fn uppercase_dna_buffer() {
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < dna_buffer_pos as libc::c_ulong {
        *dna_buffer.offset(i as isize) =
            ({
                 let mut __res: libc::c_int = 0;
                 if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong >
                        1 as libc::c_int as libc::c_ulong {
                     if 0 != 0 {
                         let mut __c: libc::c_int =
                             *dna_buffer.offset(i as isize) as libc::c_int;
                         __res =
                             if __c < -(128 as libc::c_int) ||
                                    __c > 255 as libc::c_int {
                                 __c
                             } else {
                                 *(*__ctype_toupper_loc()).offset(__c as
                                                                      isize)
                             }
                     } else {
                         __res =
                             toupper(*dna_buffer.offset(i as isize) as
                                         libc::c_int)
                     }
                 } else {
                     __res =
                         *(*__ctype_toupper_loc()).offset(*dna_buffer.offset(i
                                                                                 as
                                                                                 isize)
                                                              as libc::c_int
                                                              as isize)
                 }
                 __res
             }) as libc::c_uchar;
        i = i.wrapping_add(1)
    };
}
#[inline]
unsafe extern "C" fn print_dna_buffer_as_fasta(mut masking: libc::c_int) {
    let mut n_bp_to_print: libc::c_ulonglong =
        dna_buffer_pos as libc::c_ulonglong;
    if n_bp_to_print > total_seq_n_bp_remaining {
        n_bp_to_print = total_seq_n_bp_remaining
    }
    if masking != 0 {
        mask_dna_buffer(dna_buffer, n_bp_to_print as libc::c_uint);
    }
    let mut pos: *mut libc::c_uchar = dna_buffer;
    while n_bp_to_print >= cur_seq_len_n_bp_remaining {
        if cur_seq_len_n_bp_remaining > 0 as libc::c_int as libc::c_ulonglong
           {
            if max_line_length > 0 as libc::c_int as libc::c_ulonglong {
                print_dna_split_into_lines(pos,
                                           cur_seq_len_n_bp_remaining as
                                               size_t);
            } else {
                fwrite(pos as *const libc::c_void, 1 as libc::c_int as size_t,
                       cur_seq_len_n_bp_remaining as size_t, OUT);
            }
            pos = pos.offset(cur_seq_len_n_bp_remaining as isize);
            n_bp_to_print =
                n_bp_to_print.wrapping_sub(cur_seq_len_n_bp_remaining);
            total_seq_n_bp_remaining =
                total_seq_n_bp_remaining.wrapping_sub(cur_seq_len_n_bp_remaining)
        }
        if *lengths_buffer.offset(cur_seq_len_index as isize) !=
               4294967295 as libc::c_uint {
            fputc('\n' as i32, OUT);
            cur_seq_index = cur_seq_index.wrapping_add(1);
            if cur_seq_index < N {
                print_fasta_name(cur_seq_index);
                cur_line_n_bp_remaining = max_line_length
            }
        }
        cur_seq_len_index = cur_seq_len_index.wrapping_add(1);
        if cur_seq_len_index >= n_lengths { break ; }
        cur_seq_len_n_bp_remaining =
            *lengths_buffer.offset(cur_seq_len_index as isize) as
                libc::c_ulonglong
    }
    if n_bp_to_print > 0 as libc::c_int as libc::c_ulonglong {
        if max_line_length > 0 as libc::c_int as libc::c_ulonglong {
            print_dna_split_into_lines(pos, n_bp_to_print as size_t);
        } else {
            fwrite(pos as *const libc::c_void, 1 as libc::c_int as size_t,
                   n_bp_to_print as size_t, OUT);
        }
        cur_seq_len_n_bp_remaining =
            cur_seq_len_n_bp_remaining.wrapping_sub(n_bp_to_print);
        total_seq_n_bp_remaining =
            total_seq_n_bp_remaining.wrapping_sub(n_bp_to_print)
    }
    dna_buffer_pos = 0 as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn write_4bit_as_dna(mut buffer: *mut libc::c_uchar,
                                       mut size: size_t,
                                       mut masking: libc::c_int) {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < size {
        *(&mut *dna_buffer.offset(dna_buffer_pos as isize) as
              *mut libc::c_uchar as *mut libc::c_ushort) =
            codes_to_nucs[*buffer.offset(i as isize) as usize];
        dna_buffer_pos =
            dna_buffer_pos.wrapping_add(2 as libc::c_int as libc::c_uint);
        i = i.wrapping_add(1)
    }
    if dna_buffer_pos as libc::c_ulong > dna_buffer_flush_size {
        print_dna_buffer(masking);
    };
}
#[inline]
unsafe extern "C" fn write_4bit_as_fasta(mut buffer: *mut libc::c_uchar,
                                         mut size: size_t,
                                         mut masking: libc::c_int) {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < size {
        *(&mut *dna_buffer.offset(dna_buffer_pos as isize) as
              *mut libc::c_uchar as *mut libc::c_ushort) =
            codes_to_nucs[*buffer.offset(i as isize) as usize];
        dna_buffer_pos =
            dna_buffer_pos.wrapping_add(2 as libc::c_int as libc::c_uint);
        i = i.wrapping_add(1)
    }
    if dna_buffer_pos as libc::c_ulong > dna_buffer_flush_size {
        print_dna_buffer_as_fasta(masking);
    };
}
unsafe extern "C" fn print_dna(mut masking: libc::c_int) {
    if has_data != 0 {
        skip_ids();
        skip_names();
        skip_lengths();
        if masking != 0 { load_mask(); } else { skip_mask(); }
        total_seq_length = read_number(IN);
        compressed_seq_size = read_number(IN);
        total_seq_n_bp_remaining = total_seq_length;
        let mut bytes_to_read: size_t = initialize_input_decompression();
        let mut input_size: size_t = 0;
        if in_seq_type < seq_type_protein as libc::c_int {
            loop  {
                input_size =
                    read_next_chunk(in_buffer as *mut libc::c_void,
                                    bytes_to_read);
                if !(input_size != 0) { break ; }
                let mut in_0: ZSTD_inBuffer =
                    {
                        let mut init =
                            ZSTD_inBuffer_s{src:
                                                in_buffer as
                                                    *const libc::c_void,
                                            size: input_size,
                                            pos: 0 as libc::c_int as size_t,};
                        init
                    };
                while in_0.pos < in_0.size {
                    let mut out: ZSTD_outBuffer =
                        {
                            let mut init =
                                ZSTD_outBuffer_s{dst:
                                                     out_buffer as
                                                         *mut libc::c_void,
                                                 size: out_buffer_size,
                                                 pos:
                                                     0 as libc::c_int as
                                                         size_t,};
                            init
                        };
                    bytes_to_read =
                        ZSTD_decompressStream(input_decompression_stream,
                                              &mut out, &mut in_0);
                    if ZSTD_isError(bytes_to_read) != 0 {
                        die(b"can\'t decompress sequence: %s\n\x00" as
                                *const u8 as *const libc::c_char,
                            ZSTD_getErrorName(bytes_to_read));
                    }
                    write_4bit_as_dna(out_buffer as *mut libc::c_uchar,
                                      out.pos, masking);
                }
            }
        } else {
            loop  {
                input_size =
                    read_next_chunk(in_buffer as *mut libc::c_void,
                                    bytes_to_read);
                if !(input_size != 0) { break ; }
                let mut in_1: ZSTD_inBuffer =
                    {
                        let mut init =
                            ZSTD_inBuffer_s{src:
                                                in_buffer as
                                                    *const libc::c_void,
                                            size: input_size,
                                            pos: 0 as libc::c_int as size_t,};
                        init
                    };
                while in_1.pos < in_1.size {
                    let mut out_0: ZSTD_outBuffer =
                        {
                            let mut init =
                                ZSTD_outBuffer_s{dst:
                                                     dna_buffer as
                                                         *mut libc::c_void,
                                                 size: dna_buffer_size,
                                                 pos:
                                                     0 as libc::c_int as
                                                         size_t,};
                            init
                        };
                    bytes_to_read =
                        ZSTD_decompressStream(input_decompression_stream,
                                              &mut out_0, &mut in_1);
                    if ZSTD_isError(bytes_to_read) != 0 {
                        die(b"can\'t decompress sequence: %s\n\x00" as
                                *const u8 as *const libc::c_char,
                            ZSTD_getErrorName(bytes_to_read));
                    }
                    dna_buffer_pos = out_0.pos as libc::c_uint;
                    if !use_mask { uppercase_dna_buffer(); }
                    print_dna_buffer(masking);
                }
            }
        }
        if total_seq_n_bp_remaining > 0 as libc::c_int as libc::c_ulonglong {
            if in_seq_type >= seq_type_protein as libc::c_int && !use_mask {
                uppercase_dna_buffer();
            }
            print_dna_buffer(masking);
        }
    };
}
unsafe extern "C" fn count_dna_buffer_sequence_characters(mut counts:
                                                              *mut libc::c_ulonglong,
                                                          mut masking:
                                                              libc::c_int) {
    let mut n_bp_to_print: libc::c_ulonglong =
        dna_buffer_pos as libc::c_ulonglong;
    if n_bp_to_print > total_seq_n_bp_remaining {
        n_bp_to_print = total_seq_n_bp_remaining
    }
    if masking != 0 {
        mask_dna_buffer(dna_buffer, n_bp_to_print as libc::c_uint);
    }
    let mut end: *mut libc::c_uchar =
        dna_buffer.offset(n_bp_to_print as isize);
    let mut c: *mut libc::c_uchar = dna_buffer;
    while c < end {
        let ref mut fresh6 = *counts.offset(*c as isize);
        *fresh6 = (*fresh6).wrapping_add(1);
        c = c.offset(1)
    }
    total_seq_n_bp_remaining =
        total_seq_n_bp_remaining.wrapping_sub(n_bp_to_print);
    dna_buffer_pos = 0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn count_4bit_sequence_characters(mut counts:
                                                        *mut libc::c_ulonglong,
                                                    mut buffer:
                                                        *mut libc::c_uchar,
                                                    mut size: size_t,
                                                    mut masking:
                                                        libc::c_int) {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < size {
        *(&mut *dna_buffer.offset(dna_buffer_pos as isize) as
              *mut libc::c_uchar as *mut libc::c_ushort) =
            codes_to_nucs[*buffer.offset(i as isize) as usize];
        dna_buffer_pos =
            dna_buffer_pos.wrapping_add(2 as libc::c_int as libc::c_uint);
        i = i.wrapping_add(1)
    }
    count_dna_buffer_sequence_characters(counts, masking);
}
unsafe extern "C" fn print_charcount(mut masking: libc::c_int) {
    if has_data == 0 { return }
    skip_ids();
    skip_names();
    skip_lengths();
    if masking != 0 { load_mask(); } else { skip_mask(); }
    let mut counts: [libc::c_ulonglong; 256] = [0; 256];
    memset(counts.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<libc::c_ulonglong>() as
                libc::c_ulong).wrapping_mul(256 as libc::c_int as
                                                libc::c_ulong));
    total_seq_length = read_number(IN);
    compressed_seq_size = read_number(IN);
    total_seq_n_bp_remaining = total_seq_length;
    let mut bytes_to_read: size_t = initialize_input_decompression();
    let mut input_size: size_t = 0;
    if in_seq_type < seq_type_protein as libc::c_int {
        loop  {
            input_size =
                read_next_chunk(in_buffer as *mut libc::c_void,
                                bytes_to_read);
            if !(input_size != 0) { break ; }
            let mut in_0: ZSTD_inBuffer =
                {
                    let mut init =
                        ZSTD_inBuffer_s{src: in_buffer as *const libc::c_void,
                                        size: input_size,
                                        pos: 0 as libc::c_int as size_t,};
                    init
                };
            while in_0.pos < in_0.size {
                let mut out: ZSTD_outBuffer =
                    {
                        let mut init =
                            ZSTD_outBuffer_s{dst:
                                                 out_buffer as
                                                     *mut libc::c_void,
                                             size: out_buffer_size,
                                             pos:
                                                 0 as libc::c_int as size_t,};
                        init
                    };
                bytes_to_read =
                    ZSTD_decompressStream(input_decompression_stream,
                                          &mut out, &mut in_0);
                if ZSTD_isError(bytes_to_read) != 0 {
                    die(b"can\'t decompress sequence: %s\n\x00" as *const u8
                            as *const libc::c_char,
                        ZSTD_getErrorName(bytes_to_read));
                }
                count_4bit_sequence_characters(counts.as_mut_ptr(),
                                               out_buffer as
                                                   *mut libc::c_uchar,
                                               out.pos, masking);
            }
        }
    } else {
        loop  {
            input_size =
                read_next_chunk(in_buffer as *mut libc::c_void,
                                bytes_to_read);
            if !(input_size != 0) { break ; }
            let mut in_1: ZSTD_inBuffer =
                {
                    let mut init =
                        ZSTD_inBuffer_s{src: in_buffer as *const libc::c_void,
                                        size: input_size,
                                        pos: 0 as libc::c_int as size_t,};
                    init
                };
            while in_1.pos < in_1.size {
                let mut out_0: ZSTD_outBuffer =
                    {
                        let mut init =
                            ZSTD_outBuffer_s{dst:
                                                 dna_buffer as
                                                     *mut libc::c_void,
                                             size: dna_buffer_size,
                                             pos:
                                                 0 as libc::c_int as size_t,};
                        init
                    };
                bytes_to_read =
                    ZSTD_decompressStream(input_decompression_stream,
                                          &mut out_0, &mut in_1);
                if ZSTD_isError(bytes_to_read) != 0 {
                    die(b"can\'t decompress sequence: %s\n\x00" as *const u8
                            as *const libc::c_char,
                        ZSTD_getErrorName(bytes_to_read));
                }
                dna_buffer_pos = out_0.pos as libc::c_uint;
                if !use_mask { uppercase_dna_buffer(); }
                count_dna_buffer_sequence_characters(counts.as_mut_ptr(),
                                                     masking);
            }
        }
    }
    if total_seq_n_bp_remaining > 0 as libc::c_int as libc::c_ulonglong {
        if in_seq_type >= seq_type_protein as libc::c_int && !use_mask {
            uppercase_dna_buffer();
        }
        count_dna_buffer_sequence_characters(counts.as_mut_ptr(), masking);
    }
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < 33 as libc::c_int as libc::c_uint {
        if counts[i as usize] != 0 as libc::c_int as libc::c_ulonglong {
            fprintf(OUT,
                    b"\\x%02X\t%llu\n\x00" as *const u8 as
                        *const libc::c_char, i, counts[i as usize]);
        }
        i = i.wrapping_add(1)
    }
    let mut i_0: libc::c_uint = 33 as libc::c_int as libc::c_uint;
    while i_0 < 127 as libc::c_int as libc::c_uint {
        if counts[i_0 as usize] != 0 as libc::c_int as libc::c_ulonglong {
            fprintf(OUT,
                    b"%c\t%llu\n\x00" as *const u8 as *const libc::c_char,
                    i_0 as libc::c_uchar as libc::c_int,
                    counts[i_0 as usize]);
        }
        i_0 = i_0.wrapping_add(1)
    }
    let mut i_1: libc::c_uint = 127 as libc::c_int as libc::c_uint;
    while i_1 < 256 as libc::c_int as libc::c_uint {
        if counts[i_1 as usize] != 0 as libc::c_int as libc::c_ulonglong {
            fprintf(OUT,
                    b"\\x%02X\t%llu\n\x00" as *const u8 as
                        *const libc::c_char, i_1, counts[i_1 as usize]);
        }
        i_1 = i_1.wrapping_add(1)
    };
}
unsafe extern "C" fn print_fasta(mut masking: libc::c_int) {
    if has_data == 0 { return }
    load_ids();
    load_names();
    load_lengths();
    if masking != 0 { load_mask(); } else { skip_mask(); }
    total_seq_length = read_number(IN);
    compressed_seq_size = read_number(IN);
    total_seq_n_bp_remaining = total_seq_length;
    cur_seq_len_n_bp_remaining =
        *lengths_buffer.offset(0 as libc::c_int as isize) as
            libc::c_ulonglong;
    print_fasta_name(0 as libc::c_int as libc::c_ulonglong);
    cur_line_n_bp_remaining = max_line_length;
    let mut bytes_to_read: size_t = initialize_input_decompression();
    let mut input_size: size_t = 0;
    if in_seq_type < seq_type_protein as libc::c_int {
        while total_seq_n_bp_remaining > 0 as libc::c_int as libc::c_ulonglong
                  &&
                  {
                      input_size =
                          read_next_chunk(in_buffer as *mut libc::c_void,
                                          bytes_to_read);
                      (input_size) != 0
                  } {
            let mut in_0: ZSTD_inBuffer =
                {
                    let mut init =
                        ZSTD_inBuffer_s{src: in_buffer as *const libc::c_void,
                                        size: input_size,
                                        pos: 0 as libc::c_int as size_t,};
                    init
                };
            while in_0.pos < in_0.size {
                let mut out: ZSTD_outBuffer =
                    {
                        let mut init =
                            ZSTD_outBuffer_s{dst:
                                                 out_buffer as
                                                     *mut libc::c_void,
                                             size: out_buffer_size,
                                             pos:
                                                 0 as libc::c_int as size_t,};
                        init
                    };
                bytes_to_read =
                    ZSTD_decompressStream(input_decompression_stream,
                                          &mut out, &mut in_0);
                if ZSTD_isError(bytes_to_read) != 0 {
                    die(b"can\'t decompress sequence: %s\n\x00" as *const u8
                            as *const libc::c_char,
                        ZSTD_getErrorName(bytes_to_read));
                }
                write_4bit_as_fasta(out_buffer as *mut libc::c_uchar, out.pos,
                                    masking);
            }
        }
    } else {
        while total_seq_n_bp_remaining > 0 as libc::c_int as libc::c_ulonglong
                  &&
                  {
                      input_size =
                          read_next_chunk(in_buffer as *mut libc::c_void,
                                          bytes_to_read);
                      (input_size) != 0
                  } {
            let mut in_1: ZSTD_inBuffer =
                {
                    let mut init =
                        ZSTD_inBuffer_s{src: in_buffer as *const libc::c_void,
                                        size: input_size,
                                        pos: 0 as libc::c_int as size_t,};
                    init
                };
            while in_1.pos < in_1.size {
                let mut out_0: ZSTD_outBuffer =
                    {
                        let mut init =
                            ZSTD_outBuffer_s{dst:
                                                 dna_buffer as
                                                     *mut libc::c_void,
                                             size: dna_buffer_size,
                                             pos:
                                                 0 as libc::c_int as size_t,};
                        init
                    };
                bytes_to_read =
                    ZSTD_decompressStream(input_decompression_stream,
                                          &mut out_0, &mut in_1);
                if ZSTD_isError(bytes_to_read) != 0 {
                    die(b"can\'t decompress sequence: %s\n\x00" as *const u8
                            as *const libc::c_char,
                        ZSTD_getErrorName(bytes_to_read));
                }
                dna_buffer_pos = out_0.pos as libc::c_uint;
                if !use_mask { uppercase_dna_buffer(); }
                print_dna_buffer_as_fasta(masking);
            }
        }
    }
    if total_seq_n_bp_remaining > 0 as libc::c_int as libc::c_ulonglong {
        if in_seq_type >= seq_type_protein as libc::c_int && !use_mask {
            uppercase_dna_buffer();
        }
        print_dna_buffer_as_fasta(masking);
    };
}
/*
 * NAF decompressor
 * Copyright (c) 2018-2020 Kirill Kryukov
 * See README.md and LICENSE files of this repository
 */
#[inline]
unsafe extern "C" fn print_dna_buffer_as_sequences(mut masking: libc::c_int) {
    let mut n_bp_to_print: libc::c_ulonglong =
        dna_buffer_pos as libc::c_ulonglong;
    if n_bp_to_print > total_seq_n_bp_remaining {
        n_bp_to_print = total_seq_n_bp_remaining
    }
    if masking != 0 {
        mask_dna_buffer(dna_buffer, n_bp_to_print as libc::c_uint);
    }
    let mut pos: *mut libc::c_uchar = dna_buffer;
    while n_bp_to_print >= cur_seq_len_n_bp_remaining {
        if cur_seq_len_n_bp_remaining > 0 as libc::c_int as libc::c_ulonglong
           {
            fwrite(pos as *const libc::c_void, 1 as libc::c_int as size_t,
                   cur_seq_len_n_bp_remaining as size_t, OUT);
            pos = pos.offset(cur_seq_len_n_bp_remaining as isize);
            n_bp_to_print =
                n_bp_to_print.wrapping_sub(cur_seq_len_n_bp_remaining);
            total_seq_n_bp_remaining =
                total_seq_n_bp_remaining.wrapping_sub(cur_seq_len_n_bp_remaining)
        }
        if *lengths_buffer.offset(cur_seq_len_index as isize) !=
               4294967295 as libc::c_uint {
            fputc('\n' as i32, OUT);
            cur_seq_index = cur_seq_index.wrapping_add(1)
        }
        cur_seq_len_index = cur_seq_len_index.wrapping_add(1);
        if cur_seq_len_index >= n_lengths { break ; }
        cur_seq_len_n_bp_remaining =
            *lengths_buffer.offset(cur_seq_len_index as isize) as
                libc::c_ulonglong
    }
    if n_bp_to_print > 0 as libc::c_int as libc::c_ulonglong {
        fwrite(pos as *const libc::c_void, 1 as libc::c_int as size_t,
               n_bp_to_print as size_t, OUT);
        cur_seq_len_n_bp_remaining =
            cur_seq_len_n_bp_remaining.wrapping_sub(n_bp_to_print);
        total_seq_n_bp_remaining =
            total_seq_n_bp_remaining.wrapping_sub(n_bp_to_print)
    }
    dna_buffer_pos = 0 as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn write_4bit_as_sequences(mut buffer: *mut libc::c_uchar,
                                             mut size: size_t,
                                             mut masking: libc::c_int) {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < size {
        *(&mut *dna_buffer.offset(dna_buffer_pos as isize) as
              *mut libc::c_uchar as *mut libc::c_ushort) =
            codes_to_nucs[*buffer.offset(i as isize) as usize];
        dna_buffer_pos =
            dna_buffer_pos.wrapping_add(2 as libc::c_int as libc::c_uint);
        i = i.wrapping_add(1)
    }
    if dna_buffer_pos as libc::c_ulong > dna_buffer_flush_size {
        print_dna_buffer_as_sequences(masking);
    };
}
unsafe extern "C" fn print_sequences(mut masking: libc::c_int) {
    if has_data == 0 { return }
    skip_ids();
    skip_names();
    load_lengths();
    if masking != 0 { load_mask(); } else { skip_mask(); }
    total_seq_length = read_number(IN);
    compressed_seq_size = read_number(IN);
    total_seq_n_bp_remaining = total_seq_length;
    cur_seq_len_n_bp_remaining =
        *lengths_buffer.offset(0 as libc::c_int as isize) as
            libc::c_ulonglong;
    let mut bytes_to_read: size_t = initialize_input_decompression();
    let mut input_size: size_t = 0;
    if in_seq_type < seq_type_protein as libc::c_int {
        while total_seq_n_bp_remaining > 0 as libc::c_int as libc::c_ulonglong
                  &&
                  {
                      input_size =
                          read_next_chunk(in_buffer as *mut libc::c_void,
                                          bytes_to_read);
                      (input_size) != 0
                  } {
            let mut in_0: ZSTD_inBuffer =
                {
                    let mut init =
                        ZSTD_inBuffer_s{src: in_buffer as *const libc::c_void,
                                        size: input_size,
                                        pos: 0 as libc::c_int as size_t,};
                    init
                };
            while in_0.pos < in_0.size {
                let mut out: ZSTD_outBuffer =
                    {
                        let mut init =
                            ZSTD_outBuffer_s{dst:
                                                 out_buffer as
                                                     *mut libc::c_void,
                                             size: out_buffer_size,
                                             pos:
                                                 0 as libc::c_int as size_t,};
                        init
                    };
                bytes_to_read =
                    ZSTD_decompressStream(input_decompression_stream,
                                          &mut out, &mut in_0);
                if ZSTD_isError(bytes_to_read) != 0 {
                    die(b"can\'t decompress sequence: %s\n\x00" as *const u8
                            as *const libc::c_char,
                        ZSTD_getErrorName(bytes_to_read));
                }
                write_4bit_as_sequences(out_buffer as *mut libc::c_uchar,
                                        out.pos, masking);
            }
        }
    } else {
        while total_seq_n_bp_remaining > 0 as libc::c_int as libc::c_ulonglong
                  &&
                  {
                      input_size =
                          read_next_chunk(in_buffer as *mut libc::c_void,
                                          bytes_to_read);
                      (input_size) != 0
                  } {
            let mut in_1: ZSTD_inBuffer =
                {
                    let mut init =
                        ZSTD_inBuffer_s{src: in_buffer as *const libc::c_void,
                                        size: input_size,
                                        pos: 0 as libc::c_int as size_t,};
                    init
                };
            while in_1.pos < in_1.size {
                let mut out_0: ZSTD_outBuffer =
                    {
                        let mut init =
                            ZSTD_outBuffer_s{dst:
                                                 dna_buffer as
                                                     *mut libc::c_void,
                                             size: dna_buffer_size,
                                             pos:
                                                 0 as libc::c_int as size_t,};
                        init
                    };
                bytes_to_read =
                    ZSTD_decompressStream(input_decompression_stream,
                                          &mut out_0, &mut in_1);
                if ZSTD_isError(bytes_to_read) != 0 {
                    die(b"can\'t decompress sequence: %s\n\x00" as *const u8
                            as *const libc::c_char,
                        ZSTD_getErrorName(bytes_to_read));
                }
                dna_buffer_pos = out_0.pos as libc::c_uint;
                if !use_mask { uppercase_dna_buffer(); }
                print_dna_buffer_as_sequences(masking);
            }
        }
    }
    if total_seq_n_bp_remaining > 0 as libc::c_int as libc::c_ulonglong {
        if in_seq_type >= seq_type_protein as libc::c_int && !use_mask {
            uppercase_dna_buffer();
        }
        print_dna_buffer_as_sequences(masking);
    };
}
/*
 * NAF decompressor
 * Copyright (c) 2018-2020 Kirill Kryukov
 * See README.md and LICENSE files of this repository
 */
unsafe extern "C" fn print_dna_from_memory_4bit(mut len: libc::c_uint) {
    let mut remaining_bp: libc::c_uint = len;
    while remaining_bp > 0 as libc::c_int as libc::c_uint {
        if dna_buffer_remaining == 0 as libc::c_int as libc::c_uint {
            refill_dna_buffer_from_memory_4bit();
        }
        let mut n_bp_to_print: libc::c_uint = remaining_bp;
        if n_bp_to_print > dna_buffer_remaining {
            n_bp_to_print = dna_buffer_remaining
        }
        fwrite(dna_buffer.offset(dna_buffer_printing_pos as isize) as
                   *const libc::c_void, 1 as libc::c_int as size_t,
               n_bp_to_print as size_t, OUT);
        dna_buffer_printing_pos =
            dna_buffer_printing_pos.wrapping_add(n_bp_to_print);
        dna_buffer_remaining =
            dna_buffer_remaining.wrapping_sub(n_bp_to_print);
        remaining_bp = remaining_bp.wrapping_sub(n_bp_to_print)
    };
}
unsafe extern "C" fn print_dna_from_memory(mut len: libc::c_uint) {
    let mut remaining_bp: libc::c_uint = len;
    while remaining_bp > 0 as libc::c_int as libc::c_uint {
        if dna_buffer_remaining == 0 as libc::c_int as libc::c_uint {
            refill_dna_buffer_from_memory();
        }
        let mut n_bp_to_print: libc::c_uint = remaining_bp;
        if n_bp_to_print > dna_buffer_remaining {
            n_bp_to_print = dna_buffer_remaining
        }
        fwrite(dna_buffer.offset(dna_buffer_printing_pos as isize) as
                   *const libc::c_void, 1 as libc::c_int as size_t,
               n_bp_to_print as size_t, OUT);
        dna_buffer_printing_pos =
            dna_buffer_printing_pos.wrapping_add(n_bp_to_print);
        dna_buffer_remaining =
            dna_buffer_remaining.wrapping_sub(n_bp_to_print);
        remaining_bp = remaining_bp.wrapping_sub(n_bp_to_print)
    };
}
unsafe extern "C" fn print_next_sequence_from_memory_4bit() {
    while *lengths_buffer.offset(cur_seq_len_index as isize) ==
              4294967295 as libc::c_uint {
        print_dna_from_memory_4bit(*lengths_buffer.offset(cur_seq_len_index as
                                                              isize));
        cur_seq_len_index = cur_seq_len_index.wrapping_add(1)
    }
    print_dna_from_memory_4bit(*lengths_buffer.offset(cur_seq_len_index as
                                                          isize));
    cur_seq_len_index = cur_seq_len_index.wrapping_add(1);
    fputc('\n' as i32, OUT);
}
unsafe extern "C" fn print_next_sequence_from_memory() {
    while *lengths_buffer.offset(cur_seq_len_index as isize) ==
              4294967295 as libc::c_uint {
        print_dna_from_memory(*lengths_buffer.offset(cur_seq_len_index as
                                                         isize));
        cur_seq_len_index = cur_seq_len_index.wrapping_add(1)
    }
    print_dna_from_memory(*lengths_buffer.offset(cur_seq_len_index as isize));
    cur_seq_len_index = cur_seq_len_index.wrapping_add(1);
    fputc('\n' as i32, OUT);
}
unsafe extern "C" fn print_quality_from_file(mut len: libc::c_uint) {
    let mut remaining_bp: libc::c_uint = len;
    while remaining_bp > 0 as libc::c_int as libc::c_uint {
        if quality_buffer_remaining == 0 as libc::c_int as libc::c_uint {
            refill_quality_buffer_from_file();
        }
        let mut n_bp_to_print: libc::c_uint = remaining_bp;
        if n_bp_to_print > quality_buffer_remaining {
            n_bp_to_print = quality_buffer_remaining
        }
        fwrite(quality_buffer.offset(quality_buffer_printing_pos as isize) as
                   *const libc::c_void, 1 as libc::c_int as size_t,
               n_bp_to_print as size_t, OUT);
        quality_buffer_printing_pos =
            quality_buffer_printing_pos.wrapping_add(n_bp_to_print);
        quality_buffer_remaining =
            quality_buffer_remaining.wrapping_sub(n_bp_to_print);
        remaining_bp = remaining_bp.wrapping_sub(n_bp_to_print)
    };
}
unsafe extern "C" fn print_next_quality_from_file() {
    while *lengths_buffer.offset(cur_qual_len_index as isize) ==
              4294967295 as libc::c_uint {
        print_quality_from_file(*lengths_buffer.offset(cur_qual_len_index as
                                                           isize));
        cur_qual_len_index = cur_qual_len_index.wrapping_add(1)
    }
    print_quality_from_file(*lengths_buffer.offset(cur_qual_len_index as
                                                       isize));
    cur_qual_len_index = cur_qual_len_index.wrapping_add(1);
    fputc('\n' as i32, OUT);
}
unsafe extern "C" fn print_fastq(mut masking: libc::c_int) {
    if has_data != 0 {
        quality_buffer_flush_size = ZSTD_DStreamOutSize();
        quality_buffer_size =
            quality_buffer_flush_size.wrapping_mul(2 as libc::c_int as
                                                       libc::c_ulong).wrapping_add(10
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_ulong);
        quality_buffer =
            malloc_or_die(quality_buffer_size) as *mut libc::c_char;
        load_ids();
        load_names();
        load_lengths();
        if masking != 0 { load_mask(); } else { skip_mask(); }
        load_compressed_sequence();
        initialize_memory_decompression();
        zstd_mem_in_buffer.src = compressed_seq_buffer as *const libc::c_void;
        zstd_mem_in_buffer.size = memory_bytes_to_read;
        zstd_mem_in_buffer.pos = 0 as libc::c_int as size_t;
        compressed_seq_pos = memory_bytes_to_read as libc::c_ulonglong;
        total_quality_length = read_number(IN);
        compressed_quality_size = read_number(IN);
        initialize_quality_file_decompression();
        if in_seq_type < seq_type_protein as libc::c_int {
            let mut ri: libc::c_ulonglong =
                0 as libc::c_int as libc::c_ulonglong;
            while ri < N {
                print_fastq_name(ri);
                print_next_sequence_from_memory_4bit();
                fputs(b"+\n\x00" as *const u8 as *const libc::c_char, OUT);
                print_next_quality_from_file();
                ri = ri.wrapping_add(1)
            }
        } else {
            let mut ri_0: libc::c_ulonglong =
                0 as libc::c_int as libc::c_ulonglong;
            while ri_0 < N {
                print_fastq_name(ri_0);
                print_next_sequence_from_memory();
                fputs(b"+\n\x00" as *const u8 as *const libc::c_char, OUT);
                print_next_quality_from_file();
                ri_0 = ri_0.wrapping_add(1)
            }
        }
    };
}
unsafe extern "C" fn done() {
    if !IN.is_null() && IN != stdin { fclose(IN); IN = 0 as *mut FILE }
    close_input_file();
    close_output_file();
    if !ids.is_null() {
        free(ids as *mut libc::c_void);
        ids = 0 as *mut *mut libc::c_char
    }
    if !ids_buffer.is_null() {
        free(ids_buffer as *mut libc::c_void);
        ids_buffer = 0 as *mut libc::c_char
    }
    if !compressed_ids_buffer.is_null() {
        free(compressed_ids_buffer as *mut libc::c_void);
        compressed_ids_buffer = 0 as *mut libc::c_uchar
    }
    if !names.is_null() {
        free(names as *mut libc::c_void);
        names = 0 as *mut *mut libc::c_char
    }
    if !names_buffer.is_null() {
        free(names_buffer as *mut libc::c_void);
        names_buffer = 0 as *mut libc::c_char
    }
    if !compressed_names_buffer.is_null() {
        free(compressed_names_buffer as *mut libc::c_void);
        compressed_names_buffer = 0 as *mut libc::c_uchar
    }
    if !lengths_buffer.is_null() {
        free(lengths_buffer as *mut libc::c_void);
        lengths_buffer = 0 as *mut libc::c_uint
    }
    if !compressed_lengths_buffer.is_null() {
        free(compressed_lengths_buffer as *mut libc::c_void);
        compressed_lengths_buffer = 0 as *mut libc::c_uchar
    }
    if !mask_buffer.is_null() {
        free(mask_buffer as *mut libc::c_void);
        mask_buffer = 0 as *mut libc::c_uchar
    }
    if !compressed_mask_buffer.is_null() {
        free(compressed_mask_buffer as *mut libc::c_void);
        compressed_mask_buffer = 0 as *mut libc::c_uchar
    }
    if !compressed_seq_buffer.is_null() {
        free(compressed_seq_buffer as *mut libc::c_void);
        compressed_seq_buffer = 0 as *mut libc::c_uchar
    }
    if !in_buffer.is_null() {
        free(in_buffer as *mut libc::c_void);
        in_buffer = 0 as *mut libc::c_char
    }
    if !out_buffer.is_null() {
        free(out_buffer as *mut libc::c_void);
        out_buffer = 0 as *mut libc::c_char
    }
    if !mem_out_buffer.is_null() {
        free(mem_out_buffer as *mut libc::c_void);
        mem_out_buffer = 0 as *mut libc::c_uchar
    }
    if !out_print_buffer.is_null() {
        free(out_print_buffer as *mut libc::c_void);
        out_print_buffer = 0 as *mut libc::c_uchar
    }
    if !input_decompression_stream.is_null() {
        free(input_decompression_stream as *mut libc::c_void);
        input_decompression_stream = 0 as *mut ZSTD_DStream
    }
    if !memory_decompression_stream.is_null() {
        free(memory_decompression_stream as *mut libc::c_void);
        memory_decompression_stream = 0 as *mut ZSTD_DStream
    }
    if !dna_buffer.is_null() {
        free(dna_buffer as *mut libc::c_void);
        dna_buffer = 0 as *mut libc::c_uchar
    }
    if !quality_buffer.is_null() {
        free(quality_buffer as *mut libc::c_void);
        quality_buffer = 0 as *mut libc::c_char
    }
    if !success && created_output_file as libc::c_int != 0 {
        if remove(out_file_path) != 0 as libc::c_int {
            err(b"can\'t remove incomplete output file \"%s\"\n\x00" as
                    *const u8 as *const libc::c_char, out_file_path);
        }
    }
    if !out_file_path_auto.is_null() {
        free(out_file_path_auto as *mut libc::c_void);
        out_file_path_auto = 0 as *mut libc::c_char
    };
}
unsafe extern "C" fn set_out_type(mut new_type: OUTPUT_TYPE) {
    if out_type as libc::c_uint != UNDECIDED as libc::c_int as libc::c_uint {
        die(b"only one output type should be specified\n\x00" as *const u8 as
                *const libc::c_char);
    }
    out_type = new_type;
}
unsafe extern "C" fn set_input_file_path(mut new_path: *mut libc::c_char) {
    if !in_file_path.is_null() {
        die(b"can process only one file at a time\n\x00" as *const u8 as
                *const libc::c_char);
    }
    if *new_path as libc::c_int == '\u{0}' as i32 {
        die(b"empty input path specified\n\x00" as *const u8 as
                *const libc::c_char);
    }
    in_file_path = new_path;
}
unsafe extern "C" fn set_output_file_path(mut new_path: *mut libc::c_char) {
    if !out_file_path.is_null() {
        die(b"double --out parameter\n\x00" as *const u8 as
                *const libc::c_char);
    }
    if *new_path as libc::c_int == '\u{0}' as i32 {
        die(b"empty --out parameter\n\x00" as *const u8 as
                *const libc::c_char);
    }
    out_file_path = new_path;
}
unsafe extern "C" fn set_line_length(mut str: *mut libc::c_char) {
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut a: libc::c_longlong = strtoll(str, &mut end, 10 as libc::c_int);
    if *end as libc::c_int != '\u{0}' as i32 {
        die(b"can\'t parse the value of --line-length parameter\n\x00" as
                *const u8 as *const libc::c_char);
    }
    if a < 0 as libc::c_longlong {
        die(b"negative line length specified\n\x00" as *const u8 as
                *const libc::c_char);
    }
    let mut test_str: [libc::c_char; 21] = [0; 21];
    let mut nc: libc::c_int =
        snprintf(test_str.as_mut_ptr(), 21 as libc::c_int as libc::c_ulong,
                 b"%lld\x00" as *const u8 as *const libc::c_char, a);
    if nc < 1 as libc::c_int || nc > 20 as libc::c_int ||
           strcmp(test_str.as_mut_ptr(), str) != 0 as libc::c_int {
        die(b"can\'t parse the value of --line-length parameter\n\x00" as
                *const u8 as *const libc::c_char);
    }
    requested_line_length = a as libc::c_ulonglong;
    line_length_is_specified = 1 as libc::c_int != 0;
}
unsafe extern "C" fn show_version() {
    msg(b"unnaf - NAF decompressor, version 1.2.0, 2020-09-01\nCopyright (c) 2018-2020 Kirill Kryukov\n\x00"
            as *const u8 as *const libc::c_char);
    if verbose {
        msg(b"Built with zstd 1.4.5, using runtime zstd %s\n\x00" as *const u8
                as *const libc::c_char, ZSTD_versionString());
    };
}
unsafe extern "C" fn show_help() {
    msg(b"Usage: unnaf [OUTPUT-TYPE] [file.naf]\nOptions for selecting output type:\n  --format        - File format version\n  --part-list     - List of parts\n  --sizes         - Part sizes\n  --number        - Number of sequences\n  --title         - Dataset title\n  --ids           - Sequence ids (accession numbers)\n  --names         - Full sequence names (including ids)\n  --lengths       - Sequence lengths\n  --total-length  - Sum of sequence lengths\n  --mask          - Masked region lengths\n  --4bit          - 4bit-encoded nucleotide sequence (binary data)\n  --seq           - Continuous concatenated sequence\n  --sequences     - One sequence per line, no names\n  --fasta         - FASTA-formatted sequences\n  --fastq         - FASTQ-formatted sequences\nOther options:\n  -o FILE         - Decompress into FILE\n  -c              - Write to standard output\n  --line-length N - Use lines of width N for FASTA output\n  --no-mask       - Ignore mask\n  --binary        - Binary output (no 0D 0A on Windows)\n  -h, --help      - Show help\n  -V, --version   - Show version\n\x00"
            as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn parse_command_line(mut argc: libc::c_int,
                                        mut argv: *mut *mut libc::c_char) {
    let mut print_version: bool = 0 as libc::c_int != 0;
    let mut current_block_45: u64;
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < argc {
        if *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize) as
               libc::c_int == '-' as i32 {
            if *(*argv.offset(i as isize)).offset(1 as libc::c_int as isize)
                   as libc::c_int == '-' as i32 {
                if i < argc - 1 as libc::c_int {
                    if strcmp(*argv.offset(i as isize),
                              b"--line-length\x00" as *const u8 as
                                  *const libc::c_char) == 0 {
                        i += 1;
                        set_line_length(*argv.offset(i as isize));
                        current_block_45 = 16668937799742929182;
                    } else { current_block_45 = 13513818773234778473; }
                } else { current_block_45 = 13513818773234778473; }
                match current_block_45 {
                    16668937799742929182 => { }
                    _ => {
                        if strcmp(*argv.offset(i as isize),
                                  b"--format\x00" as *const u8 as
                                      *const libc::c_char) == 0 {
                            set_out_type(FORMAT_NAME);
                            current_block_45 = 16668937799742929182;
                        } else if strcmp(*argv.offset(i as isize),
                                         b"--part-list\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            set_out_type(PART_LIST);
                            current_block_45 = 16668937799742929182;
                        } else if strcmp(*argv.offset(i as isize),
                                         b"--sizes\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            set_out_type(PART_SIZES);
                            current_block_45 = 16668937799742929182;
                        } else if strcmp(*argv.offset(i as isize),
                                         b"--number\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            set_out_type(NUMBER_OF_SEQUENCES);
                            current_block_45 = 16668937799742929182;
                        } else if strcmp(*argv.offset(i as isize),
                                         b"--title\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            set_out_type(TITLE);
                            current_block_45 = 16668937799742929182;
                        } else if strcmp(*argv.offset(i as isize),
                                         b"--ids\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            set_out_type(IDS);
                            current_block_45 = 16668937799742929182;
                        } else if strcmp(*argv.offset(i as isize),
                                         b"--names\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            set_out_type(NAMES);
                            current_block_45 = 16668937799742929182;
                        } else if strcmp(*argv.offset(i as isize),
                                         b"--lengths\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            set_out_type(LENGTHS);
                            current_block_45 = 16668937799742929182;
                        } else if strcmp(*argv.offset(i as isize),
                                         b"--total-length\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            set_out_type(TOTAL_LENGTH);
                            current_block_45 = 16668937799742929182;
                        } else if strcmp(*argv.offset(i as isize),
                                         b"--mask\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            set_out_type(MASK);
                            current_block_45 = 16668937799742929182;
                        } else if strcmp(*argv.offset(i as isize),
                                         b"--total-mask-length\x00" as
                                             *const u8 as *const libc::c_char)
                                      == 0 {
                            set_out_type(TOTAL_MASK_LENGTH);
                            current_block_45 = 16668937799742929182;
                        } else if strcmp(*argv.offset(i as isize),
                                         b"--4bit\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            set_out_type(FOUR_BIT);
                            current_block_45 = 16668937799742929182;
                        } else if strcmp(*argv.offset(i as isize),
                                         b"--seq\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            set_out_type(SEQ);
                            current_block_45 = 16668937799742929182;
                        } else if strcmp(*argv.offset(i as isize),
                                         b"--sequences\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            set_out_type(SEQUENCES);
                            current_block_45 = 16668937799742929182;
                        } else if strcmp(*argv.offset(i as isize),
                                         b"--charcount\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            set_out_type(CHARCOUNT);
                            current_block_45 = 16668937799742929182;
                        } else if strcmp(*argv.offset(i as isize),
                                         b"--fasta\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            set_out_type(FASTA);
                            current_block_45 = 16668937799742929182;
                        } else if strcmp(*argv.offset(i as isize),
                                         b"--fastq\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            set_out_type(FASTQ);
                            current_block_45 = 16668937799742929182;
                        } else if strcmp(*argv.offset(i as isize),
                                         b"--no-mask\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            use_mask = 0 as libc::c_int != 0;
                            current_block_45 = 16668937799742929182;
                        } else if strcmp(*argv.offset(i as isize),
                                         b"--binary-stdout\x00" as *const u8
                                             as *const libc::c_char) == 0 {
                            binary_stdout = 1 as libc::c_int != 0;
                            current_block_45 = 16668937799742929182;
                        } else if strcmp(*argv.offset(i as isize),
                                         b"--binary-stderr\x00" as *const u8
                                             as *const libc::c_char) == 0 {
                            if !binary_stderr {
                                binary_stderr = 1 as libc::c_int != 0;
                                change_stderr_to_binary();
                            }
                            current_block_45 = 16668937799742929182;
                        } else {
                            if strcmp(*argv.offset(i as isize),
                                      b"--help\x00" as *const u8 as
                                          *const libc::c_char) == 0 {
                                show_help();
                                exit(0 as libc::c_int);
                            }
                            if strcmp(*argv.offset(i as isize),
                                      b"--verbose\x00" as *const u8 as
                                          *const libc::c_char) == 0 {
                                verbose = 1 as libc::c_int != 0;
                                current_block_45 = 16668937799742929182;
                            } else if strcmp(*argv.offset(i as isize),
                                             b"--version\x00" as *const u8 as
                                                 *const libc::c_char) == 0 {
                                print_version = 1 as libc::c_int != 0;
                                current_block_45 = 16668937799742929182;
                            } else if strcmp(*argv.offset(i as isize),
                                             b"--dna\x00" as *const u8 as
                                                 *const libc::c_char) == 0 {
                                set_out_type(DNA);
                                current_block_45 = 16668937799742929182;
                            } else if strcmp(*argv.offset(i as isize),
                                             b"--masked-dna\x00" as *const u8
                                                 as *const libc::c_char) == 0
                             {
                                set_out_type(MASKED_DNA);
                                current_block_45 = 16668937799742929182;
                            } else if strcmp(*argv.offset(i as isize),
                                             b"--unmasked-dna\x00" as
                                                 *const u8 as
                                                 *const libc::c_char) == 0 {
                                set_out_type(UNMASKED_DNA);
                                current_block_45 = 16668937799742929182;
                            } else if strcmp(*argv.offset(i as isize),
                                             b"--masked-fasta\x00" as
                                                 *const u8 as
                                                 *const libc::c_char) == 0 {
                                set_out_type(MASKED_FASTA);
                                current_block_45 = 16668937799742929182;
                            } else if strcmp(*argv.offset(i as isize),
                                             b"--unmasked-fasta\x00" as
                                                 *const u8 as
                                                 *const libc::c_char) == 0 {
                                set_out_type(UNMASKED_FASTA);
                                current_block_45 = 16668937799742929182;
                            } else {
                                current_block_45 = 15237655884915618618;
                            }
                        }
                    }
                }
                // Instead use "--fasta --no-mask"
            } else { current_block_45 = 15237655884915618618; }
            match current_block_45 {
                16668937799742929182 => { }
                _ => {
                    if i < argc - 1 as libc::c_int {
                        if strcmp(*argv.offset(i as isize),
                                  b"-o\x00" as *const u8 as
                                      *const libc::c_char) == 0 {
                            i += 1;
                            set_output_file_path(*argv.offset(i as isize));
                            current_block_45 = 16668937799742929182;
                        } else { current_block_45 = 496303045384785551; }
                    } else { current_block_45 = 496303045384785551; }
                    match current_block_45 {
                        16668937799742929182 => { }
                        _ => {
                            if strcmp(*argv.offset(i as isize),
                                      b"-c\x00" as *const u8 as
                                          *const libc::c_char) == 0 {
                                force_stdout = 1 as libc::c_int != 0
                            } else {
                                if strcmp(*argv.offset(i as isize),
                                          b"-h\x00" as *const u8 as
                                              *const libc::c_char) == 0 {
                                    show_help();
                                    exit(0 as libc::c_int);
                                }
                                if strcmp(*argv.offset(i as isize),
                                          b"-V\x00" as *const u8 as
                                              *const libc::c_char) == 0 {
                                    print_version = 1 as libc::c_int != 0
                                } else {
                                    die(b"unknown or incomplete argument \"%s\"\n\x00"
                                            as *const u8 as
                                            *const libc::c_char,
                                        *argv.offset(i as isize));
                                }
                            }
                        }
                    }
                }
            }
        } else { set_input_file_path(*argv.offset(i as isize)); }
        i += 1
    }
    if print_version { show_version(); exit(0 as libc::c_int); }
    if force_stdout as libc::c_int != 0 && !out_file_path.is_null() {
        die(b"-c and -o arguments can\'t be used together\n\x00" as *const u8
                as *const libc::c_char);
    };
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    atexit(Some(done as unsafe extern "C" fn() -> ()));
    parse_command_line(argc, argv);
    if in_file_path.is_null() && isatty(fileno(stdin)) != 0 {
        err(b"no input specified, use \"unnaf -h\" for help\n\x00" as
                *const u8 as *const libc::c_char);
        exit(0 as libc::c_int);
    }
    open_input_file();
    read_header();
    if in_seq_type == seq_type_rna as libc::c_int {
        code_to_nuc[1 as libc::c_int as usize] = 'U' as i32 as libc::c_uchar
    }
    if in_seq_type <= seq_type_rna as libc::c_int { init_tables(); }
    if out_type as libc::c_uint == UNDECIDED as libc::c_int as libc::c_uint {
        out_type =
            if has_quality != 0 {
                FASTQ as libc::c_int
            } else { FASTA as libc::c_int } as OUTPUT_TYPE
    }
    if (out_type as libc::c_uint == DNA as libc::c_int as libc::c_uint ||
            out_type as libc::c_uint ==
                MASKED_DNA as libc::c_int as libc::c_uint ||
            out_type as libc::c_uint ==
                UNMASKED_DNA as libc::c_int as libc::c_uint) &&
           in_seq_type != seq_type_dna as libc::c_int {
        die(b"input has not DNA, but %s data\n\x00" as *const u8 as
                *const libc::c_char, in_seq_type_name);
    }
    if out_type as libc::c_uint == FOUR_BIT as libc::c_int as libc::c_uint &&
           in_seq_type >= seq_type_protein as libc::c_int {
        die(b"input has no 4-bit encoded data, but %s sequences\n\x00" as
                *const u8 as *const libc::c_char, in_seq_type_name);
    }
    open_output_file();
    if !in_file_path.is_null() && !out_file_path.is_null() {
        if fstat(fileno(IN), &mut input_stat) == 0 as libc::c_int {
            have_input_stat = 1 as libc::c_int != 0
        } else {
            err(b"can\'t obtain status of input file\n\x00" as *const u8 as
                    *const libc::c_char);
        }
    }
    if out_type as libc::c_uint == FORMAT_NAME as libc::c_int as libc::c_uint
       {
        fprintf(OUT,
                b"%s sequences%s in NAF format version %d\n\x00" as *const u8
                    as *const libc::c_char, in_seq_type_name,
                if has_quality != 0 {
                    b" with qualities\x00" as *const u8 as *const libc::c_char
                } else { b"\x00" as *const u8 as *const libc::c_char },
                format_version as libc::c_int);
    } else if out_type as libc::c_uint ==
                  PART_LIST as libc::c_int as libc::c_uint {
        print_list_of_parts();
    } else {
        max_line_length = read_number(IN);
        if line_length_is_specified {
            max_line_length = requested_line_length
        }
        N = read_number(IN);
        if out_type as libc::c_uint ==
               NUMBER_OF_SEQUENCES as libc::c_int as libc::c_uint {
            fprintf(OUT, b"%llu\n\x00" as *const u8 as *const libc::c_char,
                    N);
        } else if out_type as libc::c_uint ==
                      PART_SIZES as libc::c_int as libc::c_uint {
            print_part_sizes();
        } else if out_type as libc::c_uint ==
                      TITLE as libc::c_int as libc::c_uint {
            print_title();
        } else if N != 0 as libc::c_int as libc::c_ulonglong {
            skip_title();
            if out_type as libc::c_uint == IDS as libc::c_int as libc::c_uint
               {
                print_ids();
            } else if out_type as libc::c_uint ==
                          NAMES as libc::c_int as libc::c_uint {
                print_names();
            } else if out_type as libc::c_uint ==
                          LENGTHS as libc::c_int as libc::c_uint {
                print_lengths();
            } else if out_type as libc::c_uint ==
                          TOTAL_LENGTH as libc::c_int as libc::c_uint {
                print_total_length();
            } else if out_type as libc::c_uint ==
                          MASK as libc::c_int as libc::c_uint {
                print_mask();
            } else if out_type as libc::c_uint ==
                          TOTAL_MASK_LENGTH as libc::c_int as libc::c_uint {
                print_total_mask_length();
            } else if out_type as libc::c_uint ==
                          FOUR_BIT as libc::c_int as libc::c_uint {
                print_4bit();
            } else {
                dna_buffer_flush_size =
                    ZSTD_DStreamOutSize().wrapping_mul(2 as libc::c_int as
                                                           libc::c_ulong);
                dna_buffer_size =
                    dna_buffer_flush_size.wrapping_mul(2 as libc::c_int as
                                                           libc::c_ulong).wrapping_add(10
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           libc::c_ulong);
                dna_buffer =
                    malloc_or_die(dna_buffer_size) as *mut libc::c_uchar;
                out_print_buffer_size =
                    dna_buffer_size.wrapping_mul(2 as libc::c_int as
                                                     libc::c_ulong);
                out_print_buffer =
                    malloc_or_die(out_print_buffer_size) as
                        *mut libc::c_uchar;
                if out_type as libc::c_uint ==
                       DNA as libc::c_int as libc::c_uint {
                    print_dna((use_mask as libc::c_int != 0 && has_mask != 0)
                                  as libc::c_int);
                } else if out_type as libc::c_uint ==
                              SEQ as libc::c_int as libc::c_uint {
                    print_dna((use_mask as libc::c_int != 0 && has_mask != 0)
                                  as libc::c_int);
                } else if out_type as libc::c_uint ==
                              MASKED_DNA as libc::c_int as libc::c_uint {
                    print_dna((use_mask as libc::c_int != 0 && has_mask != 0)
                                  as libc::c_int);
                } else if out_type as libc::c_uint ==
                              UNMASKED_DNA as libc::c_int as libc::c_uint {
                    print_dna(0 as libc::c_int);
                } else if out_type as libc::c_uint ==
                              CHARCOUNT as libc::c_int as libc::c_uint {
                    print_charcount((use_mask as libc::c_int != 0 &&
                                         has_mask != 0) as libc::c_int);
                } else if out_type as libc::c_uint ==
                              SEQUENCES as libc::c_int as libc::c_uint {
                    print_sequences((use_mask as libc::c_int != 0 &&
                                         has_mask != 0) as libc::c_int);
                } else if out_type as libc::c_uint ==
                              FASTA as libc::c_int as libc::c_uint {
                    print_fasta((use_mask as libc::c_int != 0 &&
                                     has_mask != 0) as libc::c_int);
                } else if out_type as libc::c_uint ==
                              MASKED_FASTA as libc::c_int as libc::c_uint {
                    print_fasta((use_mask as libc::c_int != 0 &&
                                     has_mask != 0) as libc::c_int);
                } else if out_type as libc::c_uint ==
                              UNMASKED_FASTA as libc::c_int as libc::c_uint {
                    print_fasta(0 as libc::c_int);
                } else if out_type as libc::c_uint ==
                              FASTQ as libc::c_int as libc::c_uint {
                    if N > 0 as libc::c_int as libc::c_ulonglong {
                        if has_quality == 0 {
                            die(b"FASTQ output requested, but input has no qualities\n\x00"
                                    as *const u8 as *const libc::c_char);
                        }
                        print_fastq(0 as libc::c_int);
                    }
                } else {
                    die(b"unknown output requested\n\x00" as *const u8 as
                            *const libc::c_char);
                }
            }
        }
    }
    close_input_file();
    if !out_file_path.is_null() && have_input_stat as libc::c_int != 0 {
        close_output_file_and_set_stat();
    } else { close_output_file(); }
    success = 1 as libc::c_int != 0;
    return 0 as libc::c_int;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
// Deprecated undocumented options.
// Instead use "--seq"
// Instead use "--seq"
// Instead use "--seq --no-mask"
// Instead use "--fasta"
