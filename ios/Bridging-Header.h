#include "../crates/ffi/include/idorid.h"

typedef void (*idorid_measure_text_fn)(const char *utf8, float size, int32_t flags, float max_width, float out[2]);
void idorid_set_text_measurer(idorid_measure_text_fn fn);
char *idorid_vm_set_window_size(IdoridVm *vm, int32_t w, int32_t h);
