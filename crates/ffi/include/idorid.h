#ifndef IDORID_H
#define IDORID_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct IdoridVm IdoridVm;
IdoridVm *idorid_vm_new(const uint8_t *apk, size_t len, char **err);
char *idorid_vm_launch(IdoridVm *vm);
char *idorid_vm_click(IdoridVm *vm, int32_t view_id);
char *idorid_vm_set_text(IdoridVm *vm, int32_t view_id, const char *utf8);
char *idorid_vm_back(IdoridVm *vm);
char *idorid_vm_take_log(IdoridVm *vm);
char *idorid_vm_info(IdoridVm *vm);
void idorid_string_free(char *s);
void idorid_vm_free(IdoridVm *vm);

#ifdef __cplusplus
}
#endif
#endif
