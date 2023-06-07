#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

/// # Safety
/// Use a valid C-String!
void hello(const char *name);

} // extern "C"
