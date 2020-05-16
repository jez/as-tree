def _update_one(input_file, actual_file, exp_file):
    update_name = "update_test/{}".format(input_file)
    native.sh_test(
        name = update_name,
        srcs = ["update_one.sh"],
        args = [
            "$(location {})".format(actual_file),
            "$(location {})".format(exp_file),
        ],
        data = [
            actual_file,
            exp_file,
        ],
        size = "small",
        tags = [
            # Avoid being caught with `//...`
            "manual",
            # Forces the test to be run locally, without sandboxing
            "local",
            # Unconditionally run this rule, and don't run in the sandbox
            "external",
        ],
    )

    return update_name

def fixture_tests(input_files):
    tests = []
    updates = []
    for input_txt_file in input_files:
        test_name = "test/{}".format(input_txt_file)
        input_txt_exp_file = "{}.exp".format(input_txt_file)
        native.sh_test(
            name = test_name,
            srcs = ["run_and_diff_one_fixture.sh"],
            args = [
                "$(location {})".format(input_txt_file),
                "$(location {})".format(input_txt_exp_file),
            ],
            data = [
                "//src:as-tree",
                input_txt_file,
                input_txt_exp_file,
            ],
            size = "small",
        )

        # update_name = _update_one(input_txt_file, actual_file, exp_file)

        tests.append(test_name)
        # updates.append(update_name)

    native.test_suite(
        name = "fixture",
        tests = tests,
    )

    native.test_suite(
        name = "fixture_update",
        tests = updates,
        tags = ["manual"],
    )

def cli_tests(input_files):
    tests = []
    updates = []
    for run_sh_file in input_files:
        input_folder, _slash, _file = run_sh_file.rpartition('/')

        sh_binary_name = run_sh_file[:-3]
        native.sh_binary(
            name = sh_binary_name,
            srcs = [run_sh_file],
            data = native.glob([
                "{}/**/*.txt".format(input_folder)
            ]) + ["//src:as-tree"],
        )

        test_name = "test/{}".format(run_sh_file)
        run_sh_exp_file = "{}.exp".format(run_sh_file)
        native.sh_test(
            name = test_name,
            srcs = ["run_and_diff_one_cli.sh"],
            args = [
                "$(location {})".format(sh_binary_name),
                "$(location {})".format(run_sh_exp_file),
            ],
            data = [
                run_sh_exp_file,
                ":{}".format(sh_binary_name),
            ],
            size = "small",
        )
        # update_name = _update_one(run_sh_file, actual_file, run_sh_exp_file)

        tests.append(test_name)
        # updates.append(update_name)

    native.test_suite(
        name = "cli",
        tests = tests,
    )

    native.test_suite(
        name = "cli_update",
        tests = updates,
        tags = ["manual"],
    )
