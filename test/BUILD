load(":generate_tests.bzl", "fixture_tests", "cli_tests")

fixture_tests(
    input_files = glob(["fixture/**/*.txt"])
)

cli_tests(
    input_files = glob(["cli/**/run.sh"])
)

test_suite(
    name = "test",
    tests = ["fixture", "cli"],
)

test_suite(
    name = "update",
    tests = ["fixture_update", "cli_update"],
)
