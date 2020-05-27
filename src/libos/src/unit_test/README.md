# Occlum Unit Testing

## Compiling the tests

```OCCLUM_UT``` controls whether ```make``` compiles the unit tests into the
occlum library. To compile the tests, run

```
$ OCCLUM_UT=1 make
```

Crate inventory does not support debug mode, so current unit test framework
only supports release build.

## Running the tests
The unit tests are invoked through `unit_test` in `test`. Each test runs in a
separate process. The tests can be run in different ways:
```
# Run unit_test along with other tests:
$ make test OCCLUM_UT=1

# Only run unit_test
$ make test TESTS=unit_test

# Assign the test_name, like net::test::test_iov, to BIN_ARGS to run specific test
$ make test BIN_ARGS=<test_name> TESTS=unit_test
```

## Adding a new test
Define a new function with the type `fn() -> ()` and add `#[occlum_test]` attribute for it.
Rust built-in attribute `#[should_panic]` without optional parameter is also supported.
