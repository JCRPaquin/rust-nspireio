#include <nspireio/nspireio.h>
#include <stdlib.h>

/**
 * If I knew how to cleanly implement these in rust, I would implement them in rust.
 * But I don't, and so they're here to stay until someone submits a pull request.
 *
 * Thanks in advance.
 */

nio_console* alloc_console() {
  return malloc(sizeof(nio_console));
}

void dealloc_console(nio_console *csl) {
  nio_free(csl);
  free(csl);
}
