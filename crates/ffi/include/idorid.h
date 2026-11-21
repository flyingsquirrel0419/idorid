#ifndef IDORID_H
#define IDORID_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct IdoridVm IdoridVm;
// Callback may be called from any thread running a VM. NULL restores the fallback.
// out receives width and height; flags: bold=1, italic=2, monospace=4,
// max lines in bits 8..15 (0 means unlimited). utf8 is valid during the call.
typedef void (*idorid_measure_text_fn)(const char *utf8, float size, int32_t flags,
                                       float max_width, float out[2]);
void idorid_set_text_measurer(idorid_measure_text_fn fn);
IdoridVm *idorid_vm_new(const uint8_t *apk, size_t len, char **err);
// Directory for app-private data (SharedPreferences); call before launch.
// Data goes to <dir>/<package>/. Returns "{}".
char *idorid_vm_set_data_dir(IdoridVm *vm, const char *dir);
char *idorid_vm_launch(IdoridVm *vm);
char *idorid_vm_set_window_size(IdoridVm *vm, int32_t w, int32_t h);
char *idorid_vm_set_size(IdoridVm *vm, int32_t view_id, int32_t w, int32_t h);
char *idorid_vm_touch(IdoridVm *vm, int32_t view_id, int32_t action, float x, float y);
// Touch in window (content area) coordinates, dispatched through the view tree
// like Android (onInterceptTouchEvent, touch targets). action as above.
char *idorid_vm_touch_window(IdoridVm *vm, int32_t action, float x, float y);
// A native scroll view (ScrollView node) scrolled to (x, y): keeps Java's scroll
// offset in sync for hit testing. Returns "{}" (no UI change).
char *idorid_vm_scroll(IdoridVm *vm, int32_t view_id, int32_t x, int32_t y);
char *idorid_vm_click(IdoridVm *vm, int32_t view_id);
char *idorid_vm_set_text(IdoridVm *vm, int32_t view_id, const char *utf8);
char *idorid_vm_back(IdoridVm *vm);
// Long press on a view.
char *idorid_vm_long_click(IdoridVm *vm, int32_t view_id);
// Return/Done key in an EditText.
char *idorid_vm_editor_action(IdoridVm *vm, int32_t view_id);
// User moved a SeekBar.
char *idorid_vm_set_progress(IdoridVm *vm, int32_t view_id, int32_t progress);
// Timer callback: the UI JSON's "next" field (ms, -1 = none) says when to call.
char *idorid_vm_tick(IdoridVm *vm);
char *idorid_vm_take_log(IdoridVm *vm);
char *idorid_vm_info(IdoridVm *vm);
// PNG/WebP/JPEG bytes of a drawable resource (the UI JSON's "src"); NULL if none.
uint8_t *idorid_vm_resource_bytes(IdoridVm *vm, int32_t id, size_t *len);
void idorid_bytes_free(uint8_t *ptr, size_t len);
void idorid_string_free(char *s);
void idorid_vm_free(IdoridVm *vm);

/* ---- idorid neo: real Android (crates/ffi/src/neo.rs) ---- */
typedef void (*IdoridNeoTextFn)(void *ctx, const uint8_t *text, size_t len);
/* Installs an Android arm64 system image zip into `out` (blocking). 0 = ok. */
int32_t idorid_neo_install(const char *zip, const char *out, const char *work, IdoridNeoTextFn progress, void *ctx);
/* Boots /system/bin/init from `rom`; `overlays` is ':'-separated; `cmdline`
   may be NULL. Console + logcat lines are passed to `log`. 0 = started. */
int32_t idorid_neo_boot(const char *rom, const char *state, const char *overlays, const char *cmdline, IdoridNeoTextFn log, void *ctx);
void idorid_neo_touch(int32_t slot, int32_t id, int32_t x, int32_t y);
uint64_t idorid_neo_instructions(void);

#ifdef __cplusplus
}
#endif
#endif
