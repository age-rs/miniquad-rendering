use ::libc;
pub type gbm_device = ();
pub type gbm_bo = ();
pub type gbm_surface = ();

extern "C" {
    #[no_mangle]
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn drmHandleEvent(fd: libc::c_int, evctx_0: drmEventContextPtr) -> libc::c_int;
    #[no_mangle]
    fn drmModeFreeConnector(ptr: drmModeConnectorPtr);
    #[no_mangle]
    fn drmModeFreeEncoder(ptr: drmModeEncoderPtr);
    #[no_mangle]
    fn drmModeGetResources(fd: libc::c_int) -> drmModeResPtr;
    #[no_mangle]
    fn drmModeAddFB(
        fd: libc::c_int,
        width: uint32_t,
        height: uint32_t,
        depth: uint8_t,
        bpp: uint8_t,
        pitch: uint32_t,
        bo_handle: uint32_t,
        buf_id: *mut uint32_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn drmModeRmFB(fd: libc::c_int, bufferId: uint32_t) -> libc::c_int;
    #[no_mangle]
    fn drmModeSetCrtc(
        fd: libc::c_int,
        crtcId: uint32_t,
        bufferId: uint32_t,
        x: uint32_t,
        y: uint32_t,
        connectors: *mut uint32_t,
        count_0: libc::c_int,
        mode: drmModeModeInfoPtr,
    ) -> libc::c_int;
    #[no_mangle]
    fn drmModeGetEncoder(fd: libc::c_int, encoder_id: uint32_t) -> drmModeEncoderPtr;
    #[no_mangle]
    fn drmModeGetConnector(fd: libc::c_int, connectorId: uint32_t) -> drmModeConnectorPtr;
    #[no_mangle]
    fn drmModePageFlip(
        fd: libc::c_int,
        crtc_id: uint32_t,
        fb_id: uint32_t,
        flags: uint32_t,
        user_data: *mut libc::c_void,
    ) -> libc::c_int;
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
    fn gbm_bo_set_user_data(
        bo_0: *mut gbm_bo,
        data: *mut libc::c_void,
        destroy_user_data: Option<unsafe extern "C" fn(_: *mut gbm_bo, _: *mut libc::c_void) -> ()>,
    );
    #[no_mangle]
    fn gbm_bo_get_user_data(bo_0: *mut gbm_bo) -> *mut libc::c_void;
    #[no_mangle]
    fn gbm_surface_create(
        gbm_0: *mut gbm_device,
        width: uint32_t,
        height: uint32_t,
        format: uint32_t,
        flags: uint32_t,
    ) -> *mut gbm_surface;
    #[no_mangle]
    fn gbm_surface_lock_front_buffer(surface: *mut gbm_surface) -> *mut gbm_bo;
    #[no_mangle]
    fn gbm_surface_release_buffer(surface: *mut gbm_surface, bo_0: *mut gbm_bo);
    #[no_mangle]
    fn glClear(mask: GLbitfield);
    #[no_mangle]
    fn glClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
    #[no_mangle]
    fn glGetString(name: GLenum) -> *const GLubyte;
    #[no_mangle]
    fn eglChooseConfig(
        dpy: EGLDisplay,
        attrib_list: *const EGLint,
        configs_0: *mut EGLConfig,
        config_size: EGLint,
        num_config_0: *mut EGLint,
    ) -> EGLBoolean;
    #[no_mangle]
    fn eglCreateContext(
        dpy: EGLDisplay,
        config: EGLConfig,
        share_context: EGLContext,
        attrib_list: *const EGLint,
    ) -> EGLContext;
    #[no_mangle]
    fn eglCreateWindowSurface(
        dpy: EGLDisplay,
        config: EGLConfig,
        win: EGLNativeWindowType,
        attrib_list: *const EGLint,
    ) -> EGLSurface;
    #[no_mangle]
    fn eglGetConfigAttrib(
        dpy: EGLDisplay,
        config: EGLConfig,
        attribute: EGLint,
        value: *mut EGLint,
    ) -> EGLBoolean;
    #[no_mangle]
    fn eglGetConfigs(
        dpy: EGLDisplay,
        configs_0: *mut EGLConfig,
        config_size: EGLint,
        num_config_0: *mut EGLint,
    ) -> EGLBoolean;
    #[no_mangle]
    fn eglGetProcAddress(procname: *const libc::c_char)
        -> __eglMustCastToProperFunctionPointerType;
    #[no_mangle]
    fn eglInitialize(dpy: EGLDisplay, major: *mut EGLint, minor: *mut EGLint) -> EGLBoolean;
    #[no_mangle]
    fn eglMakeCurrent(
        dpy: EGLDisplay,
        draw: EGLSurface,
        read: EGLSurface,
        ctx: EGLContext,
    ) -> EGLBoolean;
    #[no_mangle]
    fn eglQueryString(dpy: EGLDisplay, name: EGLint) -> *const libc::c_char;
    #[no_mangle]
    fn eglSwapBuffers(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean;
    #[no_mangle]
    fn eglBindAPI(api: EGLenum) -> EGLBoolean;
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
    pub vblank_handler: Option<
        unsafe extern "C" fn(
            _: libc::c_int,
            _: libc::c_uint,
            _: libc::c_uint,
            _: libc::c_uint,
            _: *mut libc::c_void,
        ) -> (),
    >,
    pub page_flip_handler: Option<
        unsafe extern "C" fn(
            _: libc::c_int,
            _: libc::c_uint,
            _: libc::c_uint,
            _: libc::c_uint,
            _: *mut libc::c_void,
        ) -> (),
    >,
    pub page_flip_handler2: Option<
        unsafe extern "C" fn(
            _: libc::c_int,
            _: libc::c_uint,
            _: libc::c_uint,
            _: libc::c_uint,
            _: libc::c_uint,
            _: *mut libc::c_void,
        ) -> (),
    >,
    pub sequence_handler:
        Option<unsafe extern "C" fn(_: libc::c_int, _: uint64_t, _: uint64_t, _: uint64_t) -> ()>,
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
pub type khronos_uint8_t = libc::c_uchar;
pub type khronos_float_t = libc::c_float;
pub type GLenum = libc::c_uint;
pub type GLuint = libc::c_uint;
pub type GLfloat = khronos_float_t;
pub type GLbitfield = libc::c_uint;
pub type GLint = libc::c_int;
pub type GLubyte = khronos_uint8_t;
pub type EGLNativeWindowType = *mut libc::c_void;
pub type EGLint = khronos_int32_t;
pub type EGLBoolean = libc::c_uint;
pub type EGLDisplay = *mut libc::c_void;
pub type EGLConfig = *mut libc::c_void;
pub type EGLSurface = *mut libc::c_void;
pub type EGLContext = *mut libc::c_void;
pub type __eglMustCastToProperFunctionPointerType = Option<unsafe extern "C" fn() -> ()>;
pub type EGLenum = libc::c_uint;
pub type PFNEGLGETPLATFORMDISPLAYEXTPROC =
    Option<unsafe extern "C" fn(_: EGLenum, _: *mut libc::c_void, _: *const EGLint) -> EGLDisplay>;
// gcc -o drmgl Linux_DRM_OpenGLES.c `pkg-config --cflags --libs libdrm` -lgbm -lEGL -lGLESv2
/*
 * Copyright (c) 2012 Arvin Schnell <arvin.schnell@gmail.com>
 * Copyright (c) 2012 Rob Clark <rob@ti.com>
 * Copyright (c) 2017 Miouyouyou <Myy> <myy@miouyouyou.fr>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sub license,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the
 * next paragraph) shall be included in all copies or substantial portions
 * of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */
/* Based on a egl cube test app originally written by Arvin Schnell */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub display: EGLDisplay,
    pub config: EGLConfig,
    pub context: EGLContext,
    pub surface: EGLSurface,
    pub program: GLuint,
    pub modelviewmatrix: GLint,
    pub modelviewprojectionmatrix: GLint,
    pub normalmatrix: GLint,
    pub vbo: GLuint,
    pub positionsoffset: GLuint,
    pub colorsoffset: GLuint,
    pub normalsoffset: GLuint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub dev: *mut gbm_device,
    pub surface: *mut gbm_surface,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub fd: libc::c_int,
    pub mode: *mut drmModeModeInfo,
    pub crtc_id: uint32_t,
    pub connector_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct drm_fb {
    pub bo: *mut gbm_bo,
    pub fb_id: uint32_t,
}
static mut gl: C2RustUnnamed = C2RustUnnamed {
    display: 0 as *const libc::c_void as *mut libc::c_void,
    config: 0 as *const libc::c_void as *mut libc::c_void,
    context: 0 as *const libc::c_void as *mut libc::c_void,
    surface: 0 as *const libc::c_void as *mut libc::c_void,
    program: 0,
    modelviewmatrix: 0,
    modelviewprojectionmatrix: 0,
    normalmatrix: 0,
    vbo: 0,
    positionsoffset: 0,
    colorsoffset: 0,
    normalsoffset: 0,
};
static mut gbm: C2RustUnnamed_0 = C2RustUnnamed_0 {
    dev: 0 as *const gbm_device as *mut gbm_device,
    surface: 0 as *const gbm_surface as *mut gbm_surface,
};
static mut drm: C2RustUnnamed_1 = C2RustUnnamed_1 {
    fd: 0,
    mode: 0 as *const drmModeModeInfo as *mut drmModeModeInfo,
    crtc_id: 0,
    connector_id: 0,
};
unsafe extern "C" fn find_crtc_for_encoder(
    mut resources: *const drmModeRes,
    mut encoder: *const drmModeEncoder,
) -> uint32_t {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*resources).count_crtcs {
        /* possible_crtcs is a bitmask as described here:
         * https://dvdhrm.wordpress.com/2012/09/13/linux-drm-mode-setting-api
         */
        let crtc_mask: uint32_t = ((1 as libc::c_int) << i) as uint32_t;
        let crtc_id: uint32_t = *(*resources).crtcs.offset(i as isize);
        if (*encoder).possible_crtcs & crtc_mask != 0 {
            return crtc_id;
        }
        i += 1
    }
    /* no match found */
    return -(1 as libc::c_int) as uint32_t;
}
unsafe extern "C" fn find_crtc_for_connector(
    mut resources: *const drmModeRes,
    mut connector: *const drmModeConnector,
) -> uint32_t {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*connector).count_encoders {
        let encoder_id: uint32_t = *(*connector).encoders.offset(i as isize);
        let mut encoder: *mut drmModeEncoder = drmModeGetEncoder(drm.fd, encoder_id);
        if !encoder.is_null() {
            let crtc_id: uint32_t = find_crtc_for_encoder(resources, encoder);
            drmModeFreeEncoder(encoder);
            if crtc_id != 0 as libc::c_int as libc::c_uint {
                return crtc_id;
            }
        }
        i += 1
    }
    /* no match found */
    return -(1 as libc::c_int) as uint32_t;
}
unsafe extern "C" fn init_drm() -> libc::c_int {
    let mut resources: *mut drmModeRes = 0 as *mut drmModeRes;
    let mut connector: *mut drmModeConnector = 0 as *mut drmModeConnector;
    let mut encoder: *mut drmModeEncoder = 0 as *mut drmModeEncoder;
    let mut i: libc::c_int = 0;
    let mut area: libc::c_int = 0;
    drm.fd = open(
        b"/dev/dri/card0\x00" as *const u8 as *const libc::c_char,
        0o2 as libc::c_int,
    );
    if drm.fd < 0 as libc::c_int {
        printf(b"could not open drm device\n\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    resources = drmModeGetResources(drm.fd);
    if resources.is_null() {
        printf(
            b"drmModeGetResources failed: %s\n\x00" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    /* find a connected connector: */
    i = 0 as libc::c_int;
    while i < (*resources).count_connectors {
        connector = drmModeGetConnector(drm.fd, *(*resources).connectors.offset(i as isize));
        if (*connector).connection as libc::c_uint
            == DRM_MODE_CONNECTED as libc::c_int as libc::c_uint
        {
            break;
        }
        drmModeFreeConnector(connector);
        connector = 0 as *mut drmModeConnector;
        i += 1
    }
    if connector.is_null() {
        /* we could be fancy and listen for hotplug events and wait for
         * a connector..
         */
        printf(b"no connected connector!\n\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    /* find prefered mode or the highest resolution mode: */
    i = 0 as libc::c_int;
    area = 0 as libc::c_int;
    while i < (*connector).count_modes {
        let mut current_mode: *mut drmModeModeInfo =
            &mut *(*connector).modes.offset(i as isize) as *mut _drmModeModeInfo;
        if (*current_mode).type_0 & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint != 0 {
            drm.mode = current_mode
        }
        let mut current_area: libc::c_int =
            (*current_mode).hdisplay as libc::c_int * (*current_mode).vdisplay as libc::c_int;
        if current_area > area {
            drm.mode = current_mode;
            area = current_area
        }
        i += 1
    }
    if drm.mode.is_null() {
        printf(b"could not find mode!\n\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    /* find encoder: */
    i = 0 as libc::c_int;
    while i < (*resources).count_encoders {
        encoder = drmModeGetEncoder(drm.fd, *(*resources).encoders.offset(i as isize));
        if (*encoder).encoder_id == (*connector).encoder_id {
            break;
        }
        drmModeFreeEncoder(encoder);
        encoder = 0 as *mut drmModeEncoder;
        i += 1
    }
    if !encoder.is_null() {
        drm.crtc_id = (*encoder).crtc_id
    } else {
        let mut crtc_id: uint32_t = find_crtc_for_connector(resources, connector);
        if crtc_id == 0 as libc::c_int as libc::c_uint {
            printf(b"no crtc found!\n\x00" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int);
        }
        drm.crtc_id = crtc_id
    }
    drm.connector_id = (*connector).connector_id;
    return 0 as libc::c_int;
}
#[no_mangle]
pub static mut configs: *mut EGLConfig = 0 as *const EGLConfig as *mut EGLConfig;
#[no_mangle]
pub static mut config_index: libc::c_int = 0;
#[no_mangle]
pub static mut num_config: EGLint = 0;
#[no_mangle]
pub static mut count: EGLint = 0 as libc::c_int;
unsafe extern "C" fn init_gbm() -> libc::c_int {
    gbm.dev = gbm_create_device(drm.fd);
    gbm.surface = gbm_surface_create(
        gbm.dev,
        (*drm.mode).hdisplay as uint32_t,
        (*drm.mode).vdisplay as uint32_t,
        'X' as i32 as uint32_t
            | ('R' as i32 as uint32_t) << 8 as libc::c_int
            | ('2' as i32 as uint32_t) << 16 as libc::c_int
            | ('4' as i32 as uint32_t) << 24 as libc::c_int,
        (GBM_BO_USE_SCANOUT as libc::c_int | GBM_BO_USE_RENDERING as libc::c_int) as uint32_t,
    );
    if gbm.surface.is_null() {
        printf(b"failed to create gbm surface\n\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn match_config_to_visual(
    mut egl_display: EGLDisplay,
    mut visual_id: EGLint,
    mut configs_0: *mut EGLConfig,
    mut count_0: libc::c_int,
) -> libc::c_int {
    let mut id: EGLint = 0;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < count_0 {
        if !(eglGetConfigAttrib(
            egl_display,
            *configs_0.offset(i as isize),
            0x302e as libc::c_int,
            &mut id,
        ) == 0)
        {
            if id == visual_id {
                return i;
            }
        }
        i += 1
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn init_gl() -> libc::c_int {
    let mut major: EGLint = 0;
    let mut minor: EGLint = 0;
    let mut n: EGLint = 0;
    let mut vertex_shader: GLuint = 0;
    let mut fragment_shader: GLuint = 0;
    let mut ret: GLint = 0;
    static mut context_attribs: [EGLint; 3] = [
        0x3098 as libc::c_int,
        2 as libc::c_int,
        0x3038 as libc::c_int,
    ];
    static mut config_attribs: [EGLint; 15] = [
        0x3033 as libc::c_int,
        0x4 as libc::c_int,
        0x3024 as libc::c_int,
        8 as libc::c_int,
        0x3023 as libc::c_int,
        8 as libc::c_int,
        0x3022 as libc::c_int,
        8 as libc::c_int,
        0x3021 as libc::c_int,
        0 as libc::c_int,
        0x3025 as libc::c_int,
        8 as libc::c_int,
        0x3040 as libc::c_int,
        0x4 as libc::c_int,
        0x3038 as libc::c_int,
    ];
    let mut get_platform_display: PFNEGLGETPLATFORMDISPLAYEXTPROC = None;
    get_platform_display =
        ::std::mem::transmute::<*mut libc::c_void, PFNEGLGETPLATFORMDISPLAYEXTPROC>(
            ::std::mem::transmute::<__eglMustCastToProperFunctionPointerType, *mut libc::c_void>(
                eglGetProcAddress(
                    b"eglGetPlatformDisplayEXT\x00" as *const u8 as *const libc::c_char,
                ),
            ),
        );
    gl.display = get_platform_display.expect("non-null function pointer")(
        0x31d7 as libc::c_int as EGLenum,
        gbm.dev as *mut libc::c_void,
        0 as *const EGLint,
    );
    if eglInitialize(gl.display, 0 as *mut EGLint, 0 as *mut EGLint) == 0 {
        printf(b"failed to initialize\n\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    printf(
        b"Using display %p with EGL version %d.%d\n\x00" as *const u8 as *const libc::c_char,
        gl.display,
        major,
        minor,
    );
    printf(
        b"EGL Version \"%s\"\n\x00" as *const u8 as *const libc::c_char,
        eglQueryString(gl.display, 0x3054 as libc::c_int),
    );
    printf(
        b"EGL Vendor \"%s\"\n\x00" as *const u8 as *const libc::c_char,
        eglQueryString(gl.display, 0x3053 as libc::c_int),
    );
    printf(
        b"EGL Extensions \"%s\"\n\x00" as *const u8 as *const libc::c_char,
        eglQueryString(gl.display, 0x3055 as libc::c_int),
    );
    if eglBindAPI(0x30a0 as libc::c_int as EGLenum) == 0 {
        printf(b"failed to bind api EGL_OPENGL_ES_API\n\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    /* if (!eglChooseConfig(gl.display, config_attribs, &gl.config, 1, &n) || n != 1) { */
    /* 	printf("failed to choose config: %d\n", n); */
    /* 	return -1; */
    /* } */
    eglGetConfigs(
        gl.display,
        0 as *mut EGLConfig,
        0 as libc::c_int,
        &mut count,
    );
    configs = malloc(
        (count as libc::c_ulong).wrapping_mul(::std::mem::size_of::<EGLConfig>() as libc::c_ulong),
    ) as *mut EGLConfig;
    eglChooseConfig(
        gl.display,
        config_attribs.as_ptr(),
        configs,
        count,
        &mut num_config,
    );
    config_index = match_config_to_visual(
        gl.display,
        ('X' as i32 as uint32_t
            | ('R' as i32 as uint32_t) << 8 as libc::c_int
            | ('2' as i32 as uint32_t) << 16 as libc::c_int
            | ('4' as i32 as uint32_t) << 24 as libc::c_int) as EGLint,
        configs,
        num_config,
    );
    gl.context = eglCreateContext(
        gl.display,
        *configs.offset(config_index as isize),
        0 as EGLContext,
        context_attribs.as_ptr(),
    );
    free(configs as *mut libc::c_void);
    if gl.context.is_null() {
        printf(b"failed to create context\n\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    gl.surface = eglCreateWindowSurface(
        gl.display,
        *configs.offset(config_index as isize),
        gbm.surface as EGLNativeWindowType,
        0 as *const EGLint,
    );
    if gl.surface.is_null() {
        printf(b"failed to create egl surface\n\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    /* connect the context to the surface */
    eglMakeCurrent(gl.display, gl.surface, gl.surface, gl.context);
    printf(
        b"GL Extensions: \"%s\"\n\x00" as *const u8 as *const libc::c_char,
        glGetString(0x1f03 as libc::c_int as GLenum),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn drm_fb_destroy_callback(mut bo_0: *mut gbm_bo, mut data: *mut libc::c_void) {
    let mut fb_0: *mut drm_fb = data as *mut drm_fb;
    let mut gbm_0: *mut gbm_device = gbm_bo_get_device(bo_0);
    if (*fb_0).fb_id != 0 {
        drmModeRmFB(drm.fd, (*fb_0).fb_id);
    }
    free(fb_0 as *mut libc::c_void);
}
unsafe extern "C" fn drm_fb_get_from_bo(mut bo_0: *mut gbm_bo) -> *mut drm_fb {
    let mut fb_0: *mut drm_fb = gbm_bo_get_user_data(bo_0) as *mut drm_fb;
    let mut width: uint32_t = 0;
    let mut height: uint32_t = 0;
    let mut stride: uint32_t = 0;
    let mut handle: uint32_t = 0;
    let mut ret: libc::c_int = 0;
    if !fb_0.is_null() {
        return fb_0;
    }
    fb_0 = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<drm_fb>() as libc::c_ulong,
    ) as *mut drm_fb;
    (*fb_0).bo = bo_0;
    width = gbm_bo_get_width(bo_0);
    height = gbm_bo_get_height(bo_0);
    stride = gbm_bo_get_stride(bo_0);
    handle = gbm_bo_get_handle(bo_0).u32_0;
    ret = drmModeAddFB(
        drm.fd,
        width,
        height,
        24 as libc::c_int as uint8_t,
        32 as libc::c_int as uint8_t,
        stride,
        handle,
        &mut (*fb_0).fb_id,
    );
    if ret != 0 {
        printf(
            b"failed to create fb: %s\n\x00" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        free(fb_0 as *mut libc::c_void);
        return 0 as *mut drm_fb;
    }
    gbm_bo_set_user_data(
        bo_0,
        fb_0 as *mut libc::c_void,
        Some(
            drm_fb_destroy_callback
                as unsafe extern "C" fn(_: *mut gbm_bo, _: *mut libc::c_void) -> (),
        ),
    );
    return fb_0;
}
unsafe extern "C" fn page_flip_handler(
    mut fd: libc::c_int,
    mut frame: libc::c_uint,
    mut sec: libc::c_uint,
    mut usec: libc::c_uint,
    mut data: *mut libc::c_void,
) {
    let mut waiting_for_flip: *mut libc::c_int = data as *mut libc::c_int;
    *waiting_for_flip = 0 as libc::c_int;
}
#[no_mangle]
pub static mut fds: fd_set = fd_set {
    __fds_bits: [0; 16],
};
#[no_mangle]
pub static mut bo: *mut gbm_bo = 0 as *const gbm_bo as *mut gbm_bo;
#[no_mangle]
pub static mut fb: *mut drm_fb = 0 as *const drm_fb as *mut drm_fb;
#[no_mangle]
pub static mut next_bo: *mut gbm_bo = 0 as *const gbm_bo as *mut gbm_bo;
#[no_mangle]
pub static mut evctx: drmEventContext = unsafe {
    {
        let mut init = _drmEventContext {
            version: 4 as libc::c_int,
            vblank_handler: None,
            page_flip_handler: Some(
                page_flip_handler
                    as unsafe extern "C" fn(
                        _: libc::c_int,
                        _: libc::c_uint,
                        _: libc::c_uint,
                        _: libc::c_uint,
                        _: *mut libc::c_void,
                    ) -> (),
            ),
            page_flip_handler2: None,
            sequence_handler: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn swap_buffers() {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut waiting_for_flip: libc::c_int = 1 as libc::c_int;
    eglSwapBuffers(gl.display, gl.surface);
    next_bo = gbm_surface_lock_front_buffer(gbm.surface);
    fb = drm_fb_get_from_bo(next_bo);
    ret = drmModePageFlip(
        drm.fd,
        drm.crtc_id,
        (*fb).fb_id,
        0x1 as libc::c_int as uint32_t,
        &mut waiting_for_flip as *mut libc::c_int as *mut libc::c_void,
    );
    if ret != 0 as libc::c_int {
        printf(b"drmModePageFlip failed\n\x00" as *const u8 as *const libc::c_char);
    }
    while waiting_for_flip != 0 {
        ret = select(
            drm.fd + 1 as libc::c_int,
            &mut fds,
            0 as *mut fd_set,
            0 as *mut fd_set,
            0 as *mut timeval,
        );
        if ret < 0 as libc::c_int {
            printf(
                b"select err: %s\n\x00" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            return;
        } else if ret == 0 as libc::c_int {
            printf(b"select timeout!\n\x00" as *const u8 as *const libc::c_char);
            return;
        } else if fds.__fds_bits[(0 as libc::c_int
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            & ((1 as libc::c_ulong)
                << 0 as libc::c_int
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
                as __fd_mask
            != 0 as libc::c_int as libc::c_long
        {
            printf(b"user interrupted!\n\x00" as *const u8 as *const libc::c_char);
            break;
        } else {
            drmHandleEvent(drm.fd, &mut evctx);
        }
    }
    /* release last buffer to render on again: */
    gbm_surface_release_buffer(gbm.surface, bo);
    bo = next_bo;
}
#[no_mangle]
pub unsafe extern "C" fn drm_screen_width() -> libc::c_int {
    return (*drm.mode).hdisplay as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn drm_screen_height() -> libc::c_int {
    return (*drm.mode).vdisplay as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn init() -> libc::c_int {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    let mut ret: libc::c_int = 0;
    ret = init_drm();
    if ret != 0 {
        printf(b"failed to initialize DRM\n\x00" as *const u8 as *const libc::c_char);
        return ret;
    }
    let mut __i: libc::c_uint = 0;
    let mut __arr: *mut fd_set = &mut fds;
    __i = 0 as libc::c_int as libc::c_uint;
    while (__i as libc::c_ulong)
        < (::std::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong)
    {
        (*__arr).__fds_bits[__i as usize] = 0 as libc::c_int as __fd_mask;
        __i = __i.wrapping_add(1)
    }
    fds.__fds_bits[(0 as libc::c_int
        / (8 as libc::c_int * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize] |= ((1 as libc::c_ulong)
        << 0 as libc::c_int
            % (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as __fd_mask;
    fds.__fds_bits[(drm.fd
        / (8 as libc::c_int * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize] |= ((1 as libc::c_ulong)
        << drm.fd
            % (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as __fd_mask;
    ret = init_gbm();
    if ret != 0 {
        printf(b"failed to initialize GBM\n\x00" as *const u8 as *const libc::c_char);
        return ret;
    }
    ret = init_gl();
    if ret != 0 {
        printf(b"failed to initialize EGL\n\x00" as *const u8 as *const libc::c_char);
        return ret;
    }
    /* clear the color buffer */
    glClearColor(
        0.5f64 as GLfloat,
        0.5f64 as GLfloat,
        0.5f64 as GLfloat,
        1.0f64 as GLfloat,
    );
    glClear(0x4000 as libc::c_int as GLbitfield);
    eglSwapBuffers(gl.display, gl.surface);
    bo = gbm_surface_lock_front_buffer(gbm.surface);
    fb = drm_fb_get_from_bo(bo);
    /* set mode: */
    ret = drmModeSetCrtc(
        drm.fd,
        drm.crtc_id,
        (*fb).fb_id,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        &mut drm.connector_id,
        1 as libc::c_int,
        drm.mode,
    );
    if ret != 0 {
        printf(
            b"failed to set mode: %s\n\x00" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return ret;
    }
    return ret;
}
/*     swap_buffers(); */
/*   } */
/* } */
/*   while (1) { */
/*     glClearColor(1.0, 0.0, 0.0, 1.0); */
/*     glClear(GL_COLOR_BUFFER_BIT); */
/*   printf("inited\n"); */
/* int main(void) { */
/*   init(); */
