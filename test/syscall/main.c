#include <unistd.h>
#include <sys/syscall.h>
#include <stdlib.h>

#include "test.h"

#define SYS_unittest 363

int main(int argc, const char *argv[]) {
    if (argc < 2) {
        THROW_ERROR("usage: ./syscall syscall_num ..\n");
    }

    int syscall_num = atoi(argv[1]);
    int index = atoi(argv[2]);
    if (syscall_num != SYS_unittest) {
        THROW_ERROR("only used for unit test");
    }

    return syscall(syscall_num, UT_CMD_RUNBYINDEX, NULL, index);
}
