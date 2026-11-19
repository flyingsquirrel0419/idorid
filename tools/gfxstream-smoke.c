/* Build: cc -Wall -Wextra tools/gfxstream-smoke.c -ldl -o /tmp/gfxstream-smoke
 * Run: VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json \
 *      /tmp/gfxstream-smoke "$HOME/gfx/out/lib/libgfxstream_backend.so"
 * ABI pinned to the revision in build-gfxstream.sh. */
#include <dlfcn.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

struct param { uint64_t key, value; };
struct fence;
static void fence_callback(void *cookie, struct fence *fence) { (void)cookie; (void)fence; }
int main(int argc, char **argv) {
    if (argc != 2) { fprintf(stderr, "usage: %s library\n", argv[0]); return 2; }
    void *lib = dlopen(argv[1], RTLD_NOW | RTLD_LOCAL);
    if (!lib) { fprintf(stderr, "dlopen: %s\n", dlerror()); return 1; }
    int (*init)(struct param *, uint64_t) = dlsym(lib, "stream_renderer_init");
    void (*teardown)(void) = dlsym(lib, "stream_renderer_teardown");
    if (!init || !teardown) { fprintf(stderr, "missing renderer entry point\n"); return 1; }
    int cookie = 0;
    struct param params[] = {{1, (uintptr_t)&cookie}, {2, (1u << 5) | (1u << 3)},
                             {3, (uintptr_t)fence_callback}, {4, 256}, {5, 256}};
    int result = init(params, sizeof(params) / sizeof(params[0]));
    printf("stream_renderer_init=%d\n", result);
    if (result == 0) teardown();
    dlclose(lib);
    return result != 0;
}
