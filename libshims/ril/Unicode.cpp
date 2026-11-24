#include <stdint.h>
#include <stddef.h>
#include <stdlib.h>
#include <string.h>
#include <limits.h>
#include <utils/Unicode.h>

extern "C"
__attribute__((visibility("default")))
char16_t* strdup8to16(const char* src, size_t* out_len) {
    if (out_len == nullptr) {
        return nullptr;
    }

    *out_len = 0;

    if (src == nullptr) {
        return nullptr;
    }

    const auto* input =
        reinterpret_cast<const uint8_t*>(src);

    const size_t input_len = strlen(src);

    const ssize_t measured =
        utf8_to_utf16_length(input, input_len, false);

    if (measured < 0) {
        return nullptr;
    }

    const size_t units = static_cast<size_t>(measured);

    if (units >= static_cast<size_t>(SSIZE_MAX) ||
        units > SIZE_MAX / sizeof(char16_t) - 1) {
        return nullptr;
    }

    auto* result = static_cast<char16_t*>(
        malloc((units + 1) * sizeof(char16_t))
    );

    if (result == nullptr) {
        return nullptr;
    }

    char16_t* end = utf8_to_utf16(
        input,
        input_len,
        result,
        units + 1
    );

    *out_len = static_cast<size_t>(end - result);

    return result;
}
