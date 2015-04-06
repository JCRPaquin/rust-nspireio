#include <nspireio/nspireio.h>
#include <stdlib.h>

nio_console* alloc_console() {
  return malloc(sizeof(nio_console));
}

void dealloc_console(nio_console *csl) {
  nio_free(csl);
  free(csl);
}
