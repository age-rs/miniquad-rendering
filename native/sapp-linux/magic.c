//----------------------------------------------------------------------
//--------  Trying to get OpenGL ES screen on RPi4 without X
//--------  based on drm-gbm https://github.com/eyelash/tutorials/blob/master/drm-gbm.c
//--------  and kmscube https://github.com/robclark/kmscube
//--------  pik33@o2.pl
//----------------------------------------------------------------------

#include <xf86drm.h>
#include <xf86drmMode.h>
#include <gbm.h>
#include <EGL/egl.h>
#include <GL/gl.h>
#include <stdlib.h>
#include <fcntl.h>
#include <unistd.h>
#include <stdio.h>

#define EXIT(msg) { fputs (msg, stderr); exit (EXIT_FAILURE); }

// global variables declarations

static int device;
static drmModeRes *resources;
static drmModeConnector *connector;
static uint32_t connector_id;
static drmModeEncoder *encoder;
static drmModeModeInfo mode_info;
static drmModeCrtc *crtc;
static struct gbm_device *gbm_device;
static EGLDisplay display;
static EGLContext context;
static struct gbm_surface *gbm_surface;
static EGLSurface egl_surface;
EGLConfig config;
EGLint num_config;
EGLint count=0;
EGLConfig *configs;
int config_index;
int i;
       
static struct gbm_bo *previous_bo = NULL;
static uint32_t previous_fb;       

static EGLint attributes[] = {
                              EGL_SURFACE_TYPE, EGL_WINDOW_BIT,
                              EGL_RED_SIZE, 8,
                              EGL_GREEN_SIZE, 8,
                              EGL_BLUE_SIZE, 8,
                              EGL_ALPHA_SIZE, 0,
                              EGL_DEPTH_SIZE, 8,
                              EGL_RENDERABLE_TYPE, EGL_OPENGL_ES2_BIT,
                              EGL_NONE
};

static const EGLint context_attribs[] = {
                                         EGL_CONTEXT_CLIENT_VERSION, 2,
                                         EGL_NONE
};

struct gbm_bo *bo;

uint32_t handle;
uint32_t pitch;
int32_t fb;
uint64_t modifier;


static drmModeConnector *find_connector (drmModeRes *resources) {
    for (i=0; i<resources->count_connectors; i++) {
        drmModeConnector *connector = drmModeGetConnector (device, resources->connectors[i]);
        if (connector->connection == DRM_MODE_CONNECTED) {return connector;}
        drmModeFreeConnector (connector);
    }
    return NULL; // if no connector found
}

static drmModeEncoder *find_encoder (drmModeRes *resources, drmModeConnector *connector) {
    if (connector->encoder_id) {
        return drmModeGetEncoder (device, connector->encoder_id);
    }
    return NULL; // if no encoder found
}

struct drm_fb {
	struct gbm_bo *bo;
	uint32_t fb_id;
};

static void
drm_fb_destroy_callback(struct gbm_bo *bo, void *data)
{
	struct drm_fb *fb = data;
	struct gbm_device *gbm = gbm_bo_get_device(bo);

	if (fb->fb_id)
		drmModeRmFB(device, fb->fb_id);

	free(fb);
}

static struct drm_fb * drm_fb_get_from_bo(struct gbm_bo *bo)
{
	struct drm_fb *fb = gbm_bo_get_user_data(bo);
	uint32_t width, height, stride, handle;
	int ret;

	if (fb)
		return fb;

	fb = calloc(1, sizeof *fb);
	fb->bo = bo;

	width = gbm_bo_get_width(bo);
	height = gbm_bo_get_height(bo);
	stride = gbm_bo_get_stride(bo);
	handle = gbm_bo_get_handle(bo).u32;

	ret = drmModeAddFB(device, width, height, 24, 32, stride, handle, &fb->fb_id);
	if (ret) {
		printf("failed to create fb\n");
		free(fb);
		return NULL;
	}

	gbm_bo_set_user_data(bo, fb, drm_fb_destroy_callback);

	return fb;
}

static void page_flip_handler(int fd, unsigned int frame,
		  unsigned int sec, unsigned int usec, void *data)
{
	int *waiting_for_flip = data;
	*waiting_for_flip = 0;
}

void swap_buffers() {
    struct drm_fb *fb;
    struct gbm_bo *next_bo;	

    int ret = 0;    
    int waiting_for_flip = 1;
	fd_set fds;
    drmEventContext evctx = {
			.version = DRM_EVENT_CONTEXT_VERSION,
			.page_flip_handler = page_flip_handler,
	};

    eglSwapBuffers(display, egl_surface);

    next_bo = gbm_surface_lock_front_buffer(gbm_surface);
	fb = drm_fb_get_from_bo(next_bo);
    ret = drmModePageFlip(device, crtc->crtc_id, fb->fb_id,
                          DRM_MODE_PAGE_FLIP_EVENT, &waiting_for_flip);
    if (ret != 0) {
        printf("drmModePageFlip failed\n");
    }
    while (waiting_for_flip) {
        ret = select(device + 1, &fds, NULL, NULL, NULL);
        if (ret < 0) {
            printf("select err\n");
            return;
        } else if (ret == 0) {
            printf("select timeout!\n");
            return;
        } else if (FD_ISSET(0, &fds)) {
            printf("user interrupted!\n");
            return;
        }
        drmHandleEvent(device, &evctx);
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

static int match_config_to_visual(EGLDisplay egl_display, EGLint visual_id, EGLConfig *configs, int count) {
    EGLint id;
    for (i = 0; i < count; ++i) {
        if (!eglGetConfigAttrib(egl_display, configs[i], EGL_NATIVE_VISUAL_ID,&id)) {
            continue;
        }
        if (id == visual_id) {
            return i;
        }
    }
    return -1;
}

int try_device(char *device_name) {
    device = open(device_name, O_RDWR);
    if (device == 0) {
        printf("No %s, lets try card1\n", device_name);
        return 0;
    }

    printf("Found %s\n", device_name);

    resources = drmModeGetResources(device);
    if (resources == 0) {
        printf("No resource in /dev/dri/card0, lets try card1\n");

        close(device);
        return 0;
    }
    printf("Found resources in device\n");

    return 1;    
}

void init(void) {
    struct drm_fb *fb;
	int ret;
    
    printf("Connecting to DRI\n");
    
    if (try_device("/dev/dri/card0") == 0) {              
        if (try_device("/dev/dri/card1") == 0) {
            if (try_device("/dev/dri/card2") == 0) {
                printf("No dri card found");
                return;                
            }
        }
    }
    
    resources = drmModeGetResources(device);
    if (resources == 0) {
        printf("drmModeGetResources failed\n");
    }
    
    connector = find_connector(resources);    
    connector_id = connector->connector_id;
    mode_info = connector->modes[0];
    encoder = find_encoder (resources, connector);
    
    crtc = drmModeGetCrtc (device, encoder->crtc_id);
    drmModeFreeEncoder (encoder);
    drmModeFreeConnector (connector);
    drmModeFreeResources (resources);
    gbm_device = gbm_create_device (device);
    gbm_surface = gbm_surface_create(gbm_device, 
                                     mode_info.hdisplay, 
                                     mode_info.vdisplay, 
                                     GBM_FORMAT_XRGB8888,
                                     GBM_BO_USE_SCANOUT | GBM_BO_USE_RENDERING);


    printf("requesting extensions\n");
    char *egl_exts_client = eglQueryString(EGL_NO_DISPLAY, EGL_EXTENSIONS);
    printf("%s\n", egl_exts_client);

    display = eglGetDisplay (gbm_device);
    eglInitialize (display, NULL ,NULL);
    eglBindAPI(EGL_OPENGL_ES_API);
    
    eglGetConfigs(display, NULL, 0, &count);
    configs = malloc(count * sizeof *configs);
    eglChooseConfig (display, attributes, configs, count, &num_config);
    config_index = match_config_to_visual(display,GBM_FORMAT_XRGB8888,configs,num_config);

    context = eglCreateContext (display, configs[config_index], EGL_NO_CONTEXT, context_attribs);
    egl_surface = eglCreateWindowSurface (display, configs[config_index], gbm_surface, NULL);
    free(configs);
    
    eglMakeCurrent (display, egl_surface, egl_surface, context);

    glClearColor(0.5, 0.5, 0.5, 1.0);
	glClear(GL_COLOR_BUFFER_BIT);
	eglSwapBuffers(display, egl_surface);
    
    bo = gbm_surface_lock_front_buffer(gbm_surface);
	fb = drm_fb_get_from_bo(bo);

    ret = drmModeSetCrtc(device, crtc->crtc_id, fb->fb_id, 0, 0,
			&connector_id, 1, &mode_info);
    if (ret) {
		printf("failed to set mode\n");	
	}
}

void deinit(void) {
    drmModeSetCrtc (device, crtc->crtc_id, crtc->buffer_id, crtc->x, crtc->y, &connector_id, 1, &crtc->mode);

    drmModeFreeCrtc (crtc);
    if (previous_bo) {
        drmModeRmFB (device, previous_fb);
        gbm_surface_release_buffer (gbm_surface, previous_bo);
    }
    eglDestroySurface (display, egl_surface);
    gbm_surface_destroy (gbm_surface);
    eglDestroyContext (display, context);
    eglTerminate (display);
    gbm_device_destroy (gbm_device);
    close (device);

    printf("real done\n");
}


/* int main()  */
/* { */
/*     init(); */

/*     printf("inited\n"); */
    
/*     while (1) { */
/*         glClearColor(1.0, 0.0, 0.0, 1.0);         */
/*         glClear(GL_COLOR_BUFFER_BIT); */
        
/*         swap_buffers(); */
/*     } */
    
/* } */
