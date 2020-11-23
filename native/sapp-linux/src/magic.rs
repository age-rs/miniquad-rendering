use ::libc;
pub type gbm_device = ();
pub type gbm_bo = ();
pub type gbm_surface = ();

extern "C" {
    #[no_mangle]
    fn select(__nfds: libc::c_int, __readfds: *mut fd_set,
              __writefds: *mut fd_set, __exceptfds: *mut fd_set,
              __timeout: *mut timeval) -> libc::c_int;
    #[no_mangle]
    fn drmHandleEvent(fd: libc::c_int, evctx: drmEventContextPtr)
     -> libc::c_int;
    #[no_mangle]
    fn drmModeFreeResources(ptr: drmModeResPtr);
    #[no_mangle]
    fn drmModeFreeCrtc(ptr: drmModeCrtcPtr);
    #[no_mangle]
    fn drmModeFreeConnector(ptr: drmModeConnectorPtr);
    #[no_mangle]
    fn drmModeFreeEncoder(ptr: drmModeEncoderPtr);
    #[no_mangle]
    fn drmModeGetResources(fd: libc::c_int) -> drmModeResPtr;
    #[no_mangle]
    fn drmModeAddFB(fd: libc::c_int, width: uint32_t, height: uint32_t,
                    depth: uint8_t, bpp: uint8_t, pitch_0: uint32_t,
                    bo_handle: uint32_t, buf_id: *mut uint32_t)
     -> libc::c_int;
    #[no_mangle]
    fn drmModeRmFB(fd: libc::c_int, bufferId: uint32_t) -> libc::c_int;
    #[no_mangle]
    fn drmModeGetCrtc(fd: libc::c_int, crtcId: uint32_t) -> drmModeCrtcPtr;
    #[no_mangle]
    fn drmModeSetCrtc(fd: libc::c_int, crtcId: uint32_t, bufferId: uint32_t,
                      x: uint32_t, y: uint32_t, connectors: *mut uint32_t,
                      count_0: libc::c_int, mode: drmModeModeInfoPtr)
     -> libc::c_int;
    #[no_mangle]
    fn drmModeGetEncoder(fd: libc::c_int, encoder_id: uint32_t)
     -> drmModeEncoderPtr;
    #[no_mangle]
    fn drmModeGetConnector(fd: libc::c_int, connectorId: uint32_t)
     -> drmModeConnectorPtr;
    #[no_mangle]
    fn drmModePageFlip(fd: libc::c_int, crtc_id: uint32_t, fb_id: uint32_t,
                       flags: uint32_t, user_data: *mut libc::c_void)
     -> libc::c_int;
    #[no_mangle]
    fn gbm_device_destroy(gbm: *mut gbm_device);
    #[no_mangle]
    fn gbm_create_device(fd: libc::c_int) -> *mut gbm_device;
    #[no_mangle]
    fn gbm_bo_get_width(bo_0: *mut gbm_bo) -> uint32_t;
    #[no_mangle]
    fn gbm_bo_get_height(bo_0: *mut gbm_bo) -> uint32_t;
    #[no_mangle]
    fn gbm_bo_get_stride(bo_0: *mut gbm_bo) -> uint32_t;
    #[no_mangle]
    fn gbm_bo_get_device(bo_0: *mut gbm_bo) -> *mut gbm_device;
    #[no_mangle]
    fn gbm_bo_get_handle(bo_0: *mut gbm_bo) -> gbm_bo_handle;
    #[no_mangle]
    fn gbm_bo_set_user_data(bo_0: *mut gbm_bo, data: *mut libc::c_void,
                            destroy_user_data:
                                Option<unsafe extern "C" fn(_: *mut gbm_bo,
                                                            _:
                                                                *mut libc::c_void)
                                           -> ()>);
    #[no_mangle]
    fn gbm_bo_get_user_data(bo_0: *mut gbm_bo) -> *mut libc::c_void;
    #[no_mangle]
    fn gbm_surface_create(gbm: *mut gbm_device, width: uint32_t,
                          height: uint32_t, format: uint32_t, flags: uint32_t)
     -> *mut gbm_surface;
    #[no_mangle]
    fn gbm_surface_lock_front_buffer(surface: *mut gbm_surface)
     -> *mut gbm_bo;
    #[no_mangle]
    fn gbm_surface_release_buffer(surface: *mut gbm_surface,
                                  bo_0: *mut gbm_bo);
    #[no_mangle]
    fn gbm_surface_destroy(surface: *mut gbm_surface);
    #[no_mangle]
    fn eglChooseConfig(dpy: EGLDisplay, attrib_list: *const EGLint,
                       configs_0: *mut EGLConfig, config_size: EGLint,
                       num_config_0: *mut EGLint) -> EGLBoolean;
    #[no_mangle]
    fn eglCreateContext(dpy: EGLDisplay, config_0: EGLConfig,
                        share_context: EGLContext, attrib_list: *const EGLint)
     -> EGLContext;
    #[no_mangle]
    fn eglCreateWindowSurface(dpy: EGLDisplay, config_0: EGLConfig,
                              win: EGLNativeWindowType,
                              attrib_list: *const EGLint) -> EGLSurface;
    #[no_mangle]
    fn eglDestroyContext(dpy: EGLDisplay, ctx: EGLContext) -> EGLBoolean;
    #[no_mangle]
    fn eglDestroySurface(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean;
    #[no_mangle]
    fn eglGetConfigAttrib(dpy: EGLDisplay, config_0: EGLConfig,
                          attribute: EGLint, value: *mut EGLint)
     -> EGLBoolean;
    #[no_mangle]
    fn eglGetConfigs(dpy: EGLDisplay, configs_0: *mut EGLConfig,
                     config_size: EGLint, num_config_0: *mut EGLint)
     -> EGLBoolean;
    #[no_mangle]
    fn eglGetProcAddress(procname: *const libc::c_char)
     -> __eglMustCastToProperFunctionPointerType;
    #[no_mangle]
    fn eglInitialize(dpy: EGLDisplay, major: *mut EGLint, minor: *mut EGLint)
     -> EGLBoolean;
    #[no_mangle]
    fn eglMakeCurrent(dpy: EGLDisplay, draw: EGLSurface, read: EGLSurface,
                      ctx: EGLContext) -> EGLBoolean;
    #[no_mangle]
    fn eglQueryString(dpy: EGLDisplay, name: EGLint) -> *const libc::c_char;
    #[no_mangle]
    fn eglSwapBuffers(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean;
    #[no_mangle]
    fn eglTerminate(dpy: EGLDisplay) -> EGLBoolean;
    #[no_mangle]
    fn eglBindAPI(api: EGLenum) -> EGLBoolean;
    #[no_mangle]
    fn glClearColor(red: GLclampf, green: GLclampf, blue: GLclampf,
                    alpha: GLclampf);
    #[no_mangle]
    fn glClear(mask: GLbitfield);
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _drmEventContext {
    pub version: libc::c_int,
    pub vblank_handler: Option<unsafe extern "C" fn(_: libc::c_int,
                                                    _: libc::c_uint,
                                                    _: libc::c_uint,
                                                    _: libc::c_uint,
                                                    _: *mut libc::c_void)
                                   -> ()>,
    pub page_flip_handler: Option<unsafe extern "C" fn(_: libc::c_int,
                                                       _: libc::c_uint,
                                                       _: libc::c_uint,
                                                       _: libc::c_uint,
                                                       _: *mut libc::c_void)
                                      -> ()>,
    pub page_flip_handler2: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _: libc::c_uint,
                                                        _: libc::c_uint,
                                                        _: libc::c_uint,
                                                        _: libc::c_uint,
                                                        _: *mut libc::c_void)
                                       -> ()>,
    pub sequence_handler: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: uint64_t,
                                                      _: uint64_t,
                                                      _: uint64_t) -> ()>,
}
pub type drmEventContext = _drmEventContext;
pub type drmEventContextPtr = *mut _drmEventContext;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _drmModeRes {
    pub count_fbs: libc::c_int,
    pub fbs: *mut uint32_t,
    pub count_crtcs: libc::c_int,
    pub crtcs: *mut uint32_t,
    pub count_connectors: libc::c_int,
    pub connectors: *mut uint32_t,
    pub count_encoders: libc::c_int,
    pub encoders: *mut uint32_t,
    pub min_width: uint32_t,
    pub max_width: uint32_t,
    pub min_height: uint32_t,
    pub max_height: uint32_t,
}
pub type drmModeRes = _drmModeRes;
pub type drmModeResPtr = *mut _drmModeRes;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _drmModeModeInfo {
    pub clock: uint32_t,
    pub hdisplay: uint16_t,
    pub hsync_start: uint16_t,
    pub hsync_end: uint16_t,
    pub htotal: uint16_t,
    pub hskew: uint16_t,
    pub vdisplay: uint16_t,
    pub vsync_start: uint16_t,
    pub vsync_end: uint16_t,
    pub vtotal: uint16_t,
    pub vscan: uint16_t,
    pub vrefresh: uint32_t,
    pub flags: uint32_t,
    pub type_0: uint32_t,
    pub name: [libc::c_char; 32],
}
pub type drmModeModeInfo = _drmModeModeInfo;
pub type drmModeModeInfoPtr = *mut _drmModeModeInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _drmModeCrtc {
    pub crtc_id: uint32_t,
    pub buffer_id: uint32_t,
    pub x: uint32_t,
    pub y: uint32_t,
    pub width: uint32_t,
    pub height: uint32_t,
    pub mode_valid: libc::c_int,
    pub mode: drmModeModeInfo,
    pub gamma_size: libc::c_int,
}
pub type drmModeCrtc = _drmModeCrtc;
pub type drmModeCrtcPtr = *mut _drmModeCrtc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _drmModeEncoder {
    pub encoder_id: uint32_t,
    pub encoder_type: uint32_t,
    pub crtc_id: uint32_t,
    pub possible_crtcs: uint32_t,
    pub possible_clones: uint32_t,
}
pub type drmModeEncoder = _drmModeEncoder;
pub type drmModeEncoderPtr = *mut _drmModeEncoder;
pub type drmModeConnection = libc::c_uint;
pub const DRM_MODE_UNKNOWNCONNECTION: drmModeConnection = 3;
pub const DRM_MODE_DISCONNECTED: drmModeConnection = 2;
pub const DRM_MODE_CONNECTED: drmModeConnection = 1;
pub type drmModeSubPixel = libc::c_uint;
pub const DRM_MODE_SUBPIXEL_NONE: drmModeSubPixel = 6;
pub const DRM_MODE_SUBPIXEL_VERTICAL_BGR: drmModeSubPixel = 5;
pub const DRM_MODE_SUBPIXEL_VERTICAL_RGB: drmModeSubPixel = 4;
pub const DRM_MODE_SUBPIXEL_HORIZONTAL_BGR: drmModeSubPixel = 3;
pub const DRM_MODE_SUBPIXEL_HORIZONTAL_RGB: drmModeSubPixel = 2;
pub const DRM_MODE_SUBPIXEL_UNKNOWN: drmModeSubPixel = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _drmModeConnector {
    pub connector_id: uint32_t,
    pub encoder_id: uint32_t,
    pub connector_type: uint32_t,
    pub connector_type_id: uint32_t,
    pub connection: drmModeConnection,
    pub mmWidth: uint32_t,
    pub mmHeight: uint32_t,
    pub subpixel: drmModeSubPixel,
    pub count_modes: libc::c_int,
    pub modes: drmModeModeInfoPtr,
    pub count_props: libc::c_int,
    pub props: *mut uint32_t,
    pub prop_values: *mut uint64_t,
    pub count_encoders: libc::c_int,
    pub encoders: *mut uint32_t,
}
pub type drmModeConnector = _drmModeConnector;
pub type drmModeConnectorPtr = *mut _drmModeConnector;
#[derive(Copy, Clone)]
#[repr(C)]
pub union gbm_bo_handle {
    pub ptr: *mut libc::c_void,
    pub s32: int32_t,
    pub u32_0: uint32_t,
    pub s64: int64_t,
    pub u64_0: uint64_t,
}
pub type gbm_bo_flags = libc::c_uint;
pub const GBM_BO_USE_LINEAR: gbm_bo_flags = 16;
pub const GBM_BO_USE_WRITE: gbm_bo_flags = 8;
pub const GBM_BO_USE_RENDERING: gbm_bo_flags = 4;
pub const GBM_BO_USE_CURSOR_64X64: gbm_bo_flags = 2;
pub const GBM_BO_USE_CURSOR: gbm_bo_flags = 2;
pub const GBM_BO_USE_SCANOUT: gbm_bo_flags = 1;
pub type khronos_int32_t = int32_t;
pub type EGLNativeWindowType = *mut libc::c_void;
pub type EGLint = khronos_int32_t;
pub type EGLBoolean = libc::c_uint;
pub type EGLDisplay = *mut libc::c_void;
pub type EGLConfig = *mut libc::c_void;
pub type EGLSurface = *mut libc::c_void;
pub type EGLContext = *mut libc::c_void;
pub type __eglMustCastToProperFunctionPointerType
    =
    Option<unsafe extern "C" fn() -> ()>;
pub type EGLenum = libc::c_uint;
pub type GLbitfield = libc::c_uint;
pub type GLclampf = libc::c_float;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct drm_fb {
    pub bo: *mut gbm_bo,
    pub fb_id: uint32_t,
}
pub type PFNEGLGETPLATFORMDISPLAYEXTPROC
    =
    Option<unsafe extern "C" fn(_: EGLenum, _: *mut libc::c_void,
                                _: *const EGLint) -> EGLDisplay>;
//----------------------------------------------------------------------
//--------  Trying to get OpenGL ES screen on RPi4 without X
//--------  based on drm-gbm https://github.com/eyelash/tutorials/blob/master/drm-gbm.c
//--------  and kmscube https://github.com/robclark/kmscube
//--------  pik33@o2.pl
//----------------------------------------------------------------------
// global variables declarations
static mut device: libc::c_int = 0;
static mut resources: *mut drmModeRes =
    0 as *const drmModeRes as *mut drmModeRes;
static mut connector: *mut drmModeConnector =
    0 as *const drmModeConnector as *mut drmModeConnector;
static mut connector_id: uint32_t = 0;
static mut encoder: *mut drmModeEncoder =
    0 as *const drmModeEncoder as *mut drmModeEncoder;
static mut mode_info: drmModeModeInfo =
    drmModeModeInfo{clock: 0,
                    hdisplay: 0,
                    hsync_start: 0,
                    hsync_end: 0,
                    htotal: 0,
                    hskew: 0,
                    vdisplay: 0,
                    vsync_start: 0,
                    vsync_end: 0,
                    vtotal: 0,
                    vscan: 0,
                    vrefresh: 0,
                    flags: 0,
                    type_0: 0,
                    name: [0; 32],};
static mut crtc: *mut drmModeCrtc =
    0 as *const drmModeCrtc as *mut drmModeCrtc;
static mut gbm_device: *mut gbm_device =
    0 as *const gbm_device as *mut gbm_device;
static mut display: EGLDisplay =
    0 as *const libc::c_void as *mut libc::c_void;
static mut context: EGLContext =
    0 as *const libc::c_void as *mut libc::c_void;
static mut gbm_surface: *mut gbm_surface =
    0 as *const gbm_surface as *mut gbm_surface;
static mut egl_surface: EGLSurface =
    0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub static mut config: EGLConfig =
    0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub static mut num_config: EGLint = 0;
#[no_mangle]
pub static mut count: EGLint = 0 as libc::c_int;
#[no_mangle]
pub static mut configs: *mut EGLConfig =
    0 as *const EGLConfig as *mut EGLConfig;
#[no_mangle]
pub static mut config_index: libc::c_int = 0;
#[no_mangle]
pub static mut i: libc::c_int = 0;
static mut previous_bo: *mut gbm_bo = 0 as *const gbm_bo as *mut gbm_bo;
static mut previous_fb: uint32_t = 0;
static mut attributes: [EGLint; 15] =
    [0x3033 as libc::c_int, 0x4 as libc::c_int, 0x3024 as libc::c_int,
     8 as libc::c_int, 0x3023 as libc::c_int, 8 as libc::c_int,
     0x3022 as libc::c_int, 8 as libc::c_int, 0x3021 as libc::c_int,
     0 as libc::c_int, 0x3025 as libc::c_int, 8 as libc::c_int,
     0x3040 as libc::c_int, 0x4 as libc::c_int, 0x3038 as libc::c_int];
static mut context_attribs: [EGLint; 3] =
    [0x3098 as libc::c_int, 2 as libc::c_int, 0x3038 as libc::c_int];
#[no_mangle]
pub static mut bo: *mut gbm_bo = 0 as *const gbm_bo as *mut gbm_bo;
#[no_mangle]
pub static mut handle: uint32_t = 0;
#[no_mangle]
pub static mut pitch: uint32_t = 0;
#[no_mangle]
pub static mut fb: int32_t = 0;
#[no_mangle]
pub static mut modifier: uint64_t = 0;
unsafe extern "C" fn find_connector(mut resources_0: *mut drmModeRes)
 -> *mut drmModeConnector {
    i = 0 as libc::c_int;
    while i < (*resources_0).count_connectors {
        let mut connector_0: *mut drmModeConnector =
            drmModeGetConnector(device,
                                *(*resources_0).connectors.offset(i as
                                                                      isize));
        if (*connector_0).connection as libc::c_uint ==
               DRM_MODE_CONNECTED as libc::c_int as libc::c_uint {
            return connector_0
        }
        drmModeFreeConnector(connector_0);
        i += 1
    }
    return 0 as *mut drmModeConnector;
    // if no connector found
}
unsafe extern "C" fn find_encoder(mut resources_0: *mut drmModeRes,
                                  mut connector_0: *mut drmModeConnector)
 -> *mut drmModeEncoder {
    if (*connector_0).encoder_id != 0 {
        return drmModeGetEncoder(device, (*connector_0).encoder_id)
    }
    return 0 as *mut drmModeEncoder;
    // if no encoder found
}
unsafe extern "C" fn drm_fb_destroy_callback(mut bo_0: *mut gbm_bo,
                                             mut data: *mut libc::c_void) {
    let mut fb_0: *mut drm_fb = data as *mut drm_fb;
    let mut gbm: *mut gbm_device = gbm_bo_get_device(bo_0);
    if (*fb_0).fb_id != 0 { drmModeRmFB(device, (*fb_0).fb_id); }
    free(fb_0 as *mut libc::c_void);
}
unsafe extern "C" fn drm_fb_get_from_bo(mut bo_0: *mut gbm_bo)
 -> *mut drm_fb {
    let mut fb_0: *mut drm_fb = gbm_bo_get_user_data(bo_0) as *mut drm_fb;
    let mut width: uint32_t = 0;
    let mut height: uint32_t = 0;
    let mut stride: uint32_t = 0;
    let mut handle_0: uint32_t = 0;
    let mut ret: libc::c_int = 0;
    if !fb_0.is_null() { return fb_0 }
    fb_0 =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<drm_fb>() as libc::c_ulong) as
            *mut drm_fb;
    (*fb_0).bo = bo_0;
    width = gbm_bo_get_width(bo_0);
    height = gbm_bo_get_height(bo_0);
    stride = gbm_bo_get_stride(bo_0);
    handle_0 = gbm_bo_get_handle(bo_0).u32_0;
    ret =
        drmModeAddFB(device, width, height, 24 as libc::c_int as uint8_t,
                     32 as libc::c_int as uint8_t, stride, handle_0,
                     &mut (*fb_0).fb_id);
    if ret != 0 {
        printf(b"failed to create fb\n\x00" as *const u8 as
                   *const libc::c_char);
        free(fb_0 as *mut libc::c_void);
        return 0 as *mut drm_fb
    }
    gbm_bo_set_user_data(bo_0, fb_0 as *mut libc::c_void,
                         Some(drm_fb_destroy_callback as
                                  unsafe extern "C" fn(_: *mut gbm_bo,
                                                       _: *mut libc::c_void)
                                      -> ()));
    return fb_0;
}
unsafe extern "C" fn page_flip_handler(mut fd: libc::c_int,
                                       mut frame: libc::c_uint,
                                       mut sec: libc::c_uint,
                                       mut usec: libc::c_uint,
                                       mut data: *mut libc::c_void) {
    let mut waiting_for_flip: *mut libc::c_int = data as *mut libc::c_int;
    *waiting_for_flip = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn swap_buffers() {
    let mut fb_0: *mut drm_fb = 0 as *mut drm_fb;
    let mut next_bo: *mut gbm_bo = 0 as *mut gbm_bo;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut waiting_for_flip: libc::c_int = 1 as libc::c_int;
    let mut fds: fd_set = fd_set{__fds_bits: [0; 16],};
    let mut evctx: drmEventContext =
        {
            let mut init =
                _drmEventContext{version: 4 as libc::c_int,
                                 vblank_handler: None,
                                 page_flip_handler:
                                     Some(page_flip_handler as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_uint,
                                                                   _:
                                                                       libc::c_uint,
                                                                   _:
                                                                       libc::c_uint,
                                                                   _:
                                                                       *mut libc::c_void)
                                                  -> ()),
                                 page_flip_handler2: None,
                                 sequence_handler: None,};
            init
        };
    eglSwapBuffers(display, egl_surface);
    next_bo = gbm_surface_lock_front_buffer(gbm_surface);
    fb_0 = drm_fb_get_from_bo(next_bo);
    ret =
        drmModePageFlip(device, (*crtc).crtc_id, (*fb_0).fb_id,
                        0x1 as libc::c_int as uint32_t,
                        &mut waiting_for_flip as *mut libc::c_int as
                            *mut libc::c_void);
    if ret != 0 as libc::c_int {
        printf(b"drmModePageFlip failed\n\x00" as *const u8 as
                   *const libc::c_char);
    }
    while waiting_for_flip != 0 {
        ret =
            select(device + 1 as libc::c_int, &mut fds, 0 as *mut fd_set,
                   0 as *mut fd_set, 0 as *mut timeval);
        if ret < 0 as libc::c_int {
            printf(b"select err\n\x00" as *const u8 as *const libc::c_char);
            return
        } else {
            if ret == 0 as libc::c_int {
                printf(b"select timeout!\n\x00" as *const u8 as
                           *const libc::c_char);
                return
            } else {
                if fds.__fds_bits[(0 as libc::c_int /
                                       (8 as libc::c_int *
                                            ::std::mem::size_of::<__fd_mask>()
                                                as libc::c_ulong as
                                                libc::c_int)) as usize] &
                       ((1 as libc::c_ulong) <<
                            0 as libc::c_int %
                                (8 as libc::c_int *
                                     ::std::mem::size_of::<__fd_mask>() as
                                         libc::c_ulong as libc::c_int)) as
                           __fd_mask != 0 as libc::c_int as libc::c_long {
                    printf(b"user interrupted!\n\x00" as *const u8 as
                               *const libc::c_char);
                    return
                }
            }
        }
        drmHandleEvent(device, &mut evctx);
    }
    /* release last buffer to render on again: */
    gbm_surface_release_buffer(gbm_surface, bo);
    bo = next_bo;
    /* handle = gbm_bo_get_handle(bo).u32; */
    /* pitch = gbm_bo_get_stride(bo); */
    /* drmModeAddFB(device, mode_info.hdisplay, mode_info.vdisplay, 24, 32, pitch, handle, &fb); */
    /* /\* drmModeSetCrtc (device, crtc->crtc_id, fb, 0, 0, &connector_id, 1, &mode_info); *\/ */
    /* /\* if (previous_bo) { *\/ */
    /* /\*     drmModeRmFB(device, previous_fb); *\/ */
    /* /\*     gbm_surface_release_buffer(gbm_surface, previous_bo); *\/ */
    /* /\* } *\/ */
    /* /\* previous_bo = bo; *\/ */
    /* /\* previous_fb = fb; */
}
unsafe extern "C" fn match_config_to_visual(mut egl_display: EGLDisplay,
                                            mut visual_id: EGLint,
                                            mut configs_0: *mut EGLConfig,
                                            mut count_0: libc::c_int)
 -> libc::c_int {
    let mut id: EGLint = 0;
    i = 0 as libc::c_int;
    while i < count_0 {
        if !(eglGetConfigAttrib(egl_display, *configs_0.offset(i as isize),
                                0x302e as libc::c_int, &mut id) == 0) {
            if id == visual_id { return i }
        }
        i += 1
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn try_device(mut device_name: *mut libc::c_char)
 -> libc::c_int {
    device = open(device_name, 0o2 as libc::c_int);
    if device == 0 as libc::c_int {
        printf(b"No %s, lets try card1\n\x00" as *const u8 as
                   *const libc::c_char, device_name);
        return 0 as libc::c_int
    }
    printf(b"Found %s\n\x00" as *const u8 as *const libc::c_char,
           device_name);
    resources = drmModeGetResources(device);
    if resources.is_null() {
        printf(b"No resource in /dev/dri/card0, lets try card1\n\x00" as
                   *const u8 as *const libc::c_char);
        close(device);
        return 0 as libc::c_int
    }
    printf(b"Found resources in device\n\x00" as *const u8 as
               *const libc::c_char);
    return 1 as libc::c_int;
}
unsafe extern "C" fn has_ext(mut extension_list: *const libc::c_char,
                             mut ext: *const libc::c_char) -> libc::c_int {
    let mut ptr: *const libc::c_char = extension_list;
    let mut len: libc::c_int = strlen(ext) as libc::c_int;
    if ptr.is_null() || *ptr as libc::c_int == '\u{0}' as i32 {
        return 0 as libc::c_int
    }
    loop  {
        ptr = strstr(ptr, ext);
        if ptr.is_null() { return 0 as libc::c_int }
        if *ptr.offset(len as isize) as libc::c_int == ' ' as i32 ||
               *ptr.offset(len as isize) as libc::c_int == '\u{0}' as i32 {
            return 1 as libc::c_int
        }
        ptr = ptr.offset(len as isize)
    };
}
#[no_mangle]
pub static mut eglGetPlatformDisplayEXT: PFNEGLGETPLATFORMDISPLAYEXTPROC =
    None;
#[no_mangle]
pub unsafe extern "C" fn init() {
    let mut fb_0: *mut drm_fb = 0 as *mut drm_fb;
    let mut ret: libc::c_int = 0;
    printf(b"Connecting to DRI\n\x00" as *const u8 as *const libc::c_char);
    if try_device(b"/dev/dri/card0\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char) == 0 as libc::c_int {
        if try_device(b"/dev/dri/card1\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char) ==
               0 as libc::c_int {
            if try_device(b"/dev/dri/card2\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char) ==
                   0 as libc::c_int {
                printf(b"No dri card found\x00" as *const u8 as
                           *const libc::c_char);
                return
            }
        }
    }
    resources = drmModeGetResources(device);
    if resources.is_null() {
        printf(b"drmModeGetResources failed\n\x00" as *const u8 as
                   *const libc::c_char);
    }
    connector = find_connector(resources);
    connector_id = (*connector).connector_id;
    mode_info = *(*connector).modes.offset(0 as libc::c_int as isize);
    encoder = find_encoder(resources, connector);
    crtc = drmModeGetCrtc(device, (*encoder).crtc_id);
    drmModeFreeEncoder(encoder);
    drmModeFreeConnector(connector);
    drmModeFreeResources(resources);
    gbm_device = gbm_create_device(device);
    gbm_surface =
        gbm_surface_create(gbm_device, mode_info.hdisplay as uint32_t,
                           mode_info.vdisplay as uint32_t,
                           'X' as i32 as uint32_t |
                               ('R' as i32 as uint32_t) << 8 as libc::c_int |
                               ('2' as i32 as uint32_t) << 16 as libc::c_int |
                               ('4' as i32 as uint32_t) << 24 as libc::c_int,
                           (GBM_BO_USE_SCANOUT as libc::c_int |
                                GBM_BO_USE_RENDERING as libc::c_int) as
                               uint32_t);
    printf(b"requesting extensions\n\x00" as *const u8 as
               *const libc::c_char);
    let mut egl_exts_client: *mut libc::c_char =
        eglQueryString(0 as EGLDisplay, 0x3055 as libc::c_int) as
            *mut libc::c_char;
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char, egl_exts_client);
    if has_ext(egl_exts_client,
               b"EGL_EXT_platform_base\x00" as *const u8 as
                   *const libc::c_char) == 0 as libc::c_int {
        printf(b"No EGL_EXT_platform_base extension!\n\x00" as *const u8 as
                   *const libc::c_char);
    }
    eglGetPlatformDisplayEXT =
        ::std::mem::transmute::<*mut libc::c_void,
                                PFNEGLGETPLATFORMDISPLAYEXTPROC>(::std::mem::transmute::<__eglMustCastToProperFunctionPointerType,
                                                                                         *mut libc::c_void>(eglGetProcAddress(b"eglGetPlatformDisplayEXT\x00"
                                                                                                                                  as
                                                                                                                                  *const u8
                                                                                                                                  as
                                                                                                                                  *const libc::c_char)));
    //display = eglGetDisplay (gbm_device);
    display =
        eglGetPlatformDisplayEXT.expect("non-null function pointer")(0x31d7 as
                                                                         libc::c_int
                                                                         as
                                                                         EGLenum,
                                                                     gbm_device
                                                                         as
                                                                         *mut libc::c_void,
                                                                     0 as
                                                                         *const EGLint);
    eglInitialize(display, 0 as *mut EGLint, 0 as *mut EGLint);
    eglBindAPI(0x30a0 as libc::c_int as EGLenum);
    eglGetConfigs(display, 0 as *mut EGLConfig, 0 as libc::c_int, &mut count);
    configs =
        malloc((count as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<EGLConfig>()
                                                    as libc::c_ulong)) as
            *mut EGLConfig;
    eglChooseConfig(display, attributes.as_mut_ptr(), configs, count,
                    &mut num_config);
    config_index =
        match_config_to_visual(display,
                               ('X' as i32 as uint32_t |
                                    ('R' as i32 as uint32_t) <<
                                        8 as libc::c_int |
                                    ('2' as i32 as uint32_t) <<
                                        16 as libc::c_int |
                                    ('4' as i32 as uint32_t) <<
                                        24 as libc::c_int) as EGLint, configs,
                               num_config);
    context =
        eglCreateContext(display, *configs.offset(config_index as isize),
                         0 as EGLContext, context_attribs.as_ptr());
    egl_surface =
        eglCreateWindowSurface(display,
                               *configs.offset(config_index as isize),
                               gbm_surface as EGLNativeWindowType,
                               0 as *const EGLint);
    free(configs as *mut libc::c_void);
    eglMakeCurrent(display, egl_surface, egl_surface, context);
    glClearColor(0.5f64 as GLclampf, 0.5f64 as GLclampf, 0.5f64 as GLclampf,
                 1.0f64 as GLclampf);
    glClear(0x4000 as libc::c_int as GLbitfield);
    eglSwapBuffers(display, egl_surface);
    bo = gbm_surface_lock_front_buffer(gbm_surface);
    fb_0 = drm_fb_get_from_bo(bo);
    ret =
        drmModeSetCrtc(device, (*crtc).crtc_id, (*fb_0).fb_id,
                       0 as libc::c_int as uint32_t,
                       0 as libc::c_int as uint32_t, &mut connector_id,
                       1 as libc::c_int, &mut mode_info);
    if ret != 0 {
        printf(b"failed to set mode\n\x00" as *const u8 as
                   *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn deinit() {
    drmModeSetCrtc(device, (*crtc).crtc_id, (*crtc).buffer_id, (*crtc).x,
                   (*crtc).y, &mut connector_id, 1 as libc::c_int,
                   &mut (*crtc).mode);
    drmModeFreeCrtc(crtc);
    if !previous_bo.is_null() {
        drmModeRmFB(device, previous_fb);
        gbm_surface_release_buffer(gbm_surface, previous_bo);
    }
    eglDestroySurface(display, egl_surface);
    gbm_surface_destroy(gbm_surface);
    eglDestroyContext(display, context);
    eglTerminate(display);
    gbm_device_destroy(gbm_device);
    close(device);
    printf(b"real done\n\x00" as *const u8 as *const libc::c_char);
}
/* } */
/*         swap_buffers(); */
/*     } */
/*     while (1) { */
/*         glClearColor(1.0, 0.0, 0.0, 1.0);         */
/*         glClear(GL_COLOR_BUFFER_BIT); */
/*     printf("inited\n"); */
/* int main()  */
/* { */
/*     init(); */
