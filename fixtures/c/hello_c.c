void _start(void) {
    const char msg[] = "Hello from C on ARM64!\n";
    long len = sizeof(msg) - 1;

    register long x0 __asm__("x0") = 1;
    register const char *x1 __asm__("x1") = msg;
    register long x2 __asm__("x2") = len;
    register long x8 __asm__("x8") = 64; // sys_write
    __asm__ __volatile__("svc #0" : : "r"(x0), "r"(x1), "r"(x2), "r"(x8) : "memory");

    register long ex0 __asm__("x0") = 0;
    register long ex8 __asm__("x8") = 93; // sys_exit
    __asm__ __volatile__("svc #0" : : "r"(ex0), "r"(ex8) : "memory");

    while (1) {}
}
