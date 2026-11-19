#include <stdint.h>
#include <stdio.h>
#include <fcntl.h>
#include <sys/ioctl.h>
#include <unistd.h>

struct version { int major, minor, patch; uint64_t name_len; char *name; uint64_t date_len; char *date; uint64_t desc_len; char *desc; };
struct getcap { uint64_t cap, value; };
struct getparam { uint64_t param; uint64_t value; };
struct getcaps { uint32_t id, version; uint64_t addr; uint32_t size, pad; };
int main(void) {
    int fd = open("/dev/dri/renderD128", O_RDWR);
    if (fd < 0) { perror("open"); return 1; }
    char name[32] = {0}, caps[8] = {0};
    struct version v = {.name_len = sizeof(name), .name = name};
    uint64_t value = 123;
    struct getparam p = {1, (uint64_t)&value};
    struct getcap drmcap = {5, 123};
    struct getcaps c = {3, 0, (uint64_t)caps, sizeof(caps), 0};
    if (ioctl(fd, 0xc0406400, &v) || ioctl(fd, 0xc010640c, &drmcap) || ioctl(fd, 0xc0106443, &p) || ioctl(fd, 0xc0186449, &c)) { perror("ioctl"); return 1; }
    printf("driver=%s prime=%llu param=%llu caps=%s\n", name, (unsigned long long)drmcap.value, (unsigned long long)value, caps);
    close(fd);
    return 0;
}
