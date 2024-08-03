#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#if defined(DYNAMIC_ROLL)

// setup for dlopen
#include <dlfcn.h>
typedef uint16_t (*roll_t)(uint8_t, uint8_t);

#if defined(__APPLE__)
#define LIB_FILE "libroll.dylib"
#else
#define LIB_FILE "libroll.so"
#endif

#else

// just include header; linker handles the rest
#include "roll.h"

#endif

const int EXIT_USAGE = 1;
const int EXIT_RUNTIME = 2;

int main(int argc, const char **argv) {

  if (argc < 3) {
    fprintf(stderr, "Usage: %s COUNT SIDES\n", argv[0]);
    return EXIT_USAGE;
  }

  int32_t count_arg = atoi(argv[1]);
  if (count_arg < 1 || count_arg > 255) {
    fprintf(stderr, "Error: COUNT must be between 1 and 255!\n");
    return EXIT_USAGE;
  }

  int32_t sides_arg = atoi(argv[2]);
  if (sides_arg < 2 || sides_arg > 255) {
    fprintf(stderr, "Error: SIDES must be between 2 and 255!\n");
    return EXIT_USAGE;
  }

#ifdef DYNAMIC_ROLL
  dlerror(); // clear errors

  void *dlhandle = dlopen("../target/debug/libroll.so", RTLD_LAZY);
  if (!dlhandle) {
    fprintf(stderr, "Error: dlopen() failed\n");
    return EXIT_RUNTIME;
  }

  roll_t roll = (roll_t)dlsym(dlhandle, "roll");

  char *error = dlerror();
  if (error != NULL) {
    fprintf(stderr, "Error: dlsym() failed\n");
    return EXIT_RUNTIME;
  }
#endif

  uint8_t count = (uint8_t)(count_arg & 0xff);
  uint8_t sides = (uint8_t)(sides_arg & 0xff);
  uint16_t sum = roll(count, sides);
  if (sum == 0) {
    fprintf(stderr, "Error: library returned 0!\n");
    return EXIT_RUNTIME;
  }

  printf("%dd%d => %d\n", count, sides, sum);

#ifdef DYNAMIC_ROLL
  dlclose(dlhandle);
#endif
}
