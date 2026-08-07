#include <stdio.h>
#include <stdlib.h>
#include <math.h>

// Forward declarations for PulseAudio Simple API
typedef struct pa_simple pa_simple;

extern pa_simple* pa_simple_new(
    const char *server,
    const char *name,
    int dir,
    const char *dev,
    const char *stream_name,
    const void *ss,
    const void *map,
    const void *attr,
    int *error
);
extern int pa_simple_write(pa_simple *s, const void *data, size_t bytes, int *error);
extern int pa_simple_drain(pa_simple *s, int *error);
extern void pa_simple_free(pa_simple *s);

struct sample_spec {
    int format;
    unsigned int rate;
    unsigned char channels;
};

int main() {
    printf("==================================================\n");
    printf("[AArch64 Audio Demo] Starting PCM Sine Wave Audio Playback...\n");
    printf("==================================================\n");

    struct sample_spec ss = { 3, 44100, 2 }; // S16LE, 44100 Hz, Stereo
    int error = 0;

    pa_simple *s = pa_simple_new(NULL, "Maarch64 Audio Demo", 1, NULL, "Sine Tone", &ss, NULL, NULL, &error);
    if (!s) {
        fprintf(stderr, "[!] pa_simple_new failed with error: %d\n", error);
        return 1;
    }

    printf("[+] Connected to Host Sound Device via PulseAudio Simple Thunk!\n");
    printf("[+] Generating 440 Hz Sine Wave (Tone A4) audio stream for 2 seconds...\n");

    #define SAMPLE_RATE 44100
    #define DURATION_SEC 2
    #define NUM_SAMPLES (SAMPLE_RATE * DURATION_SEC)

    short *buffer = (short *)malloc(NUM_SAMPLES * 2 * sizeof(short));
    if (!buffer) {
        fprintf(stderr, "[!] Memory allocation failed!\n");
        return 1;
    }

    double freq = 440.0; // A4 tone
    for (int i = 0; i < NUM_SAMPLES; i++) {
        double t = (double)i / SAMPLE_RATE;
        short val = (short)(32767.0 * 0.5 * sin(2.0 * M_PI * freq * t));
        buffer[2 * i]     = val; // Left channel
        buffer[2 * i + 1] = val; // Right channel
    }

    printf("[+] Playing 440 Hz Audio Tone through Host Speakers...\n");
    pa_simple_write(s, buffer, NUM_SAMPLES * 2 * sizeof(short), &error);
    pa_simple_drain(s, &error);
    pa_simple_free(s);
    free(buffer);

    printf("==================================================\n");
    printf("[AArch64 Audio Demo] Sound Playback Finished Successfully!\n");
    printf("==================================================\n");
    return 0;
}
