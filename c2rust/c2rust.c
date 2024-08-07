#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#if defined(DYLIB_PATH)

/*
 * dlopen (runtime-linked) build
 */

// include library for dynamically loading libraries at runtime
#include <dlfcn.h>

// This macro shuffling converts the library path passed
// in `DYLIB_PATH` into a valid C string.
#define STR(s) ISTR(s)
#define ISTR(s) #s

// The result of this macro is the properly quoted path to `libroll.so`.
#define PATH_TO_SHARED_LIBROLL STR(DYLIB_PATH)

// The result of loading the `roll` symbol from `libroll.so` is just a pointer.
// Here we tell the C compiler to treat the raw pointer as a properly typed
// function pointer.
typedef uint16_t (*roll_t)(uint8_t, uint8_t);

#else

/*
 * shared/static build
 */

// No fancy #defines or typedefs. Just declare the function.
// Normally this would go in a header file and be #included, but this is a demo.
uint16_t roll(uint8_t count, uint8_t sides);

#endif

const int EXIT_USAGE = 1;
const int EXIT_RUNTIME = 2;

int main(int argc, const char **argv) {
  uint8_t count = 0;
  uint8_t sides = 0;

  if (argc < 3 || (count = (uint8_t)atoi(argv[1])) < 1 ||
      (sides = (uint8_t)atoi(argv[2])) < 2) {
    fprintf(stderr,
            "Usage: %s COUNT SIDES\n\n"
            "\tCOUNT - must be an integer between 1 and 255"
            "\tSIDES - must be an integer between 2 and 255",
            argv[0]);
    return EXIT_USAGE;
  }

#if defined(DYLIB_PATH)
  // Clear any previous errors
  dlerror();

  // Load the library at the path we were built with, and get a handle back.
  void *libroll_handle = dlopen(PATH_TO_SHARED_LIBROLL, RTLD_LAZY);
  if (libroll_handle == NULL) {
    fprintf(stderr, "ERROR: dlopen() failed: %s\n", dlerror());
    return EXIT_RUNTIME;
  }

  // Look up the location of the `roll()` function
  // and cast it as the function pointer defined above.
  roll_t roll = (roll_t)dlsym(libroll_handle, "roll");
  if (roll == NULL) {
    fprintf(stderr, "ERROR: dlsym() failed: %s\n", dlerror());
    return EXIT_RUNTIME;
  }
#endif

  uint16_t sum = roll(count, sides);
  if (sum == 0) {
    fprintf(stderr, "ERROR: roll() failed!\n");
    return EXIT_RUNTIME;
  }

  printf("%dd%d => %d\n", count, sides, sum);

#if defined(DYLIB_PATH)
  // Dispose of the `dlopen()` handle when we're done.
  dlclose(libroll_handle);
#endif
}
