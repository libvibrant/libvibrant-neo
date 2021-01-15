#ifndef LIBVIBRANT_FFI
#define LIBVIBRANT_FFI

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

extern struct vibrant_instance;
typedef struct vibrant_instance vibrant_instance;

typedef enum vibrant_backend {
  XNVCtrl,
  CTM,
} vibrant_backend;

typedef enum vibrant_error {
  Ok,
  OpenDisplay,
  NullName,
  BadName,
  OutOfRange,
} vibrant_error;

enum vibrant_error vibrant_instance_new(const vibrant_instance **_ret);

enum vibrant_error vibrant_instance_from_display_name(const char *name,
                                                      const vibrant_instance **_ret);

void vibrant_instance_free(vibrant_instance *instance);

size_t vibrant_instance_controllers_size(vibrant_instance *instance);

enum vibrant_error vibrant_instance_get_controller_saturation(vibrant_instance *instance,
                                                              size_t idx,
                                                              double *saturation);

enum vibrant_error vibrant_instance_set_controller_saturation(vibrant_instance *instance,
                                                              size_t idx,
                                                              double saturation);

enum vibrant_error vibrant_instance_get_controller_backend(vibrant_instance *instance,
                                                           size_t idx,
                                                           enum vibrant_backend *backend);

enum vibrant_error vibrant_instance_get_controller_name(vibrant_instance *instance,
                                                        size_t idx,
                                                        const unsigned char **str,
                                                        size_t *len);

#endif /* LIBVIBRANT_FFI */
