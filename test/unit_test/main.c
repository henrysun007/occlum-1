#include <spawn.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <sys/syscall.h>
#include <sys/wait.h>

#include "test.h"

#define SYS_unittest 363

#define GREEN_OK "\x1b[0;32mok\x1b[0m"
#define RED_FAILED "\x1b[0;31mFAILED\x1b[0m"

// Run one unit test in a process
int run_test(int index) {
    int ret, child_pid, status;
    char syscall_num[5];
    char index_str[32];
    sprintf(syscall_num, "%d", SYS_unittest);
    sprintf(index_str, "%d", index);

    char *client_argv[] = {"syscall", syscall_num, index_str, NULL};
    ret = posix_spawn(&child_pid, "/bin/syscall", NULL, NULL, client_argv, NULL);

    if (ret < 0) {
        THROW_ERROR("ERROR: failed to spawn a child process\n");
    }

    ret = wait4(-1, &status, 0, NULL);
    if (ret < 0) {
        THROW_ERROR("ERROR: failed to wait4 the child process\n");
    }

    return 0;
}

void print_result(int test_count) {
    int passed_test = syscall(SYS_unittest, UT_CMD_GETPASS);
    printf("\ntest result: %s. %d passed; %d failed;\n\n",
           passed_test == test_count ? GREEN_OK : RED_FAILED, passed_test, test_count - passed_test);
}

int main(int argc, const char *argv[]) {
    if (argc == 1) {
        int ret = 0;
        int test_count = syscall(SYS_unittest, UT_CMD_GETCOUNT);
        printf("running %d test%s\n", test_count, test_count == 1 ? "" : "s");

        for (int i = 0; i < test_count; i++) {
            ret = run_test(i);
            if (ret < 0) {
                THROW_ERROR("fail to run one unit test");
            }
        }

        print_result(test_count);
        return 0;
    } else if (argc == 2) {
        printf("running 1 test\n");
        // Run in currrent process when only one test is called
        int ret = syscall(SYS_unittest, UT_CMD_RUNBYNAME, argv[1]);
        print_result(1);
        return ret;
    } else {
        THROW_ERROR("At most one input is accepted.");
    }
}
