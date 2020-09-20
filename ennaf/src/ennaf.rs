#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, const_raw_ptr_to_usize_cast, extern_types, main,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type ZSTD_CCtx_s;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
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
    fn strtol(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char,
              __base: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn strtoll(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char,
               __base: libc::c_int) -> libc::c_longlong;
    #[no_mangle]
    fn rand() -> libc::c_int;
    #[no_mangle]
    fn srand(__seed: libc::c_uint);
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn fchown(__fd: libc::c_int, __owner: __uid_t, __group: __gid_t)
     -> libc::c_int;
    #[no_mangle]
    fn getpid() -> __pid_t;
    #[no_mangle]
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn __fxstat(__ver: libc::c_int, __fildes: libc::c_int,
                __stat_buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn fchmod(__fd: libc::c_int, __mode: __mode_t) -> libc::c_int;
    #[no_mangle]
    fn futimens(__fd: libc::c_int, __times: *const timespec) -> libc::c_int;
    #[no_mangle]
    fn ZSTD_versionString() -> *const libc::c_char;
    #[no_mangle]
    fn ZSTD_isError(code: size_t) -> libc::c_uint;
    #[no_mangle]
    fn ZSTD_getErrorName(code: size_t) -> *const libc::c_char;
    #[no_mangle]
    fn ZSTD_minCLevel() -> libc::c_int;
    #[no_mangle]
    fn ZSTD_maxCLevel() -> libc::c_int;
    #[no_mangle]
    fn ZSTD_createCStream() -> *mut ZSTD_CStream;
    #[no_mangle]
    fn ZSTD_CStreamOutSize() -> size_t;
    #[no_mangle]
    fn ZSTD_initCStream(zcs: *mut ZSTD_CStream, compressionLevel: libc::c_int)
     -> size_t;
    #[no_mangle]
    fn ZSTD_compressStream(zcs: *mut ZSTD_CStream,
                           output: *mut ZSTD_outBuffer,
                           input: *mut ZSTD_inBuffer) -> size_t;
    #[no_mangle]
    fn ZSTD_endStream(zcs: *mut ZSTD_CStream, output: *mut ZSTD_outBuffer)
     -> size_t;
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
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub type ZSTD_CCtx = ZSTD_CCtx_s;
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
pub type ZSTD_CStream = ZSTD_CCtx;
pub type ZSTD_allocFunction
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: size_t)
               -> *mut libc::c_void>;
pub type ZSTD_freeFunction
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void)
               -> ()>;
pub type C2RustUnnamed = libc::c_uint;
pub const in_format_fastq: C2RustUnnamed = 2;
pub const in_format_fasta: C2RustUnnamed = 1;
pub const in_format_unknown: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const seq_type_text: C2RustUnnamed_0 = 3;
pub const seq_type_protein: C2RustUnnamed_0 = 2;
pub const seq_type_rna: C2RustUnnamed_0 = 1;
pub const seq_type_dna: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct compressor_t {
    pub allocated: size_t,
    pub fill: size_t,
    pub uncompressed_size: libc::c_ulonglong,
    pub compressed_size: libc::c_ulonglong,
    pub written: libc::c_ulonglong,
    pub cstream: *mut ZSTD_CStream,
    pub file: *mut FILE,
    pub path: *mut libc::c_char,
    pub buf: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct string_t {
    pub length: size_t,
    pub data: *mut libc::c_uchar,
    pub writer: Option<unsafe extern "C" fn(_: *mut libc::c_uchar, _: size_t)
                           -> ()>,
}
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
 * NAF compressor
 * Copyright (c) 2018-2020 Kirill Kryukov
 * See README.md and LICENSE files of this repository
 */
/*
 * Replacement for unknown input characters.
 */
static mut unexpected_seq_char_replacement: libc::c_uchar =
    'N' as i32 as libc::c_uchar;
static mut unexpected_name_char_replacement: libc::c_uchar =
    '?' as i32 as libc::c_uchar;
static mut unexpected_qual_char_replacement: libc::c_uchar =
    '!' as i32 as libc::c_uchar;
// Unknown character can only mean poor quality.
/*
 * Character class tables have 257 entries to make space for an EOF mark.
 */
/*
 * End-of-line ASCII characters:
 *   '\x0A' - LF - line feed
 *   '\x0B' - VT - vertical tab
 *   '\x0C' - FF - form feed
 *   '\x0D' - CR - carriage return
 */
static mut is_eol_arr: [bool; 257] =
    [0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0];
/*
 * Space ASCII characters:
 *   '\x09' - TAB - horizontal tab
 *   '\x0A' - LF - line feed
 *   '\x0B' - VT - vertical tab
 *   '\x0C' - FF - form feed
 *   '\x0D' - CR - carriage return
 *   '\x20' - space
 */
static mut is_space_arr: [bool; 257] =
    [0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 1 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0];
/*
 * Well-formed input can have "spaces" of two kinds only:
 *   '\x0A' - LF - line feed
 *   '\x20' - space
 */
static mut is_well_formed_space_arr: [bool; 257] =
    [0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 1 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 1 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0];
static mut is_unexpected_dna_arr: [bool; 257] =
    [1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     0 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     0 as libc::c_int != 0, 1 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 1 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 1 as libc::c_int != 0, 0 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 0 as libc::c_int != 0,
     1 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     1 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     1 as libc::c_int != 0, 0 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0];
static mut is_unexpected_rna_arr: [bool; 257] =
    [1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     0 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     0 as libc::c_int != 0, 1 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     1 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 1 as libc::c_int != 0, 0 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 0 as libc::c_int != 0,
     1 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 1 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     1 as libc::c_int != 0, 0 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0];
/*
 * Supported amino-acid codes:
 *   'ARNDCQEGHILKMFPSTWYV' - standard 20 amino acids
 *   'U' - Selenocysteine
 *   'O' - Pyrrolysine
 *   'J' - Leucine or Isoleucine ('L' or 'I')
 *   'B' - Aspartic acid or Asparagine ('D' or 'N')
 *   'Z' - Glutamic acid or Glutamine ('E' or 'Q')
 *   'X' - Any amino acid
 *   '*' - Stop codon
 *   '-' - Gap
 */
static mut is_unexpected_protein_arr: [bool; 257] =
    [1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     0 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     0 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0];
// Text sequence consists of printable non-space characters.
static mut is_unexpected_text_arr: [bool; 257] =
    [1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 1 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0];
// Comment consists of printable characters.
static mut is_unexpected_comment_arr: [bool; 257] =
    [1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 1 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0];
// Quality can include characters with codes from 33 to 126.
static mut is_unexpected_qual_arr: [bool; 257] =
    [1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 0 as libc::c_int != 0, 0 as libc::c_int != 0,
     0 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0, 1 as libc::c_int != 0,
     1 as libc::c_int != 0, 1 as libc::c_int != 0];
static mut n_unexpected_id_characters: [libc::c_ulonglong; 257] =
    [0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong];
static mut n_unexpected_comment_characters: [libc::c_ulonglong; 257] =
    [0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong];
static mut n_unexpected_seq_characters: [libc::c_ulonglong; 257] =
    [0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong];
static mut n_unexpected_qual_characters: [libc::c_ulonglong; 257] =
    [0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong,
     0 as libc::c_int as libc::c_ulonglong];
static mut nuc_code: [libc::c_uchar; 257] =
    [15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 8 as libc::c_int as libc::c_uchar,
     7 as libc::c_int as libc::c_uchar, 4 as libc::c_int as libc::c_uchar,
     11 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 2 as libc::c_int as libc::c_uchar,
     13 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 3 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 12 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     10 as libc::c_int as libc::c_uchar, 6 as libc::c_int as libc::c_uchar,
     1 as libc::c_int as libc::c_uchar, 1 as libc::c_int as libc::c_uchar,
     14 as libc::c_int as libc::c_uchar, 9 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 5 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 8 as libc::c_int as libc::c_uchar,
     7 as libc::c_int as libc::c_uchar, 4 as libc::c_int as libc::c_uchar,
     11 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 2 as libc::c_int as libc::c_uchar,
     13 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 3 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 12 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     10 as libc::c_int as libc::c_uchar, 6 as libc::c_int as libc::c_uchar,
     1 as libc::c_int as libc::c_uchar, 1 as libc::c_int as libc::c_uchar,
     14 as libc::c_int as libc::c_uchar, 9 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 5 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 15 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar];
static mut naf_magic_number: [libc::c_uchar; 3] =
    [0x1 as libc::c_uint as libc::c_uchar,
     0xf9 as libc::c_uint as libc::c_uchar,
     0xec as libc::c_uint as libc::c_uchar];
static mut verbose: bool = 0 as libc::c_int != 0;
static mut binary_stderr: bool = 0 as libc::c_int != 0;
static mut keep_temp_files: bool = 0 as libc::c_int != 0;
static mut no_mask: bool = 0 as libc::c_int != 0;
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
static mut created_output_file: bool = 0 as libc::c_int != 0;
static mut compression_level: libc::c_int = 1 as libc::c_int;
static mut temp_dir: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut dataset_name: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut dataset_title: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut temp_prefix_length: size_t = 0 as libc::c_int as size_t;
static mut temp_path_length: size_t = 0 as libc::c_int as size_t;
static mut temp_prefix: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut in_format_from_command_line: libc::c_int =
    in_format_unknown as libc::c_int;
static mut in_format_from_input: libc::c_int =
    in_format_unknown as libc::c_int;
static mut in_format_from_extension: libc::c_int =
    in_format_unknown as libc::c_int;
static mut in_seq_type: libc::c_int = seq_type_dna as libc::c_int;
static mut in_seq_type_name: *const libc::c_char =
    b"DNA\x00" as *const u8 as *const libc::c_char;
static mut store_title: bool = 0 as libc::c_int != 0;
static mut store_mask: bool = 1 as libc::c_int != 0;
static mut store_qual: bool = 0 as libc::c_int != 0;
static mut parity: bool = 0 as libc::c_int != 0;
static mut out_4bit_buffer: *mut libc::c_uchar =
    0 as *const libc::c_uchar as *mut libc::c_uchar;
static mut out_4bit_pos: *mut libc::c_uchar =
    0 as *const libc::c_uchar as *mut libc::c_uchar;
static mut seq_size_original: libc::c_ulonglong = 0 as libc::c_ulonglong;
static mut longest_line_length: libc::c_ulonglong = 0 as libc::c_ulonglong;
static mut line_length_is_specified: bool = 0 as libc::c_int != 0;
static mut requested_line_length: libc::c_ulonglong = 0 as libc::c_ulonglong;
static mut file_copy_buffer: *mut libc::c_uchar =
    0 as *const libc::c_uchar as *mut libc::c_uchar;
static mut length_units: *mut libc::c_uint =
    0 as *const libc::c_uint as *mut libc::c_uint;
static mut length_unit_index: libc::c_uint = 0 as libc::c_int as libc::c_uint;
static mut mask_units: *mut libc::c_uchar =
    0 as *const libc::c_uchar as *mut libc::c_uchar;
static mut mask_units_end: *mut libc::c_uchar =
    0 as *const libc::c_uchar as *mut libc::c_uchar;
static mut mask_units_pos: *mut libc::c_uchar =
    0 as *const libc::c_uchar as *mut libc::c_uchar;
static mut mask_len: libc::c_ulonglong =
    0 as libc::c_int as libc::c_ulonglong;
static mut mask_on: bool = 0 as libc::c_int != 0;
static mut in_buffer: *mut libc::c_uchar =
    0 as *const libc::c_uchar as *mut libc::c_uchar;
static mut in_begin: size_t = 0 as libc::c_int as size_t;
static mut in_end: size_t = 0 as libc::c_int as size_t;
static mut n_sequences: libc::c_ulonglong = 0 as libc::c_ulonglong;
static mut is_unexpected_arr: *mut bool =
    unsafe { is_unexpected_dna_arr.as_ptr() as *mut _ };
static mut abort_on_unexpected_code: bool = 0 as libc::c_int != 0;
static mut assume_well_formed_input: bool = 0 as libc::c_int != 0;
static mut out_4bit_buffer_size: size_t = 0 as libc::c_int as size_t;
static mut zstd_stream_recommended_out_buffer_size: size_t =
    0 as libc::c_int as size_t;
#[no_mangle]
pub static mut IDS: compressor_t =
    {
        let mut init =
            compressor_t{allocated: 0 as libc::c_int as size_t,
                         fill: 0 as libc::c_int as size_t,
                         uncompressed_size:
                             0 as libc::c_int as libc::c_ulonglong,
                         compressed_size:
                             0 as libc::c_int as libc::c_ulonglong,
                         written: 0 as libc::c_int as libc::c_ulonglong,
                         cstream:
                             0 as *const ZSTD_CStream as *mut ZSTD_CStream,
                         file: 0 as *const FILE as *mut FILE,
                         path: 0 as *const libc::c_char as *mut libc::c_char,
                         buf:
                             0 as *const libc::c_uchar as
                                 *mut libc::c_uchar,};
        init
    };
#[no_mangle]
pub static mut COMM: compressor_t =
    {
        let mut init =
            compressor_t{allocated: 0 as libc::c_int as size_t,
                         fill: 0 as libc::c_int as size_t,
                         uncompressed_size:
                             0 as libc::c_int as libc::c_ulonglong,
                         compressed_size:
                             0 as libc::c_int as libc::c_ulonglong,
                         written: 0 as libc::c_int as libc::c_ulonglong,
                         cstream:
                             0 as *const ZSTD_CStream as *mut ZSTD_CStream,
                         file: 0 as *const FILE as *mut FILE,
                         path: 0 as *const libc::c_char as *mut libc::c_char,
                         buf:
                             0 as *const libc::c_uchar as
                                 *mut libc::c_uchar,};
        init
    };
#[no_mangle]
pub static mut LEN: compressor_t =
    {
        let mut init =
            compressor_t{allocated: 0 as libc::c_int as size_t,
                         fill: 0 as libc::c_int as size_t,
                         uncompressed_size:
                             0 as libc::c_int as libc::c_ulonglong,
                         compressed_size:
                             0 as libc::c_int as libc::c_ulonglong,
                         written: 0 as libc::c_int as libc::c_ulonglong,
                         cstream:
                             0 as *const ZSTD_CStream as *mut ZSTD_CStream,
                         file: 0 as *const FILE as *mut FILE,
                         path: 0 as *const libc::c_char as *mut libc::c_char,
                         buf:
                             0 as *const libc::c_uchar as
                                 *mut libc::c_uchar,};
        init
    };
#[no_mangle]
pub static mut MASK: compressor_t =
    {
        let mut init =
            compressor_t{allocated: 0 as libc::c_int as size_t,
                         fill: 0 as libc::c_int as size_t,
                         uncompressed_size:
                             0 as libc::c_int as libc::c_ulonglong,
                         compressed_size:
                             0 as libc::c_int as libc::c_ulonglong,
                         written: 0 as libc::c_int as libc::c_ulonglong,
                         cstream:
                             0 as *const ZSTD_CStream as *mut ZSTD_CStream,
                         file: 0 as *const FILE as *mut FILE,
                         path: 0 as *const libc::c_char as *mut libc::c_char,
                         buf:
                             0 as *const libc::c_uchar as
                                 *mut libc::c_uchar,};
        init
    };
#[no_mangle]
pub static mut SEQ: compressor_t =
    {
        let mut init =
            compressor_t{allocated: 0 as libc::c_int as size_t,
                         fill: 0 as libc::c_int as size_t,
                         uncompressed_size:
                             0 as libc::c_int as libc::c_ulonglong,
                         compressed_size:
                             0 as libc::c_int as libc::c_ulonglong,
                         written: 0 as libc::c_int as libc::c_ulonglong,
                         cstream:
                             0 as *const ZSTD_CStream as *mut ZSTD_CStream,
                         file: 0 as *const FILE as *mut FILE,
                         path: 0 as *const libc::c_char as *mut libc::c_char,
                         buf:
                             0 as *const libc::c_uchar as
                                 *mut libc::c_uchar,};
        init
    };
#[no_mangle]
pub static mut QUAL: compressor_t =
    {
        let mut init =
            compressor_t{allocated: 0 as libc::c_int as size_t,
                         fill: 0 as libc::c_int as size_t,
                         uncompressed_size:
                             0 as libc::c_int as libc::c_ulonglong,
                         compressed_size:
                             0 as libc::c_int as libc::c_ulonglong,
                         written: 0 as libc::c_int as libc::c_ulonglong,
                         cstream:
                             0 as *const ZSTD_CStream as *mut ZSTD_CStream,
                         file: 0 as *const FILE as *mut FILE,
                         path: 0 as *const libc::c_char as *mut libc::c_char,
                         buf:
                             0 as *const libc::c_uchar as
                                 *mut libc::c_uchar,};
        init
    };
static mut success: bool = 0 as libc::c_int != 0;
/*
 * NAF compressor
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
unsafe extern "C" fn warn(mut format: *const libc::c_char, mut args: ...) {
    fputs(b"ennaf warning: \x00" as *const u8 as *const libc::c_char, stderr);
    let mut argptr: ::std::ffi::VaListImpl;
    argptr = args.clone();
    vfprintf(stderr, format, argptr.as_va_list());
}
#[cold]
unsafe extern "C" fn err(mut format: *const libc::c_char, mut args: ...) {
    fputs(b"ennaf error: \x00" as *const u8 as *const libc::c_char, stderr);
    let mut argptr: ::std::ffi::VaListImpl;
    argptr = args.clone();
    vfprintf(stderr, format, argptr.as_va_list());
}
#[cold]
unsafe extern "C" fn die(mut format: *const libc::c_char, mut args: ...)
 -> ! {
    fputs(b"ennaf error: \x00" as *const u8 as *const libc::c_char, stderr);
    let mut argptr: ::std::ffi::VaListImpl;
    argptr = args.clone();
    vfprintf(stderr, format, argptr.as_va_list());
    exit(1 as libc::c_int);
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
unsafe extern "C" fn string_has_characters_unsafe_in_file_names(mut str:
                                                                    *mut libc::c_char)
 -> bool {
    let mut c: *mut libc::c_char = str;
    while *c != 0 {
        if (*c as libc::c_int) < 32 as libc::c_int ||
               *c as libc::c_int == '\\' as i32 ||
               *c as libc::c_int == '/' as i32 ||
               *c as libc::c_int == ':' as i32 ||
               *c as libc::c_int == '*' as i32 ||
               *c as libc::c_int == '?' as i32 ||
               *c as libc::c_int == '\"' as i32 ||
               *c as libc::c_int == '<' as i32 ||
               *c as libc::c_int == '>' as i32 ||
               *c as libc::c_int == '|' as i32 {
            return 1 as libc::c_int != 0
        }
        c = c.offset(1)
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn fread_or_die(mut ptr: *mut libc::c_void,
                                  mut element_size: size_t,
                                  mut n_elements: size_t, mut F: *mut FILE) {
    let mut elements_read: size_t = fread(ptr, element_size, n_elements, F);
    if elements_read != n_elements {
        die(b"can\'t read from file\n\x00" as *const u8 as
                *const libc::c_char);
    };
}
unsafe extern "C" fn fwrite_or_die(mut ptr: *const libc::c_void,
                                   mut element_size: size_t,
                                   mut n_elements: size_t, mut F: *mut FILE) {
    let mut elements_written: size_t =
        fwrite(ptr, element_size, n_elements, F);
    if elements_written != n_elements {
        die(b"can\'t write to file - disk full?\n\x00" as *const u8 as
                *const libc::c_char);
    };
}
unsafe extern "C" fn fputc_or_die(mut c: libc::c_int, mut F: *mut FILE) {
    if fputc(c, F) != c {
        die(b"can\'t write to file - disk full?\n\x00" as *const u8 as
                *const libc::c_char);
    };
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
 * NAF compressor
 * Copyright (c) 2018-2020 Kirill Kryukov
 * See README.md and LICENSE files of this repository
 */
unsafe extern "C" fn open_input_file() {
    if in_file_path.is_null() {
        if freopen(0 as *const libc::c_char,
                   b"rb\x00" as *const u8 as *const libc::c_char,
                   stdin).is_null() {
            die(b"can\'t read input in binary mode\n\x00" as *const u8 as
                    *const libc::c_char); // Some C std libs define pid_t as 'int', some as 'long long'.
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
    }
    in_buffer =
        malloc_or_die(16384 as libc::c_int as size_t) as *mut libc::c_uchar;
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
    if !out_file_path.is_null() && !force_stdout {
        OUT =
            fopen(out_file_path,
                  b"wb\x00" as *const u8 as *const libc::c_char);
        if OUT.is_null() {
            die(b"can\'t create output file\n\x00" as *const u8 as
                    *const libc::c_char);
        }
        created_output_file = 1 as libc::c_int != 0
    } else {
        if freopen(0 as *const libc::c_char,
                   b"wb\x00" as *const u8 as *const libc::c_char,
                   stdout).is_null() {
            die(b"can\'t set output stream to binary mode\n\x00" as *const u8
                    as *const libc::c_char);
        }
        OUT = stdout
    };
}
unsafe extern "C" fn close_input_file() {
    if !IN.is_null() && IN != stdin { fclose(IN); IN = 0 as *mut FILE };
}
unsafe extern "C" fn make_temp_prefix() {
    if !dataset_name.is_null() {
        temp_prefix_length = strlen(dataset_name);
        temp_prefix =
            malloc_or_die(temp_prefix_length.wrapping_add(1 as libc::c_int as
                                                              libc::c_ulong))
                as *mut libc::c_char;
        strcpy(temp_prefix, dataset_name);
    } else if !in_file_path.is_null() {
        let mut in_file_name: *mut libc::c_char =
            in_file_path.offset(strlen(in_file_path) as isize);
        while in_file_name > in_file_path &&
                  *in_file_name.offset(-(1 as libc::c_int as isize)) as
                      libc::c_int != '/' as i32 &&
                  *in_file_name.offset(-(1 as libc::c_int as isize)) as
                      libc::c_int != '\\' as i32 {
            in_file_name = in_file_name.offset(-1)
        }
        if verbose {
            fprintf(stderr,
                    b"Input file name: %s\n\x00" as *const u8 as
                        *const libc::c_char, in_file_name);
        }
        temp_prefix_length = strlen(in_file_name);
        temp_prefix =
            malloc_or_die(temp_prefix_length.wrapping_add(1 as libc::c_int as
                                                              libc::c_ulong))
                as *mut libc::c_char;
        strcpy(temp_prefix, in_file_name);
    } else {
        let mut pid: libc::c_longlong = getpid() as libc::c_longlong;
        srand(time(0 as *mut time_t) as libc::c_uint);
        let mut r: libc::c_long =
            rand() as libc::c_long % 2147483648 as libc::c_long;
        temp_prefix =
            malloc_or_die(32 as libc::c_int as size_t) as *mut libc::c_char;
        snprintf(temp_prefix, 32 as libc::c_int as libc::c_ulong,
                 b"%lld-%ld\x00" as *const u8 as *const libc::c_char, pid, r);
        temp_prefix_length = strlen(temp_prefix)
    }
    if verbose {
        msg(b"Temp file prefix: \"%s\"\n\x00" as *const u8 as
                *const libc::c_char, temp_prefix);
    }
    temp_path_length =
        strlen(temp_dir).wrapping_add(temp_prefix_length).wrapping_add(11 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_ulong);
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
/*
 * NAF compressor
 * Copyright (c) 2018-2020 Kirill Kryukov
 * See README.md and LICENSE files of this repository
 */
unsafe extern "C" fn create_zstd_cstream(mut level: libc::c_int)
 -> *mut ZSTD_CStream {
    let mut s: *mut ZSTD_CStream = ZSTD_createCStream();
    if s.is_null() {
        die(b"ZSTD_createCStream() error\n\x00" as *const u8 as
                *const libc::c_char);
    }
    let initResult: size_t = ZSTD_initCStream(s, level);
    if ZSTD_isError(initResult) != 0 {
        die(b"ZSTD_initCStream() error: %s\n\x00" as *const u8 as
                *const libc::c_char, ZSTD_getErrorName(initResult));
    }
    return s;
}
unsafe extern "C" fn compressor_init(mut w: *mut compressor_t,
                                     mut name_0: *const libc::c_char) {
    (*w).allocated =
        (2 as
             libc::c_ulonglong).wrapping_mul(1000 as libc::c_int as
                                                 libc::c_ulonglong).wrapping_mul(1000
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulonglong)
            as size_t;
    (*w).buf = malloc_or_die((*w).allocated) as *mut libc::c_uchar;
    (*w).cstream = create_zstd_cstream(compression_level);
    (*w).path =
        malloc_or_die(temp_path_length.wrapping_add(1 as libc::c_int as
                                                        libc::c_ulong)) as
            *mut libc::c_char;
    snprintf((*w).path, temp_path_length,
             b"%s/%s.%s\x00" as *const u8 as *const libc::c_char, temp_dir,
             temp_prefix, name_0);
    if verbose {
        msg(b"Temp %s file: \"%s\"\n\x00" as *const u8 as *const libc::c_char,
            name_0, (*w).path);
    };
}
#[inline(always)]
unsafe extern "C" fn compressor_create_file(mut w: *mut compressor_t) {
    if (*w).file.is_null() {
        (*w).file =
            fopen((*w).path, b"wb+\x00" as *const u8 as *const libc::c_char);
        if (*w).file.is_null() {
            die(b"can\'t create temporary file \"%s\"\n\x00" as *const u8 as
                    *const libc::c_char, (*w).path);
        }
    };
}
unsafe extern "C" fn compressor_end_stream(mut w: *mut compressor_t) {
    if !(*w).cstream.is_null() {
        if (*w).fill.wrapping_add(zstd_stream_recommended_out_buffer_size) >
               (*w).allocated {
            compressor_create_file(w);
            fwrite_or_die((*w).buf as *const libc::c_void,
                          1 as libc::c_int as size_t, (*w).fill, (*w).file);
            (*w).written =
                (*w).written.wrapping_add((*w).fill as libc::c_ulonglong);
            (*w).fill = 0 as libc::c_int as size_t
        }
        let mut output: ZSTD_outBuffer =
            {
                let mut init =
                    ZSTD_outBuffer_s{dst:
                                         (*w).buf.offset((*w).fill as isize)
                                             as *mut libc::c_void,
                                     size:
                                         (*w).allocated.wrapping_sub((*w).fill),
                                     pos: 0 as libc::c_int as size_t,};
                init
            };
        let remainingToFlush: size_t =
            ZSTD_endStream((*w).cstream, &mut output);
        if remainingToFlush != 0 as libc::c_int as libc::c_ulong {
            die(b"can\'t end zstd stream\n\x00" as *const u8 as
                    *const libc::c_char);
        }
        (*w).fill =
            ((*w).fill as libc::c_ulong).wrapping_add(output.pos) as size_t as
                size_t;
        (*w).compressed_size =
            (*w).compressed_size.wrapping_add(output.pos as
                                                  libc::c_ulonglong);
        (*w).cstream = 0 as *mut ZSTD_CStream;
        if keep_temp_files {
            compressor_create_file(w);
            fwrite_or_die((*w).buf as *const libc::c_void,
                          1 as libc::c_int as size_t, (*w).fill, (*w).file);
            (*w).written =
                (*w).written.wrapping_add((*w).fill as libc::c_ulonglong);
            (*w).fill = 0 as libc::c_int as size_t
        }
    };
}
unsafe extern "C" fn compressor_done(mut w: *mut compressor_t) {
    if !(*w).buf.is_null() {
        free((*w).buf as *mut libc::c_void);
        (*w).buf = 0 as *mut libc::c_uchar
    }
    if !(*w).file.is_null() {
        fclose_or_die((*w).file);
        (*w).file = 0 as *mut FILE;
        if !keep_temp_files {
            if remove((*w).path) != 0 as libc::c_int {
                err(b"can\'t remove temporary file \"%s\"\n\x00" as *const u8
                        as *const libc::c_char, (*w).path);
            }
        }
    };
}
#[inline(always)]
unsafe extern "C" fn compress(mut w: *mut compressor_t,
                              mut data: *const libc::c_void,
                              mut size: size_t) {
    let mut input: ZSTD_inBuffer =
        {
            let mut init =
                ZSTD_inBuffer_s{src: data,
                                size: size,
                                pos: 0 as libc::c_int as size_t,};
            init
        };
    while input.pos < input.size {
        let mut output: ZSTD_outBuffer =
            {
                let mut init =
                    ZSTD_outBuffer_s{dst:
                                         (*w).buf.offset((*w).fill as isize)
                                             as *mut libc::c_void,
                                     size:
                                         (*w).allocated.wrapping_sub((*w).fill),
                                     pos: 0 as libc::c_int as size_t,};
                init
            };
        let mut toRead: size_t =
            ZSTD_compressStream((*w).cstream, &mut output, &mut input);
        if ZSTD_isError(toRead) != 0 {
            die(b"ZSTD_compressStream() error: %s\n\x00" as *const u8 as
                    *const libc::c_char, ZSTD_getErrorName(toRead));
        }
        (*w).fill =
            ((*w).fill as libc::c_ulong).wrapping_add(output.pos) as size_t as
                size_t;
        (*w).compressed_size =
            (*w).compressed_size.wrapping_add(output.pos as
                                                  libc::c_ulonglong);
        if (*w).fill.wrapping_add(zstd_stream_recommended_out_buffer_size) >=
               (*w).allocated {
            compressor_create_file(w);
            fwrite_or_die((*w).buf as *const libc::c_void,
                          1 as libc::c_int as size_t, (*w).fill, (*w).file);
            (*w).written =
                (*w).written.wrapping_add((*w).fill as libc::c_ulonglong);
            (*w).fill = 0 as libc::c_int as size_t
        }
    }
    (*w).uncompressed_size =
        (*w).uncompressed_size.wrapping_add(size as libc::c_ulonglong);
}
unsafe extern "C" fn write_compressed_data(mut F: *mut FILE,
                                           mut w: *mut compressor_t) {
    if (*w).compressed_size < 4 as libc::c_int as libc::c_ulonglong {
        die(b"compression failed\n\x00" as *const u8 as *const libc::c_char);
    }
    write_variable_length_encoded_number(F,
                                         (*w).compressed_size.wrapping_sub(4
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_ulonglong));
    if (*w).file.is_null() {
        if (*w).fill > 0 as libc::c_int as libc::c_ulong {
            if (*w).fill < 4 as libc::c_int as libc::c_ulong {
                die(b"compression failed\n\x00" as *const u8 as
                        *const libc::c_char);
            }
            fwrite_or_die((*w).buf.offset(4 as libc::c_int as isize) as
                              *const libc::c_void, 1 as libc::c_int as size_t,
                          (*w).fill.wrapping_sub(4 as libc::c_int as
                                                     libc::c_ulong), F);
        }
    } else {
        copy_file_to_out((*w).file, (*w).path,
                         4 as libc::c_int as libc::c_long,
                         (*w).written.wrapping_sub(4 as libc::c_int as
                                                       libc::c_ulonglong));
        if (*w).fill > 0 as libc::c_int as libc::c_ulong {
            fwrite_or_die((*w).buf as *const libc::c_void,
                          1 as libc::c_int as size_t, (*w).fill, F);
        }
    };
}
/*
 * NAF compressor
 * Copyright (c) 2018-2020 Kirill Kryukov
 * See README.md and LICENSE files of this repository
 */
unsafe extern "C" fn init_encoders() {
    zstd_stream_recommended_out_buffer_size = ZSTD_CStreamOutSize();
    out_4bit_buffer_size = ZSTD_CStreamOutSize();
    out_4bit_buffer =
        malloc_or_die(out_4bit_buffer_size) as *mut libc::c_uchar;
    out_4bit_pos = out_4bit_buffer;
    file_copy_buffer =
        malloc_or_die(131072 as libc::c_int as size_t) as *mut libc::c_uchar;
    length_units =
        malloc_or_die((::std::mem::size_of::<libc::c_uint>() as
                           libc::c_ulong).wrapping_mul(4096 as libc::c_int as
                                                           libc::c_ulong)) as
            *mut libc::c_uint;
    mask_units =
        malloc_or_die(16384 as libc::c_int as size_t) as *mut libc::c_uchar;
    mask_units_end = mask_units.offset(16384 as libc::c_int as isize);
    mask_units_pos = mask_units;
}
unsafe extern "C" fn encode_dna(mut str: *const libc::c_uchar,
                                mut size: size_t) {
    let mut end: *const libc::c_uchar = str.offset(size as isize);
    let mut p: *const libc::c_uchar = str;
    if p < end && parity as libc::c_int != 0 {
        let fresh0 = out_4bit_pos;
        out_4bit_pos = out_4bit_pos.offset(1);
        *fresh0 =
            (*fresh0 as libc::c_int |
                 (nuc_code[*p as usize] as libc::c_int * 16 as libc::c_int) as
                     libc::c_uchar as libc::c_int) as libc::c_uchar;
        if out_4bit_pos >=
               out_4bit_buffer.offset(out_4bit_buffer_size as isize) {
            compress(&mut SEQ, out_4bit_buffer as *const libc::c_void,
                     out_4bit_buffer_size);
            out_4bit_pos = out_4bit_buffer
        }
        parity = 0 as libc::c_int != 0;
        p = p.offset(1)
    }
    let mut end1: *const libc::c_uchar =
        p.offset((end.wrapping_offset_from(p) as libc::c_long as
                      libc::c_ulonglong & !(1 as libc::c_ulonglong)) as
                     isize);
    while p < end1 {
        let fresh1 = out_4bit_pos;
        out_4bit_pos = out_4bit_pos.offset(1);
        *fresh1 =
            (nuc_code[*p as usize] as libc::c_int |
                 (nuc_code[*p.offset(1 as libc::c_int as isize) as usize] as
                      libc::c_int * 16 as libc::c_int) as libc::c_uchar as
                     libc::c_int) as libc::c_uchar;
        if out_4bit_pos >=
               out_4bit_buffer.offset(out_4bit_buffer_size as isize) {
            compress(&mut SEQ, out_4bit_buffer as *const libc::c_void,
                     out_4bit_buffer_size);
            out_4bit_pos = out_4bit_buffer
        }
        p = p.offset(2 as libc::c_int as isize)
    }
    if p < end {
        *out_4bit_pos = nuc_code[*p as usize];
        parity = 1 as libc::c_int != 0
    };
}
unsafe extern "C" fn add_length(mut len: size_t) {
    while len as libc::c_ulonglong >= 0xffffffff as libc::c_ulonglong {
        let fresh2 = length_unit_index;
        length_unit_index = length_unit_index.wrapping_add(1);
        *length_units.offset(fresh2 as isize) = 0xffffffff as libc::c_uint;
        len =
            (len as
                 libc::c_ulonglong).wrapping_sub(0xffffffff as
                                                     libc::c_ulonglong) as
                size_t as size_t;
        if length_unit_index >= 4096 as libc::c_int as libc::c_uint {
            compress(&mut LEN, length_units as *const libc::c_void,
                     (::std::mem::size_of::<libc::c_uint>() as
                          libc::c_ulong).wrapping_mul(4096 as libc::c_int as
                                                          libc::c_ulong));
            length_unit_index = 0 as libc::c_int as libc::c_uint
        }
    }
    let fresh3 = length_unit_index;
    length_unit_index = length_unit_index.wrapping_add(1);
    *length_units.offset(fresh3 as isize) = len as libc::c_uint;
    if length_unit_index >= 4096 as libc::c_int as libc::c_uint {
        compress(&mut LEN, length_units as *const libc::c_void,
                 (::std::mem::size_of::<libc::c_uint>() as
                      libc::c_ulong).wrapping_mul(4096 as libc::c_int as
                                                      libc::c_ulong));
        length_unit_index = 0 as libc::c_int as libc::c_uint
    };
}
unsafe extern "C" fn add_mask(mut len: libc::c_ulonglong) {
    while len >= 255 as libc::c_ulonglong {
        let fresh4 = mask_units_pos;
        mask_units_pos = mask_units_pos.offset(1);
        *fresh4 = 255 as libc::c_int as libc::c_uchar;
        len = len.wrapping_sub(255 as libc::c_ulonglong);
        if mask_units_pos >= mask_units_end {
            compress(&mut MASK, mask_units as *const libc::c_void,
                     16384 as libc::c_int as size_t);
            mask_units_pos = mask_units
        }
    }
    let fresh5 = mask_units_pos;
    mask_units_pos = mask_units_pos.offset(1);
    *fresh5 = len as libc::c_uchar;
    if mask_units_pos >= mask_units_end {
        compress(&mut MASK, mask_units as *const libc::c_void,
                 16384 as libc::c_int as size_t);
        mask_units_pos = mask_units
    };
}
unsafe extern "C" fn extract_mask(mut seq_0: *const libc::c_uchar,
                                  mut len: size_t) {
    let mut end: *const libc::c_uchar = seq_0.offset(len as isize);
    let mut c: *const libc::c_uchar = seq_0;
    while c < end {
        if mask_on as libc::c_int !=
               (*c as libc::c_int >= 96 as libc::c_int) as libc::c_int {
            add_mask(mask_len);
            mask_len = 0 as libc::c_int as libc::c_ulonglong;
            mask_on = !mask_on
        }
        let mut start: *const libc::c_uchar = c;
        if mask_on {
            while c < end && *c as libc::c_int >= 96 as libc::c_int {
                c = c.offset(1)
            }
        } else {
            while c < end && (*c as libc::c_int) < 96 as libc::c_int {
                c = c.offset(1)
            }
        }
        mask_len =
            mask_len.wrapping_add(c.wrapping_offset_from(start) as
                                      libc::c_long as libc::c_ulonglong)
    };
}
/*
 * NAF compressor
 * Copyright (c) 2018-2020 Kirill Kryukov
 * See README.md and LICENSE files of this repository
 */
/*
 * Copies the content of an already open file into output stream.
 * Starts from "start", copies exactly "expected_size" bytes.
 */
unsafe extern "C" fn copy_file_to_out(mut FROM: *mut FILE,
                                      mut from_path: *mut libc::c_char,
                                      mut start: libc::c_long,
                                      mut data_size: libc::c_ulonglong) {
    fflush_or_die(FROM);
    if fseek(FROM, start, 0 as libc::c_int) != 0 as libc::c_int {
        die(b"can\'t seek to data start in \"%s\"\n\x00" as *const u8 as
                *const libc::c_char, from_path);
    }
    let mut remaining: libc::c_ulonglong = data_size;
    while remaining > 0 as libc::c_int as libc::c_ulonglong {
        let mut to_read: size_t =
            if 131072 as libc::c_int as libc::c_ulonglong <= remaining {
                131072 as libc::c_int as libc::c_ulonglong
            } else { remaining } as size_t;
        fread_or_die(file_copy_buffer as *mut libc::c_void,
                     1 as libc::c_int as size_t, to_read, FROM);
        fwrite_or_die(file_copy_buffer as *const libc::c_void,
                      1 as libc::c_int as size_t, to_read, OUT);
        remaining = remaining.wrapping_sub(to_read as libc::c_ulonglong)
    };
}
unsafe extern "C" fn write_variable_length_encoded_number(mut F: *mut FILE,
                                                          mut a:
                                                              libc::c_ulonglong) {
    let mut vle_buffer: [libc::c_uchar; 10] = [0; 10];
    let mut b: *mut libc::c_uchar =
        vle_buffer.as_mut_ptr().offset(10 as libc::c_int as isize);
    b = b.offset(-1);
    *b = (a & 127 as libc::c_ulonglong) as libc::c_uchar;
    a >>= 7 as libc::c_int;
    while a > 0 as libc::c_int as libc::c_ulonglong {
        b = b.offset(-1);
        *b =
            (128 as libc::c_ulonglong | a & 127 as libc::c_ulonglong) as
                libc::c_uchar;
        a >>= 7 as libc::c_int
    }
    let mut len: size_t =
        vle_buffer.as_mut_ptr().offset(10 as libc::c_int as
                                           isize).wrapping_offset_from(b) as
            libc::c_long as size_t;
    fwrite_or_die(b as *const libc::c_void, 1 as libc::c_int as size_t, len,
                  F);
}
unsafe extern "C" fn name_writer(mut str: *mut libc::c_uchar,
                                 mut size: size_t) {
    compress(&mut IDS, str as *const libc::c_void, size);
}
unsafe extern "C" fn comm_writer(mut str: *mut libc::c_uchar,
                                 mut size: size_t) {
    compress(&mut COMM, str as *const libc::c_void, size);
}
unsafe extern "C" fn seq_writer_masked_4bit(mut str: *mut libc::c_uchar,
                                            mut size: size_t) {
    seq_size_original =
        seq_size_original.wrapping_add(size as libc::c_ulonglong);
    extract_mask(str, size);
    encode_dna(str, size);
}
unsafe extern "C" fn seq_writer_nonmasked_4bit(mut str: *mut libc::c_uchar,
                                               mut size: size_t) {
    seq_size_original =
        seq_size_original.wrapping_add(size as libc::c_ulonglong);
    encode_dna(str, size);
}
unsafe extern "C" fn seq_writer_masked_text(mut str: *mut libc::c_uchar,
                                            mut size: size_t) {
    seq_size_original =
        seq_size_original.wrapping_add(size as libc::c_ulonglong);
    compress(&mut SEQ, str as *const libc::c_void, size);
}
unsafe extern "C" fn seq_writer_nonmasked_text(mut str: *mut libc::c_uchar,
                                               mut size: size_t) {
    seq_size_original =
        seq_size_original.wrapping_add(size as libc::c_ulonglong);
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < size {
        *str.offset(i as isize) =
            ({
                 let mut __res: libc::c_int = 0;
                 if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong >
                        1 as libc::c_int as libc::c_ulong {
                     if 0 != 0 {
                         let mut __c: libc::c_int =
                             *str.offset(i as isize) as libc::c_int;
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
                             toupper(*str.offset(i as isize) as libc::c_int)
                     }
                 } else {
                     __res =
                         *(*__ctype_toupper_loc()).offset(*str.offset(i as
                                                                          isize)
                                                              as libc::c_int
                                                              as isize)
                 }
                 __res
             }) as libc::c_uchar;
        i = i.wrapping_add(1)
    }
    compress(&mut SEQ, str as *const libc::c_void, size);
}
unsafe extern "C" fn qual_writer(mut str: *mut libc::c_uchar,
                                 mut size: size_t) {
    compress(&mut QUAL, str as *const libc::c_void, size);
}
static mut name: string_t =
    {
        let mut init =
            string_t{length: 0 as libc::c_int as size_t,
                     data: 0 as *const libc::c_uchar as *mut libc::c_uchar,
                     writer:
                         Some(name_writer as
                                  unsafe extern "C" fn(_: *mut libc::c_uchar,
                                                       _: size_t) -> ()),};
        init
    };
static mut comment: string_t =
    {
        let mut init =
            string_t{length: 0 as libc::c_int as size_t,
                     data: 0 as *const libc::c_uchar as *mut libc::c_uchar,
                     writer:
                         Some(comm_writer as
                                  unsafe extern "C" fn(_: *mut libc::c_uchar,
                                                       _: size_t) -> ()),};
        init
    };
static mut seq: string_t =
    {
        let mut init =
            string_t{length: 0 as libc::c_int as size_t,
                     data: 0 as *const libc::c_uchar as *mut libc::c_uchar,
                     writer: None,};
        init
    };
static mut qual: string_t =
    {
        let mut init =
            string_t{length: 0 as libc::c_int as size_t,
                     data: 0 as *const libc::c_uchar as *mut libc::c_uchar,
                     writer:
                         Some(qual_writer as
                                  unsafe extern "C" fn(_: *mut libc::c_uchar,
                                                       _: size_t) -> ()),};
        init
    };
unsafe extern "C" fn report_unexpected_char_stats(mut n:
                                                      *mut libc::c_ulonglong,
                                                  mut seq_type_name:
                                                      *const libc::c_char) {
    let mut total: libc::c_ulonglong = 0 as libc::c_int as libc::c_ulonglong;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < 257 as libc::c_int as libc::c_uint {
        total = total.wrapping_add(*n.offset(i as isize));
        i = i.wrapping_add(1)
    }
    if total > 0 as libc::c_int as libc::c_ulonglong {
        msg(b"input has %llu unexpected %s characters:\n\x00" as *const u8 as
                *const libc::c_char, total, seq_type_name);
        let mut i_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while i_0 < 32 as libc::c_int as libc::c_uint {
            if *n.offset(i_0 as isize) !=
                   0 as libc::c_int as libc::c_ulonglong {
                msg(b"    \'\\x%02X\': %llu\n\x00" as *const u8 as
                        *const libc::c_char, i_0, *n.offset(i_0 as isize));
            }
            i_0 = i_0.wrapping_add(1)
        }
        let mut i_1: libc::c_uint = 32 as libc::c_int as libc::c_uint;
        while i_1 < 127 as libc::c_int as libc::c_uint {
            if *n.offset(i_1 as isize) !=
                   0 as libc::c_int as libc::c_ulonglong {
                msg(b"    \'%c\': %llu\n\x00" as *const u8 as
                        *const libc::c_char,
                    i_1 as libc::c_uchar as libc::c_int,
                    *n.offset(i_1 as isize));
            }
            i_1 = i_1.wrapping_add(1)
        }
        let mut i_2: libc::c_uint = 127 as libc::c_int as libc::c_uint;
        while i_2 < 256 as libc::c_int as libc::c_uint {
            if *n.offset(i_2 as isize) !=
                   0 as libc::c_int as libc::c_ulonglong {
                msg(b"    \'\\x%02X\': %llu\n\x00" as *const u8 as
                        *const libc::c_char, i_2, *n.offset(i_2 as isize));
            }
            i_2 = i_2.wrapping_add(1)
        }
        if *n.offset(256 as libc::c_int as isize) !=
               0 as libc::c_int as libc::c_ulonglong {
            msg(b"    EOF: %llu\n\x00" as *const u8 as *const libc::c_char,
                *n.offset(256 as libc::c_int as isize));
        }
    };
}
unsafe extern "C" fn report_unexpected_input_char_stats() {
    report_unexpected_char_stats(n_unexpected_id_characters.as_mut_ptr(),
                                 b"id\x00" as *const u8 as
                                     *const libc::c_char);
    report_unexpected_char_stats(n_unexpected_comment_characters.as_mut_ptr(),
                                 b"comment\x00" as *const u8 as
                                     *const libc::c_char);
    report_unexpected_char_stats(n_unexpected_seq_characters.as_mut_ptr(),
                                 in_seq_type_name);
    report_unexpected_char_stats(n_unexpected_qual_characters.as_mut_ptr(),
                                 b"quality\x00" as *const u8 as
                                     *const libc::c_char);
}
#[cold]
unsafe extern "C" fn unexpected_id_char(mut c: libc::c_uint) {
    if abort_on_unexpected_code {
        die(b"unexpected character \'%c\' in ID of sequence %llu\n\x00" as
                *const u8 as *const libc::c_char,
            c as libc::c_uchar as libc::c_int,
            n_sequences.wrapping_add(1 as libc::c_int as libc::c_ulonglong));
    } else {
        n_unexpected_id_characters[c as usize] =
            n_unexpected_id_characters[c as usize].wrapping_add(1)
    };
}
#[cold]
unsafe extern "C" fn unexpected_comment_char(mut c: libc::c_uint) {
    if abort_on_unexpected_code {
        die(b"unexpected character \'%c\' in comment of sequence %llu\n\x00"
                as *const u8 as *const libc::c_char,
            c as libc::c_uchar as libc::c_int,
            n_sequences.wrapping_add(1 as libc::c_int as libc::c_ulonglong));
    } else {
        n_unexpected_comment_characters[c as usize] =
            n_unexpected_comment_characters[c as usize].wrapping_add(1)
    };
}
#[cold]
unsafe extern "C" fn unexpected_input_char(mut c: libc::c_uint) {
    if abort_on_unexpected_code {
        die(b"unexpected %s code \'%c\' in sequence %llu\n\x00" as *const u8
                as *const libc::c_char, in_seq_type_name,
            c as libc::c_uchar as libc::c_int,
            n_sequences.wrapping_add(1 as libc::c_int as libc::c_ulonglong));
    } else {
        n_unexpected_seq_characters[c as usize] =
            n_unexpected_seq_characters[c as usize].wrapping_add(1)
    };
}
#[cold]
unsafe extern "C" fn unexpected_quality_char(mut c: libc::c_uint) {
    if abort_on_unexpected_code {
        die(b"unexpected quality code \'%c\' in sequence %llu\n\x00" as
                *const u8 as *const libc::c_char,
            c as libc::c_uchar as libc::c_int,
            n_sequences.wrapping_add(1 as libc::c_int as libc::c_ulonglong));
    } else {
        n_unexpected_qual_characters[c as usize] =
            n_unexpected_qual_characters[c as usize].wrapping_add(1)
    };
}
#[inline(always)]
unsafe extern "C" fn refill_in_buffer() {
    in_begin = 0 as libc::c_int as size_t;
    in_end =
        fread(in_buffer as *mut libc::c_void, 1 as libc::c_int as size_t,
              16384 as libc::c_int as size_t, IN);
}
#[inline(always)]
unsafe extern "C" fn in_get_char() -> libc::c_uint {
    if in_begin >= in_end {
        refill_in_buffer();
        if in_end == 0 as libc::c_int as libc::c_ulong {
            return 256 as libc::c_int as libc::c_uint
        }
    }
    let fresh6 = in_begin;
    in_begin = in_begin.wrapping_add(1);
    return *in_buffer.offset(fresh6 as isize) as libc::c_uint;
}
#[inline]
unsafe extern "C" fn in_skip_until(mut delim_arr: *const bool)
 -> libc::c_uint {
    let mut d: libc::c_uint = 256 as libc::c_int as libc::c_uint;
    loop  {
        if in_begin >= in_end {
            refill_in_buffer();
            if in_end == 0 as libc::c_int as libc::c_ulong { break ; }
        }
        let mut i: size_t = 0;
        i = in_begin;
        while i < in_end {
            if *delim_arr.offset(*in_buffer.offset(i as isize) as isize) {
                d = *in_buffer.offset(i as isize) as libc::c_uint;
                break ;
            } else { i = i.wrapping_add(1) }
        }
        in_begin = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
        if d != 256 as libc::c_int as libc::c_uint { break ; }
    }
    return d;
}
/*
 * Reads input until a specific delimiter character is found.
 * Stores text until delimiter into 'str' (not including delimiter).
 * Returns delimiter, or INEOF at end of input.
 * Does NOT zero-terminate the text stored in 'str'.
 * Whenever 'str' fills, writes it out.
 */
#[inline]
unsafe extern "C" fn in_get_until_specific_char(delim: libc::c_uchar,
                                                mut str: *mut string_t)
 -> libc::c_uint {
    let mut d: libc::c_uint = 256 as libc::c_int as libc::c_uint;
    loop  {
        if in_begin >= in_end {
            refill_in_buffer();
            if in_end == 0 as libc::c_int as libc::c_ulong { break ; }
        }
        let mut i: size_t = 0;
        i = in_begin;
        while i < in_end {
            if *in_buffer.offset(i as isize) as libc::c_int ==
                   delim as libc::c_int {
                d = delim as libc::c_uint;
                break ;
            } else { i = i.wrapping_add(1) }
        }
        let mut s: size_t = i.wrapping_sub(in_begin);
        if (*str).length.wrapping_add(s) as libc::c_ulonglong >=
               (1 as
                    libc::c_ulonglong).wrapping_mul(1000 as libc::c_int as
                                                        libc::c_ulonglong).wrapping_mul(1000
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_ulonglong)
           {
            let mut s1: size_t =
                (1 as
                     libc::c_ulonglong).wrapping_mul(1000 as libc::c_int as
                                                         libc::c_ulonglong).wrapping_mul(1000
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulonglong).wrapping_sub((*str).length
                                                                                                                                 as
                                                                                                                                 libc::c_ulonglong)
                    as size_t;
            let mut s2: size_t = s.wrapping_sub(s1);
            memcpy((*str).data.offset((*str).length as isize) as
                       *mut libc::c_void,
                   in_buffer.offset(in_begin as isize) as *const libc::c_void,
                   s1);
            (*str).writer.expect("non-null function pointer")((*str).data,
                                                              (1 as
                                                                   libc::c_ulonglong).wrapping_mul(1000
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulonglong).wrapping_mul(1000
                                                                                                                                           as
                                                                                                                                           libc::c_int
                                                                                                                                           as
                                                                                                                                           libc::c_ulonglong)
                                                                  as size_t);
            memcpy((*str).data as *mut libc::c_void,
                   in_buffer.offset(in_begin as isize).offset(s1 as isize) as
                       *const libc::c_void, s2);
            (*str).length = s2
        } else {
            memcpy((*str).data.offset((*str).length as isize) as
                       *mut libc::c_void,
                   in_buffer.offset(in_begin as isize) as *const libc::c_void,
                   s);
            (*str).length =
                ((*str).length as libc::c_ulong).wrapping_add(s) as size_t as
                    size_t
        }
        in_begin = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
        if d != 256 as libc::c_int as libc::c_uint { break ; }
    }
    return d;
}
/*
 * Reads input until a delimiter character is found (any matching 'delim_arr').
 * Stores text until delimiter into 'str' (not including delimiter).
 * Returns delimiter, or INEOF at end of input.
 * Does NOT zero-terminate the text stored in 'str'.
 * Whenever 'str' fills, writes it out.
 */
#[inline]
unsafe extern "C" fn in_get_until(mut delim_arr: *const bool,
                                  mut str: *mut string_t) -> libc::c_uint {
    let mut d: libc::c_uint = 256 as libc::c_int as libc::c_uint;
    loop  {
        if in_begin >= in_end {
            refill_in_buffer();
            if in_end == 0 as libc::c_int as libc::c_ulong { break ; }
        }
        let mut i: size_t = 0;
        i = in_begin;
        while i < in_end {
            if *delim_arr.offset(*in_buffer.offset(i as isize) as isize) {
                d = *in_buffer.offset(i as isize) as libc::c_uint;
                break ;
            } else { i = i.wrapping_add(1) }
        }
        let mut s: size_t = i.wrapping_sub(in_begin);
        if (*str).length.wrapping_add(s) as libc::c_ulonglong >=
               (1 as
                    libc::c_ulonglong).wrapping_mul(1000 as libc::c_int as
                                                        libc::c_ulonglong).wrapping_mul(1000
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_ulonglong)
           {
            let mut s1: size_t =
                (1 as
                     libc::c_ulonglong).wrapping_mul(1000 as libc::c_int as
                                                         libc::c_ulonglong).wrapping_mul(1000
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulonglong).wrapping_sub((*str).length
                                                                                                                                 as
                                                                                                                                 libc::c_ulonglong)
                    as size_t;
            let mut s2: size_t = s.wrapping_sub(s1);
            memcpy((*str).data.offset((*str).length as isize) as
                       *mut libc::c_void,
                   in_buffer.offset(in_begin as isize) as *const libc::c_void,
                   s1);
            (*str).writer.expect("non-null function pointer")((*str).data,
                                                              (1 as
                                                                   libc::c_ulonglong).wrapping_mul(1000
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulonglong).wrapping_mul(1000
                                                                                                                                           as
                                                                                                                                           libc::c_int
                                                                                                                                           as
                                                                                                                                           libc::c_ulonglong)
                                                                  as size_t);
            memcpy((*str).data as *mut libc::c_void,
                   in_buffer.offset(in_begin as isize).offset(s1 as isize) as
                       *const libc::c_void, s2);
            (*str).length = s2
        } else {
            memcpy((*str).data.offset((*str).length as isize) as
                       *mut libc::c_void,
                   in_buffer.offset(in_begin as isize) as *const libc::c_void,
                   s);
            (*str).length =
                ((*str).length as libc::c_ulong).wrapping_add(s) as size_t as
                    size_t
        }
        in_begin = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
        if d != 256 as libc::c_int as libc::c_uint { break ; }
    }
    return d;
}
#[inline(always)]
unsafe extern "C" fn str_append_char(mut str: *mut string_t,
                                     mut c: libc::c_uchar) {
    *(*str).data.offset((*str).length as isize) = c;
    (*str).length = (*str).length.wrapping_add(1);
    if (*str).length as libc::c_ulonglong >=
           (1 as
                libc::c_ulonglong).wrapping_mul(1000 as libc::c_int as
                                                    libc::c_ulonglong).wrapping_mul(1000
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_ulonglong)
       {
        (*str).writer.expect("non-null function pointer")((*str).data,
                                                          (1 as
                                                               libc::c_ulonglong).wrapping_mul(1000
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_ulonglong).wrapping_mul(1000
                                                                                                                                       as
                                                                                                                                       libc::c_int
                                                                                                                                       as
                                                                                                                                       libc::c_ulonglong)
                                                              as size_t);
        (*str).length = 0 as libc::c_int as size_t
    };
}
unsafe extern "C" fn process_well_formed_fasta() {
    let mut c: libc::c_uint = 0;
    loop  {
        c = in_get_until(is_well_formed_space_arr.as_ptr(), &mut name);
        str_append_char(&mut name, '\u{0}' as i32 as libc::c_uchar);
        if c == ' ' as i32 as libc::c_uint {
            c =
                in_get_until_specific_char('\n' as i32 as libc::c_uchar,
                                           &mut comment)
        }
        str_append_char(&mut comment, '\u{0}' as i32 as libc::c_uchar);
        let mut old_total_seq_size: libc::c_ulonglong =
            seq_size_original.wrapping_add(seq.length as libc::c_ulonglong);
        if c != 256 as libc::c_int as libc::c_uint {
            let mut old_len: libc::c_ulonglong = old_total_seq_size;
            loop  {
                c =
                    in_get_until_specific_char('\n' as i32 as libc::c_uchar,
                                               &mut seq);
                if !(c != 256 as libc::c_int as libc::c_uint) { break ; }
                let mut new_len: libc::c_ulonglong =
                    seq_size_original.wrapping_add(seq.length as
                                                       libc::c_ulonglong);
                if new_len.wrapping_sub(old_len) > longest_line_length {
                    longest_line_length = new_len.wrapping_sub(old_len)
                }
                old_len = new_len;
                c = in_get_char();
                if c == '>' as i32 as libc::c_uint ||
                       c == 256 as libc::c_int as libc::c_uint {
                    break ;
                }
                in_begin = in_begin.wrapping_sub(1)
            }
            // If the last line is the longest, and has no end-of-line character, handle it correctly.
            if c == 256 as libc::c_int as libc::c_uint {
                let mut new_len_0: libc::c_ulonglong =
                    seq_size_original.wrapping_add(seq.length as
                                                       libc::c_ulonglong);
                if new_len_0.wrapping_sub(old_len) > longest_line_length {
                    longest_line_length = new_len_0.wrapping_sub(old_len)
                }
            }
        }
        add_length(seq_size_original.wrapping_add(seq.length as
                                                      libc::c_ulonglong).wrapping_sub(old_total_seq_size)
                       as size_t);
        n_sequences = n_sequences.wrapping_add(1);
        if !(c != 256 as libc::c_int as libc::c_uint) { break ; }
    };
}
unsafe extern "C" fn process_non_well_formed_fasta() {
    let mut c: libc::c_uint = 0;
    loop  {
        loop 
             // At this point the '>' was already read, so we immediately proceed to read the name.
             {
            c = in_get_until(is_unexpected_text_arr.as_mut_ptr(), &mut name);
            if !(c != 256 as libc::c_int as libc::c_uint) { break ; }
            if is_space_arr[c as usize] { break ; }
            unexpected_id_char(c);
            str_append_char(&mut seq, unexpected_name_char_replacement);
        }
        str_append_char(&mut name, '\u{0}' as i32 as libc::c_uchar);
        if c != 256 as libc::c_int as libc::c_uint && !is_eol_arr[c as usize]
           {
            loop  {
                c =
                    in_get_until(is_unexpected_comment_arr.as_ptr(),
                                 &mut comment);
                if !(c != 256 as libc::c_int as libc::c_uint) { break ; }
                if is_eol_arr[c as usize] { break ; }
                unexpected_comment_char(c);
                str_append_char(&mut comment,
                                unexpected_name_char_replacement);
            }
        }
        str_append_char(&mut comment, '\u{0}' as i32 as libc::c_uchar);
        let mut old_total_seq_size: libc::c_ulonglong =
            seq_size_original.wrapping_add(seq.length as libc::c_ulonglong);
        if c != 256 as libc::c_int as libc::c_uint {
            let mut old_len: libc::c_ulonglong = old_total_seq_size;
            loop  {
                c = in_get_until(is_unexpected_arr, &mut seq);
                if !(c != 256 as libc::c_int as libc::c_uint) { break ; }
                if is_eol_arr[c as usize] {
                    let mut new_len: libc::c_ulonglong =
                        seq_size_original.wrapping_add(seq.length as
                                                           libc::c_ulonglong);
                    if new_len.wrapping_sub(old_len) > longest_line_length {
                        longest_line_length = new_len.wrapping_sub(old_len)
                    }
                    old_len = new_len;
                    c = in_get_char();
                    if !*is_unexpected_arr.offset(c as isize) {
                        str_append_char(&mut seq, c as libc::c_uchar);
                    } else {
                        if c == '>' as i32 as libc::c_uint ||
                               c == 256 as libc::c_int as libc::c_uint {
                            break ;
                        }
                        if is_eol_arr[c as usize] {
                            while c != 256 as libc::c_int as libc::c_uint &&
                                      is_eol_arr[c as usize] as libc::c_int !=
                                          0 {
                                c = in_get_char()
                            }
                            if c == '>' as i32 as libc::c_uint ||
                                   c == 256 as libc::c_int as libc::c_uint {
                                break ;
                            }
                            if !*is_unexpected_arr.offset(c as isize) {
                                str_append_char(&mut seq, c as libc::c_uchar);
                            } else if !is_space_arr[c as usize] {
                                unexpected_input_char(c);
                                str_append_char(&mut seq,
                                                unexpected_seq_char_replacement);
                            }
                        } else if !is_space_arr[c as usize] {
                            unexpected_input_char(c);
                            str_append_char(&mut seq,
                                            unexpected_seq_char_replacement);
                        }
                    }
                } else if !is_space_arr[c as usize] {
                    if c == '>' as i32 as libc::c_uint &&
                           in_seq_type == seq_type_text as libc::c_int {
                        str_append_char(&mut seq, c as libc::c_uchar);
                    } else {
                        unexpected_input_char(c);
                        str_append_char(&mut seq,
                                        unexpected_seq_char_replacement);
                    }
                }
            }
            // If the last line is the longest, and has no end-of-line character, handle it correctly.
            if c == 256 as libc::c_int as libc::c_uint {
                let mut new_len_0: libc::c_ulonglong =
                    seq_size_original.wrapping_add(seq.length as
                                                       libc::c_ulonglong);
                if new_len_0.wrapping_sub(old_len) > longest_line_length {
                    longest_line_length = new_len_0.wrapping_sub(old_len)
                }
            }
        }
        add_length(seq_size_original.wrapping_add(seq.length as
                                                      libc::c_ulonglong).wrapping_sub(old_total_seq_size)
                       as size_t);
        n_sequences = n_sequences.wrapping_add(1);
        if !(c != 256 as libc::c_int as libc::c_uint) { break ; }
    };
}
unsafe extern "C" fn process_well_formed_fastq() {
    let mut c: libc::c_uint = 0;
    loop  {
        c = in_get_until(is_well_formed_space_arr.as_ptr(), &mut name);
        str_append_char(&mut name, '\u{0}' as i32 as libc::c_uchar);
        if c == ' ' as i32 as libc::c_uint {
            c =
                in_get_until_specific_char('\n' as i32 as libc::c_uchar,
                                           &mut comment)
        }
        str_append_char(&mut comment, '\u{0}' as i32 as libc::c_uchar);
        if c == 256 as libc::c_int as libc::c_uint {
            die(b"truncated FASTQ input: last sequence has no sequence data\n\x00"
                    as *const u8 as *const libc::c_char);
        }
        let mut old_len: libc::c_ulonglong =
            seq_size_original.wrapping_add(seq.length as libc::c_ulonglong);
        c =
            in_get_until_specific_char('\n' as i32 as libc::c_uchar,
                                       &mut seq);
        let mut read_length: libc::c_ulonglong =
            seq_size_original.wrapping_add(seq.length as
                                               libc::c_ulonglong).wrapping_sub(old_len);
        if read_length > longest_line_length {
            longest_line_length = read_length
        }
        c = in_get_char();
        if c != '+' as i32 as libc::c_uint {
            if c == 256 as libc::c_int as libc::c_uint {
                die(b"truncated FASTQ input: last sequence has no quality\n\x00"
                        as *const u8 as *const libc::c_char);
            } else {
                die(b"not well-formed FASTQ input\n\x00" as *const u8 as
                        *const libc::c_char);
            }
        }
        c = in_get_char();
        if c != '\n' as i32 as libc::c_uint {
            die(b"not well-formed FASTQ input\n\x00" as *const u8 as
                    *const libc::c_char);
        }
        old_len =
            QUAL.uncompressed_size.wrapping_add(qual.length as
                                                    libc::c_ulonglong);
        c =
            in_get_until_specific_char('\n' as i32 as libc::c_uchar,
                                       &mut qual);
        if QUAL.uncompressed_size.wrapping_add(qual.length as
                                                   libc::c_ulonglong).wrapping_sub(old_len)
               != read_length {
            die(b"quality length of sequence %llu doesn\'t match sequence length\n\x00"
                    as *const u8 as *const libc::c_char,
                n_sequences.wrapping_add(1 as libc::c_int as
                                             libc::c_ulonglong));
        }
        add_length(read_length as size_t);
        n_sequences = n_sequences.wrapping_add(1);
        c = in_get_char();
        if !(c != '@' as i32 as libc::c_uint) { continue ; }
        if c == 256 as libc::c_int as libc::c_uint { break ; }
        die(b"not well-formed FASTQ input\n\x00" as *const u8 as
                *const libc::c_char);
    };
}
unsafe extern "C" fn process_non_well_formed_fastq() {
    let mut c: libc::c_uint = 0;
    loop  {
        loop  {
            c = in_get_until(is_unexpected_text_arr.as_mut_ptr(), &mut name);
            if !(c != 256 as libc::c_int as libc::c_uint) { break ; }
            if is_space_arr[c as usize] { break ; }
            unexpected_id_char(c);
            str_append_char(&mut seq, unexpected_name_char_replacement);
        }
        str_append_char(&mut name, '\u{0}' as i32 as libc::c_uchar);
        if c != 256 as libc::c_int as libc::c_uint && !is_eol_arr[c as usize]
           {
            loop  {
                c =
                    in_get_until(is_unexpected_comment_arr.as_ptr(),
                                 &mut comment);
                if !(c != 256 as libc::c_int as libc::c_uint) { break ; }
                if is_eol_arr[c as usize] { break ; }
                unexpected_comment_char(c);
                str_append_char(&mut comment,
                                unexpected_name_char_replacement);
            }
        }
        str_append_char(&mut comment, '\u{0}' as i32 as libc::c_uchar);
        if c == 256 as libc::c_int as libc::c_uint {
            die(b"truncated FASTQ input: last sequence has no sequence data\n\x00"
                    as *const u8 as *const libc::c_char);
        }
        let mut old_len: libc::c_ulonglong =
            seq_size_original.wrapping_add(seq.length as libc::c_ulonglong);
        loop  {
            c = in_get_until(is_unexpected_arr, &mut seq);
            if !(c != 256 as libc::c_int as libc::c_uint) { break ; }
            if is_eol_arr[c as usize] { break ; }
            if !is_space_arr[c as usize] {
                unexpected_input_char(c);
                str_append_char(&mut seq, unexpected_seq_char_replacement);
            }
        }
        let mut read_length: libc::c_ulonglong =
            seq_size_original.wrapping_add(seq.length as
                                               libc::c_ulonglong).wrapping_sub(old_len);
        if read_length > longest_line_length {
            longest_line_length = read_length
        }
        if c == 256 as libc::c_int as libc::c_uint {
            die(b"truncated FASTQ input: last sequence has no quality\n\x00"
                    as *const u8 as *const libc::c_char);
        }
        loop  { c = in_get_char(); if !is_eol_arr[c as usize] { break ; } }
        if c == 256 as libc::c_int as libc::c_uint {
            die(b"truncated FASTQ input: last sequence has no quality\n\x00"
                    as *const u8 as *const libc::c_char);
        }
        if c != '+' as i32 as libc::c_uint {
            die(b"invalid FASTQ input: can\'t find \'+\' line of sequence %llu\n\x00"
                    as *const u8 as *const libc::c_char,
                n_sequences.wrapping_add(1 as libc::c_int as
                                             libc::c_ulonglong));
        }
        c = in_skip_until(is_eol_arr.as_ptr());
        if c == 256 as libc::c_int as libc::c_uint {
            die(b"truncated FASTQ input: last sequence has no quality\n\x00"
                    as *const u8 as *const libc::c_char);
        }
        loop  { c = in_get_char(); if !is_eol_arr[c as usize] { break ; } }
        if c == 256 as libc::c_int as libc::c_uint {
            die(b"truncated FASTQ input: last sequence has no quality\n\x00"
                    as *const u8 as *const libc::c_char);
        }
        old_len =
            QUAL.uncompressed_size.wrapping_add(qual.length as
                                                    libc::c_ulonglong);
        str_append_char(&mut qual, c as libc::c_uchar);
        loop  {
            c = in_get_until(is_unexpected_qual_arr.as_ptr(), &mut qual);
            if !(c != 256 as libc::c_int as libc::c_uint) { break ; }
            if is_eol_arr[c as usize] { break ; }
            if !is_space_arr[c as usize] {
                unexpected_quality_char(c);
                str_append_char(&mut qual, unexpected_qual_char_replacement);
            }
        }
        let mut qual_length: libc::c_ulonglong =
            QUAL.uncompressed_size.wrapping_add(qual.length as
                                                    libc::c_ulonglong).wrapping_sub(old_len);
        if qual_length != read_length {
            die(b"quality length of sequence %llu (%llu) doesn\'t match sequence length (%llu)\n\x00"
                    as *const u8 as *const libc::c_char,
                n_sequences.wrapping_add(1 as libc::c_int as
                                             libc::c_ulonglong), qual_length,
                read_length);
        }
        add_length(read_length as size_t);
        n_sequences = n_sequences.wrapping_add(1);
        loop  { c = in_get_char(); if !is_eol_arr[c as usize] { break ; } }
        if c == 256 as libc::c_int as libc::c_uint { break ; }
        if c != '@' as i32 as libc::c_uint {
            die(b"invalid FASTQ input: Can\'t find \'@\' after sequence %llu\n\x00"
                    as *const u8 as *const libc::c_char, n_sequences);
        }
    };
}
unsafe extern "C" fn confirm_input_format() {
    let mut last_c: libc::c_uint = '\n' as i32 as libc::c_uint;
    let mut c: libc::c_uint = 0;
    loop  {
        c = in_get_char();
        if !(c != 256 as libc::c_int as libc::c_uint &&
                 is_space_arr[c as usize] as libc::c_int != 0) {
            break ;
        }
        last_c = c
    }
    if c == 256 as libc::c_int as libc::c_uint { return }
    if c == '>' as i32 as libc::c_uint &&
           is_eol_arr[last_c as usize] as libc::c_int != 0 {
        in_format_from_input = in_format_fasta as libc::c_int
    } else if c == '@' as i32 as libc::c_uint &&
                  is_eol_arr[last_c as usize] as libc::c_int != 0 {
        in_format_from_input = in_format_fastq as libc::c_int
    } else if c == '>' as i32 as libc::c_uint ||
                  c == '@' as i32 as libc::c_uint {
        die(b"invalid input - first \'%c\' is not at the beginning of the line\n\x00"
                as *const u8 as *const libc::c_char,
            c as libc::c_uchar as libc::c_int);
    } else {
        die(b"input data is in unknown format - first non-space character is neither \'>\' nor \'@\'\n\x00"
                as *const u8 as *const libc::c_char);
    }
    if in_format_from_command_line != in_format_unknown as libc::c_int &&
           in_format_from_command_line != in_format_from_input {
        die(b"input format is different from format specified in the command line\n\x00"
                as *const u8 as *const libc::c_char);
    }
    if in_format_from_extension != in_format_unknown as libc::c_int &&
           in_format_from_extension != in_format_from_input {
        warn(b"input file extension does not match its actual format\n\x00" as
                 *const u8 as *const libc::c_char);
    }
    if in_format_from_extension != in_format_unknown as libc::c_int &&
           in_format_from_command_line != in_format_unknown as libc::c_int &&
           in_format_from_extension != in_format_from_command_line {
        warn(b"input file extension does not match format specified in the command line\n\x00"
                 as *const u8 as *const libc::c_char);
    };
}
unsafe extern "C" fn process() {
    // If input format is unknown at this point, it indicates empty input.
    if in_format_from_input == in_format_unknown as libc::c_int { return }
    name.data =
        malloc_or_die((1 as
                           libc::c_ulonglong).wrapping_mul(1000 as libc::c_int
                                                               as
                                                               libc::c_ulonglong).wrapping_mul(1000
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_ulonglong)
                          as size_t) as *mut libc::c_uchar;
    comment.data =
        malloc_or_die((1 as
                           libc::c_ulonglong).wrapping_mul(1000 as libc::c_int
                                                               as
                                                               libc::c_ulonglong).wrapping_mul(1000
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_ulonglong)
                          as size_t) as *mut libc::c_uchar;
    seq.data =
        malloc_or_die((1 as
                           libc::c_ulonglong).wrapping_mul(1000 as libc::c_int
                                                               as
                                                               libc::c_ulonglong).wrapping_mul(1000
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_ulonglong)
                          as size_t) as *mut libc::c_uchar;
    seq.writer =
        if no_mask as libc::c_int != 0 {
            if in_seq_type < seq_type_protein as libc::c_int {
                Some(seq_writer_nonmasked_4bit as
                         unsafe extern "C" fn(_: *mut libc::c_uchar,
                                              _: size_t) -> ())
            } else {
                Some(seq_writer_nonmasked_text as
                         unsafe extern "C" fn(_: *mut libc::c_uchar,
                                              _: size_t) -> ())
            }
        } else if in_seq_type < seq_type_protein as libc::c_int {
            Some(seq_writer_masked_4bit as
                     unsafe extern "C" fn(_: *mut libc::c_uchar, _: size_t)
                         -> ())
        } else {
            Some(seq_writer_masked_text as
                     unsafe extern "C" fn(_: *mut libc::c_uchar, _: size_t)
                         -> ())
        };
    if in_format_from_input == in_format_fasta as libc::c_int {
        if assume_well_formed_input {
            process_well_formed_fasta();
        } else { process_non_well_formed_fasta(); }
    } else if in_format_from_input == in_format_fastq as libc::c_int {
        qual.data =
            malloc_or_die((1 as
                               libc::c_ulonglong).wrapping_mul(1000 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ulonglong).wrapping_mul(1000
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulonglong)
                              as size_t) as *mut libc::c_uchar;
        if assume_well_formed_input {
            process_well_formed_fastq();
        } else { process_non_well_formed_fastq(); }
        if qual.length != 0 as libc::c_int as libc::c_ulong {
            qual.writer.expect("non-null function pointer")(qual.data,
                                                            qual.length);
            qual.length = 0 as libc::c_int as size_t
        }
    }
    if name.length != 0 as libc::c_int as libc::c_ulong {
        name.writer.expect("non-null function pointer")(name.data,
                                                        name.length);
        name.length = 0 as libc::c_int as size_t
    }
    if comment.length != 0 as libc::c_int as libc::c_ulong {
        comment.writer.expect("non-null function pointer")(comment.data,
                                                           comment.length);
        comment.length = 0 as libc::c_int as size_t
    }
    if seq.length != 0 as libc::c_int as libc::c_ulong {
        seq.writer.expect("non-null function pointer")(seq.data, seq.length);
        seq.length = 0 as libc::c_int as size_t
    };
}
unsafe extern "C" fn done() {
    compressor_done(&mut IDS);
    compressor_done(&mut COMM);
    compressor_done(&mut LEN);
    compressor_done(&mut MASK);
    compressor_done(&mut SEQ);
    compressor_done(&mut QUAL);
    if !name.data.is_null() {
        free(name.data as *mut libc::c_void);
        name.data = 0 as *mut libc::c_uchar
    }
    if !comment.data.is_null() {
        free(comment.data as *mut libc::c_void);
        comment.data = 0 as *mut libc::c_uchar
    }
    if !seq.data.is_null() {
        free(seq.data as *mut libc::c_void);
        seq.data = 0 as *mut libc::c_uchar
    }
    if !qual.data.is_null() {
        free(qual.data as *mut libc::c_void);
        qual.data = 0 as *mut libc::c_uchar
    }
    if !in_buffer.is_null() {
        free(in_buffer as *mut libc::c_void);
        in_buffer = 0 as *mut libc::c_uchar
    }
    if !out_4bit_buffer.is_null() {
        free(out_4bit_buffer as *mut libc::c_void);
        out_4bit_buffer = 0 as *mut libc::c_uchar
    }
    if !file_copy_buffer.is_null() {
        free(file_copy_buffer as *mut libc::c_void);
        file_copy_buffer = 0 as *mut libc::c_uchar
    }
    if !length_units.is_null() {
        free(length_units as *mut libc::c_void);
        length_units = 0 as *mut libc::c_uint
    }
    if !mask_units.is_null() {
        free(mask_units as *mut libc::c_void);
        mask_units = 0 as *mut libc::c_uchar
    }
    close_output_file();
    close_input_file();
    if !success && created_output_file as libc::c_int != 0 {
        if remove(out_file_path) != 0 as libc::c_int {
            err(b"can\'t remove incomplete output file \"%s\"\n\x00" as
                    *const u8 as *const libc::c_char, out_file_path);
        }
    }
    if !out_file_path_auto.is_null() {
        free(out_file_path_auto as *mut libc::c_void);
        out_file_path_auto = 0 as *mut libc::c_char
    }
    if !temp_prefix.is_null() {
        free(temp_prefix as *mut libc::c_void);
        temp_prefix = 0 as *mut libc::c_char
    };
}
unsafe extern "C" fn set_input_file_path(mut new_path: *mut libc::c_char) {
    if !in_file_path.is_null() {
        die(b"can compress only one file at a time\n\x00" as *const u8 as
                *const libc::c_char);
    }
    if *new_path as libc::c_int == '\u{0}' as i32 {
        die(b"empty input file name\n\x00" as *const u8 as
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
unsafe extern "C" fn set_temp_dir(mut new_temp_dir: *mut libc::c_char) {
    if !temp_dir.is_null() {
        die(b"double --temp-dir parameter\n\x00" as *const u8 as
                *const libc::c_char);
    }
    if *new_temp_dir as libc::c_int == '\u{0}' as i32 {
        die(b"empty --temp-dir parameter\n\x00" as *const u8 as
                *const libc::c_char);
    }
    temp_dir = new_temp_dir;
}
unsafe extern "C" fn set_dataset_name(mut new_name: *mut libc::c_char) {
    if !dataset_name.is_null() {
        die(b"double --name parameter\n\x00" as *const u8 as
                *const libc::c_char);
    }
    if *new_name as libc::c_int == '\u{0}' as i32 {
        die(b"empty --name parameter\n\x00" as *const u8 as
                *const libc::c_char);
    }
    if string_has_characters_unsafe_in_file_names(new_name) {
        die(b"--name \"%s\" - contains characters unsafe in file names\n\x00"
                as *const u8 as *const libc::c_char, new_name);
    }
    dataset_name = new_name;
}
unsafe extern "C" fn set_dataset_title(mut new_title: *mut libc::c_char) {
    if !dataset_title.is_null() {
        die(b"double --title parameter\n\x00" as *const u8 as
                *const libc::c_char);
    }
    if *new_title as libc::c_int == '\u{0}' as i32 {
        die(b"empty --title parameter\n\x00" as *const u8 as
                *const libc::c_char);
    }
    dataset_title = new_title;
    store_title = 1 as libc::c_int != 0;
}
unsafe extern "C" fn set_compression_level(mut str: *mut libc::c_char) {
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut a: libc::c_long = strtol(str, &mut end, 10 as libc::c_int);
    let mut min_level: libc::c_long = ZSTD_minCLevel() as libc::c_long;
    let mut max_level: libc::c_long = ZSTD_maxCLevel() as libc::c_long;
    if a < min_level || a > max_level || *end as libc::c_int != '\u{0}' as i32
       {
        die(b"invalid value of --level, should be from %ld to %ld\n\x00" as
                *const u8 as *const libc::c_char, min_level, max_level);
    }
    compression_level = a as libc::c_int;
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
unsafe extern "C" fn parse_input_format(mut str: *const libc::c_char)
 -> libc::c_int {
    if strcasecmp(str, b"fasta\x00" as *const u8 as *const libc::c_char) == 0
           ||
           strcasecmp(str, b"fa\x00" as *const u8 as *const libc::c_char) == 0
           ||
           strcasecmp(str, b"fna\x00" as *const u8 as *const libc::c_char) ==
               0 {
        return in_format_fasta as libc::c_int
    }
    if strcasecmp(str, b"fastq\x00" as *const u8 as *const libc::c_char) == 0
           ||
           strcasecmp(str, b"fq\x00" as *const u8 as *const libc::c_char) == 0
       {
        return in_format_fastq as libc::c_int
    }
    return in_format_unknown as libc::c_int;
}
unsafe extern "C" fn set_input_format_from_command_line(mut new_format:
                                                            *const libc::c_char) {
    if in_format_from_command_line != in_format_unknown as libc::c_int {
        die(b"input format specified more than once\n\x00" as *const u8 as
                *const libc::c_char);
    }
    in_format_from_command_line = parse_input_format(new_format);
    if in_format_from_command_line == in_format_unknown as libc::c_int {
        die(b"unknown input format specified: \"%s\"\n\x00" as *const u8 as
                *const libc::c_char, new_format);
    };
}
unsafe extern "C" fn detect_input_format_from_input_file_extension() {
    if !in_file_path.is_null() {
        let mut ext: *mut libc::c_char =
            in_file_path.offset(strlen(in_file_path) as isize);
        while ext > in_file_path &&
                  *ext.offset(-(1 as libc::c_int as isize)) as libc::c_int !=
                      '/' as i32 &&
                  *ext.offset(-(1 as libc::c_int as isize)) as libc::c_int !=
                      '\\' as i32 &&
                  *ext.offset(-(1 as libc::c_int as isize)) as libc::c_int !=
                      '.' as i32 {
            ext = ext.offset(-1)
        }
        if ext > in_file_path &&
               *ext.offset(-(1 as libc::c_int as isize)) as libc::c_int ==
                   '.' as i32 {
            in_format_from_extension = parse_input_format(ext)
        }
    };
}
unsafe extern "C" fn detect_temp_directory() {
    if temp_dir.is_null() {
        temp_dir = getenv(b"TMPDIR\x00" as *const u8 as *const libc::c_char)
    }
    if temp_dir.is_null() {
        temp_dir = getenv(b"TMP\x00" as *const u8 as *const libc::c_char)
    }
    if temp_dir.is_null() {
        die(b"temporary directory is not specified.\nPlease either set TMPDIR or TMP environment variable, or add \'--temp-dir DIR\' to command line.\n\x00"
                as *const u8 as *const libc::c_char);
    }
    if verbose {
        msg(b"Using temporary directory \"%s\"\n\x00" as *const u8 as
                *const libc::c_char, temp_dir);
    };
}
unsafe extern "C" fn show_version() {
    msg(b"ennaf - NAF compressor, version 1.2.0, 2020-09-01\nCopyright (c) 2018-2020 Kirill Kryukov\n\x00"
            as *const u8 as *const libc::c_char);
    if verbose {
        msg(b"Built with zstd 1.4.5, using runtime zstd %s\n\x00" as *const u8
                as *const libc::c_char, ZSTD_versionString());
    };
}
unsafe extern "C" fn show_help() {
    let mut min_level: libc::c_int = ZSTD_minCLevel();
    let mut max_level: libc::c_int = ZSTD_maxCLevel();
    msg(b"Usage: ennaf [OPTIONS] [infile]\nOptions:\n  -o FILE            - Write compressed output to FILE\n  -c                 - Write to standard output\n  -#, --level #      - Use compression level # (from %d to %d, default: 1)\n  --temp-dir DIR     - Use DIR as temporary directory\n  --name NAME        - Use NAME as prefix for temporary files\n  --title TITLE      - Store TITLE as dataset title\n  --fasta            - Input is in FASTA format\n  --fastq            - Input is in FASTQ format\n  --dna              - Input sequence is DNA (default)\n  --rna              - Input sequence is RNA\n  --protein          - Input sequence is protein\n  --text             - Input sequence is text\n  --strict           - Fail on unexpected input characters\n  --line-length N    - Override line length to N\n  --verbose          - Verbose mode\n  --keep-temp-files  - Keep temporary files\n  --no-mask          - Don\'t store mask\n  -h, --help         - Show help\n  -V, --version      - Show version\n\x00"
            as *const u8 as *const libc::c_char, min_level, max_level);
}
unsafe extern "C" fn parse_command_line(mut argc: libc::c_int,
                                        mut argv: *mut *mut libc::c_char) {
    let mut print_version: bool = 0 as libc::c_int != 0;
    let mut current_block_46: u64;
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < argc {
        if *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize) as
               libc::c_int == '-' as i32 {
            if *(*argv.offset(i as isize)).offset(1 as libc::c_int as isize)
                   as libc::c_int == '-' as i32 {
                if i < argc - 1 as libc::c_int {
                    if strcmp(*argv.offset(i as isize),
                              b"--temp-dir\x00" as *const u8 as
                                  *const libc::c_char) == 0 {
                        i += 1;
                        set_temp_dir(*argv.offset(i as isize));
                        current_block_46 = 16668937799742929182;
                    } else if strcmp(*argv.offset(i as isize),
                                     b"--name\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                        i += 1;
                        set_dataset_name(*argv.offset(i as isize));
                        current_block_46 = 16668937799742929182;
                    } else if strcmp(*argv.offset(i as isize),
                                     b"--title\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                        i += 1;
                        set_dataset_title(*argv.offset(i as isize));
                        current_block_46 = 16668937799742929182;
                    } else if strcmp(*argv.offset(i as isize),
                                     b"--level\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                        i += 1;
                        set_compression_level(*argv.offset(i as isize));
                        current_block_46 = 16668937799742929182;
                    } else if strcmp(*argv.offset(i as isize),
                                     b"--line-length\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                        i += 1;
                        set_line_length(*argv.offset(i as isize));
                        current_block_46 = 16668937799742929182;
                    } else if strcmp(*argv.offset(i as isize),
                                     b"--out\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                        i += 1;
                        set_output_file_path(*argv.offset(i as isize));
                        current_block_46 = 16668937799742929182;
                    } else if strcmp(*argv.offset(i as isize),
                                     b"--in\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                        i += 1;
                        set_input_file_path(*argv.offset(i as isize));
                        current_block_46 = 16668937799742929182;
                    } else if strcmp(*argv.offset(i as isize),
                                     b"--in-format\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                        i += 1;
                        set_input_format_from_command_line(*argv.offset(i as
                                                                            isize));
                        current_block_46 = 16668937799742929182;
                    } else { current_block_46 = 18377268871191777778; }
                } else { current_block_46 = 18377268871191777778; }
                match current_block_46 {
                    16668937799742929182 => { }
                    _ => {
                        if strcmp(*argv.offset(i as isize),
                                  b"--help\x00" as *const u8 as
                                      *const libc::c_char) == 0 {
                            show_help();
                            exit(0 as libc::c_int);
                        }
                        if strcmp(*argv.offset(i as isize),
                                  b"--version\x00" as *const u8 as
                                      *const libc::c_char) == 0 {
                            print_version = 1 as libc::c_int != 0;
                            current_block_46 = 16668937799742929182;
                        } else if strcmp(*argv.offset(i as isize),
                                         b"--verbose\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            verbose = 1 as libc::c_int != 0;
                            current_block_46 = 16668937799742929182;
                        } else if strcmp(*argv.offset(i as isize),
                                         b"--binary-stderr\x00" as *const u8
                                             as *const libc::c_char) == 0 {
                            if !binary_stderr {
                                binary_stderr = 1 as libc::c_int != 0;
                                change_stderr_to_binary();
                            }
                            current_block_46 = 16668937799742929182;
                        } else if strcmp(*argv.offset(i as isize),
                                         b"--keep-temp-files\x00" as *const u8
                                             as *const libc::c_char) == 0 {
                            keep_temp_files = 1 as libc::c_int != 0;
                            current_block_46 = 16668937799742929182;
                        } else if strcmp(*argv.offset(i as isize),
                                         b"--no-mask\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            no_mask = 1 as libc::c_int != 0;
                            current_block_46 = 16668937799742929182;
                        } else if strcmp(*argv.offset(i as isize),
                                         b"--fasta\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            set_input_format_from_command_line(b"fasta\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
                            current_block_46 = 16668937799742929182;
                        } else if strcmp(*argv.offset(i as isize),
                                         b"--fastq\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            set_input_format_from_command_line(b"fastq\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
                            current_block_46 = 16668937799742929182;
                        } else if strcmp(*argv.offset(i as isize),
                                         b"--dna\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            in_seq_type = seq_type_dna as libc::c_int;
                            current_block_46 = 16668937799742929182;
                        } else if strcmp(*argv.offset(i as isize),
                                         b"--rna\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            in_seq_type = seq_type_rna as libc::c_int;
                            current_block_46 = 16668937799742929182;
                        } else if strcmp(*argv.offset(i as isize),
                                         b"--protein\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            in_seq_type = seq_type_protein as libc::c_int;
                            current_block_46 = 16668937799742929182;
                        } else if strcmp(*argv.offset(i as isize),
                                         b"--text\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            in_seq_type = seq_type_text as libc::c_int;
                            current_block_46 = 16668937799742929182;
                        } else if strcmp(*argv.offset(i as isize),
                                         b"--well-formed\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            assume_well_formed_input = 1 as libc::c_int != 0;
                            current_block_46 = 16668937799742929182;
                        } else if strcmp(*argv.offset(i as isize),
                                         b"--strict\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            abort_on_unexpected_code = 1 as libc::c_int != 0;
                            current_block_46 = 16668937799742929182;
                        } else { current_block_46 = 6367734732029634840; }
                    }
                }
            } else { current_block_46 = 6367734732029634840; }
            match current_block_46 {
                16668937799742929182 => { }
                _ => {
                    if i < argc - 1 as libc::c_int {
                        if strcmp(*argv.offset(i as isize),
                                  b"-o\x00" as *const u8 as
                                      *const libc::c_char) == 0 {
                            i += 1;
                            set_output_file_path(*argv.offset(i as isize));
                            current_block_46 = 16668937799742929182;
                        } else { current_block_46 = 479107131381816815; }
                    } else { current_block_46 = 479107131381816815; }
                    match current_block_46 {
                        16668937799742929182 => { }
                        _ => {
                            if strcmp(*argv.offset(i as isize),
                                      b"-c\x00" as *const u8 as
                                          *const libc::c_char) == 0 {
                                force_stdout = 1 as libc::c_int != 0
                            } else if *(*argv.offset(i as
                                                         isize)).offset(1 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)
                                          as libc::c_int >= '0' as i32 &&
                                          *(*argv.offset(i as
                                                             isize)).offset(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize)
                                              as libc::c_int <= '9' as i32 {
                                set_compression_level((*argv.offset(i as
                                                                        isize)).offset(1
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           isize));
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
        die(b"\'-c\' and \'-o\' can\'t be used together\n\x00" as *const u8 as
                *const libc::c_char);
    }
    if assume_well_formed_input as libc::c_int != 0 &&
           abort_on_unexpected_code as libc::c_int != 0 {
        die(b"\'--well-formed\' and \'--strict\' can\'t be used together\n\x00"
                as *const u8 as *const libc::c_char);
    };
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    atexit(Some(done as unsafe extern "C" fn() -> ()));
    init_encoders();
    parse_command_line(argc, argv);
    if in_file_path.is_null() && isatty(fileno(stdin)) != 0 {
        err(b"no input specified, use \"ennaf -h\" for help\n\x00" as
                *const u8 as *const libc::c_char);
        exit(0 as libc::c_int);
    }
    if no_mask as libc::c_int != 0 ||
           in_seq_type >= seq_type_protein as libc::c_int {
        store_mask = 0 as libc::c_int != 0
    }
    if in_seq_type == seq_type_dna as libc::c_int {
        is_unexpected_arr = is_unexpected_dna_arr.as_mut_ptr();
        in_seq_type_name = b"DNA\x00" as *const u8 as *const libc::c_char;
        unexpected_seq_char_replacement = 'N' as i32 as libc::c_uchar
    }
    if in_seq_type == seq_type_rna as libc::c_int {
        is_unexpected_arr = is_unexpected_rna_arr.as_mut_ptr();
        in_seq_type_name = b"RNA\x00" as *const u8 as *const libc::c_char;
        unexpected_seq_char_replacement = 'N' as i32 as libc::c_uchar
    } else if in_seq_type == seq_type_protein as libc::c_int {
        is_unexpected_arr = is_unexpected_protein_arr.as_mut_ptr();
        in_seq_type_name = b"protein\x00" as *const u8 as *const libc::c_char;
        unexpected_seq_char_replacement = 'X' as i32 as libc::c_uchar
    } else if in_seq_type == seq_type_text as libc::c_int {
        is_unexpected_arr = is_unexpected_text_arr.as_mut_ptr();
        in_seq_type_name = b"text\x00" as *const u8 as *const libc::c_char;
        unexpected_seq_char_replacement = '?' as i32 as libc::c_uchar
    }
    detect_temp_directory();
    detect_input_format_from_input_file_extension();
    open_input_file();
    confirm_input_format();
    store_qual = in_format_from_input == in_format_fastq as libc::c_int;
    if in_seq_type == seq_type_text as libc::c_int &&
           in_format_from_input == in_format_fasta as libc::c_int {
        *is_unexpected_arr.offset('>' as i32 as isize) = 1 as libc::c_int != 0
    }
    if !force_stdout && out_file_path.is_null() && isatty(fileno(stdout)) != 0
       {
        if in_file_path.is_null() {
            die(b"output file is not specified\n\x00" as *const u8 as
                    *const libc::c_char);
        } else {
            let mut len: size_t =
                strlen(in_file_path).wrapping_add(5 as libc::c_int as
                                                      libc::c_ulong);
            out_file_path_auto = malloc_or_die(len) as *mut libc::c_char;
            snprintf(out_file_path_auto, len,
                     b"%s.naf\x00" as *const u8 as *const libc::c_char,
                     in_file_path);
            out_file_path = out_file_path_auto
        }
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
    make_temp_prefix();
    compressor_init(&mut IDS, b"ids\x00" as *const u8 as *const libc::c_char);
    compressor_init(&mut COMM,
                    b"comments\x00" as *const u8 as *const libc::c_char);
    compressor_init(&mut LEN,
                    b"lengths\x00" as *const u8 as *const libc::c_char);
    if store_mask {
        compressor_init(&mut MASK,
                        b"mask\x00" as *const u8 as *const libc::c_char);
    }
    compressor_init(&mut SEQ,
                    b"sequence\x00" as *const u8 as *const libc::c_char);
    if store_qual {
        compressor_init(&mut QUAL,
                        b"quality\x00" as *const u8 as *const libc::c_char);
    }
    process();
    close_input_file();
    if mask_len > 0 as libc::c_int as libc::c_ulonglong {
        add_mask(mask_len);
    }
    if length_unit_index > 0 as libc::c_int as libc::c_uint {
        compress(&mut LEN, length_units as *const libc::c_void,
                 (::std::mem::size_of::<libc::c_uint>() as
                      libc::c_ulong).wrapping_mul(length_unit_index as
                                                      libc::c_ulong));
        length_unit_index = 0 as libc::c_int as libc::c_uint
    }
    if mask_units_pos > mask_units {
        compress(&mut MASK, mask_units as *const libc::c_void,
                 mask_units_pos.wrapping_offset_from(mask_units) as
                     libc::c_long as size_t);
        mask_units_pos = mask_units
    }
    if parity { out_4bit_pos = out_4bit_pos.offset(1) }
    if out_4bit_pos > out_4bit_buffer {
        compress(&mut SEQ, out_4bit_buffer as *const libc::c_void,
                 out_4bit_pos.wrapping_offset_from(out_4bit_buffer) as
                     libc::c_long as size_t);
    }
    compressor_end_stream(&mut IDS);
    compressor_end_stream(&mut COMM);
    compressor_end_stream(&mut LEN);
    compressor_end_stream(&mut MASK);
    compressor_end_stream(&mut SEQ);
    compressor_end_stream(&mut QUAL);
    fwrite_or_die(naf_magic_number.as_ptr() as *const libc::c_void,
                  1 as libc::c_int as size_t, 3 as libc::c_int as size_t,
                  OUT);
    // Deprecated, undocumented.
    // In case of DNA input, write NAFv1 format.
    // Otherwise write NAFv2 where we can store sequence type.
    if in_seq_type == seq_type_dna as libc::c_int {
        fputc_or_die(1 as libc::c_int, OUT);
    } else {
        fputc_or_die(2 as libc::c_int, OUT);
        fputc_or_die(in_seq_type, OUT);
    }
    fputc_or_die((0 as libc::c_int) << 7 as libc::c_int |
                     (store_title as libc::c_int) << 6 as libc::c_int |
                     (1 as libc::c_int) << 5 as libc::c_int |
                     (1 as libc::c_int) << 4 as libc::c_int |
                     (1 as libc::c_int) << 3 as libc::c_int |
                     (store_mask as libc::c_int) << 2 as libc::c_int |
                     (1 as libc::c_int) << 1 as libc::c_int |
                     store_qual as libc::c_int, OUT);
    fputc_or_die(' ' as i32, OUT);
    let mut out_line_length: libc::c_ulonglong =
        if line_length_is_specified as libc::c_int != 0 {
            requested_line_length
        } else { longest_line_length };
    if verbose {
        msg(b"Output line length: %llu\n\x00" as *const u8 as
                *const libc::c_char, out_line_length);
    }
    write_variable_length_encoded_number(OUT, out_line_length);
    write_variable_length_encoded_number(OUT, n_sequences);
    if store_title {
        let mut title_length: size_t = strlen(dataset_title);
        write_variable_length_encoded_number(OUT,
                                             title_length as
                                                 libc::c_ulonglong);
        fwrite_or_die(dataset_title as *const libc::c_void,
                      1 as libc::c_int as size_t, title_length, OUT);
    }
    write_variable_length_encoded_number(OUT, IDS.uncompressed_size);
    write_compressed_data(OUT, &mut IDS);
    write_variable_length_encoded_number(OUT, COMM.uncompressed_size);
    write_compressed_data(OUT, &mut COMM);
    write_variable_length_encoded_number(OUT, LEN.uncompressed_size);
    write_compressed_data(OUT, &mut LEN);
    if store_mask {
        write_variable_length_encoded_number(OUT, MASK.uncompressed_size);
        write_compressed_data(OUT, &mut MASK);
    }
    write_variable_length_encoded_number(OUT, seq_size_original);
    write_compressed_data(OUT, &mut SEQ);
    if store_qual {
        write_variable_length_encoded_number(OUT, QUAL.uncompressed_size);
        write_compressed_data(OUT, &mut QUAL);
    }
    if !out_file_path.is_null() && have_input_stat as libc::c_int != 0 {
        close_output_file_and_set_stat();
    } else { close_output_file(); }
    if !assume_well_formed_input { report_unexpected_input_char_stats(); }
    if verbose {
        msg(b"Processed %llu sequences\n\x00" as *const u8 as
                *const libc::c_char, n_sequences);
    }
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
