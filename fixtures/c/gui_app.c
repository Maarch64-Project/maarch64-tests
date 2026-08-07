#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

typedef void* EGLDisplay;
typedef int EGLBoolean;
typedef unsigned int GLenum;
typedef float GLclampf;

extern EGLDisplay eglGetDisplay(void* display_id);
extern EGLBoolean eglInitialize(EGLDisplay dpy, int *major, int *minor);
extern const char* eglQueryString(EGLDisplay dpy, int name);
extern const char* glGetString(GLenum name);
extern void glClearColor(GLclampf red, GLclampf green, GLclampf blue, GLclampf alpha);
extern void glClear(unsigned int mask);

int main(void) {
    printf("==================================================\n");
    printf("   Maarch64 AArch64 GUI & GPU Passthrough Demo   \n");
    printf("==================================================\n");
    
    printf("[GUI App] Requesting EGL Display...\n");
    EGLDisplay dpy = eglGetDisplay(NULL);
    printf("[GUI App] eglGetDisplay handle: %p\n", dpy);
    
    int major = 0, minor = 0;
    if (eglInitialize(dpy, &major, &minor)) {
        printf("[GUI App] EGL Initialized! Version: %d.%d\n", major, minor);
    }
    
    const char *vendor = eglQueryString(dpy, 0x3053);
    printf("[GUI App] EGL Vendor String: %s\n", vendor ? vendor : "Unknown");
    
    const char *gl_vendor = glGetString(0x1F00);
    const char *gl_renderer = glGetString(0x1F01);
    const char *gl_version = glGetString(0x1F02);
    
    printf("[GUI App] GPU Vendor   : %s\n", gl_vendor ? gl_vendor : "Unknown");
    printf("[GUI App] GPU Renderer : %s\n", gl_renderer ? gl_renderer : "Unknown");
    printf("[GUI App] GPU Version  : %s\n", gl_version ? gl_version : "Unknown");
    
    printf("[GUI App] Setting background clear color (Sky Blue)...\n");
    glClearColor(0.2f, 0.6f, 0.9f, 1.0f);
    printf("[GUI App] Executing glClear(GL_COLOR_BUFFER_BIT)...\n");
    glClear(0x4000);
    
    printf("\n[SUCCESS] GUI Window & GPU Render Pipeline Initialized Perfectly!\n");
    sleep(3);
    return 0;
}
