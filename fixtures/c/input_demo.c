#include <X11/Xlib.h>
#include <X11/Xutil.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
    printf("==================================================\n");
    printf("[AArch64 Interactive GUI] Starting X11 Input Demo...\n");
    printf("==================================================\n");

    Display *dpy = XOpenDisplay(NULL);
    if (!dpy) {
        fprintf(stderr, "[!] Failed to open X11 Display!\n");
        return 1;
    }

    int screen = XDefaultScreen(dpy);
    Window root = XRootWindow(dpy, screen);

    unsigned long bg = 0x001E1E2E; // Dark Slate Blue
    Window win = XCreateSimpleWindow(dpy, root, 100, 100, 600, 400, 2, 0x00FFFFFF, bg);

    XStoreName(dpy, win, "Maarch64 AArch64 Interactive Input Window");

    // Select Mouse Click, Mouse Motion, and Keyboard events
    XSelectInput(dpy, win, ExposureMask | KeyPressMask | ButtonPressMask | ButtonReleaseMask | PointerMotionMask);

    XMapWindow(dpy, win);
    XFlush(dpy);

    printf("[+] Window Created and Mapped successfully!\n");
    printf("[+] Click anywhere inside the window or press any key to test input events!\n\n");

    XEvent ev;
    int running = 1;
    int click_count = 0;
    unsigned long colors[] = {
        0x0089B4FA, // Blue
        0x00A6E3A1, // Green
        0x00F38BA8, // Pink / Red
        0x00FAB387, // Peach
        0x00CBA6F7  // Mauve
    };
    int num_colors = sizeof(colors) / sizeof(colors[0]);

    GC gc = XCreateGC(dpy, win, 0, NULL);

    while (running) {
        XNextEvent(dpy, &ev);

        switch (ev.type) {
            case Expose:
                XSetForeground(dpy, gc, colors[click_count % num_colors]);
                XFillRectangle(dpy, win, gc, 50, 50, 500, 300);
                XFlush(dpy);
                break;

            case ButtonPress:
                click_count++;
                printf("🖱️ [AArch64 Event] Mouse CLICK Pressed at X=%d, Y=%d (Button=%d) | Click Count: %d\n",
                       ev.xbutton.x, ev.xbutton.y, ev.xbutton.button, click_count);

                // Change color on click
                XSetForeground(dpy, gc, colors[click_count % num_colors]);
                XFillRectangle(dpy, win, gc, 50, 50, 500, 300);
                XFlush(dpy);
                break;

            case ButtonRelease:
                printf("🖱️ [AArch64 Event] Mouse CLICK Released at X=%d, Y=%d\n", ev.xbutton.x, ev.xbutton.y);
                break;

            case MotionNotify:
                // Print position periodically
                if (ev.xmotion.x % 20 == 0) {
                    printf("🖐️ [AArch64 Event] Mouse Hover Motion at X=%d, Y=%d\n", ev.xmotion.x, ev.xmotion.y);
                }
                break;

            case KeyPress: {
                char buf[32] = {0};
                KeySym keysym;
                XLookupString(&ev.xkey, buf, sizeof(buf) - 1, &keysym, NULL);
                printf("⌨️ [AArch64 Event] KEYBOARD Key Pressed: '%s' (KeySym: {:#lx})\n", buf[0] ? buf : "Special", keysym);
                if (buf[0] == 'q' || keysym == 0xff1b) { // 'q' or ESC
                    printf("[AArch64 Event] Quit key pressed. Exiting interactive loop...\n");
                    running = 0;
                }
                break;
            }
        }
    }

    XCloseDisplay(dpy);
    printf("==================================================\n");
    printf("[AArch64 Interactive GUI] Input Test Completed Successfully!\n");
    printf("==================================================\n");
    return 0;
}
