def _diff_one(input_file, actual_file):
    test_name = "test/{}".format(input_file)
    exp_file = "{}.exp".format(input_file)
    native.sh_test(
        name = test_name,
        srcs = ["diff_one.sh"],
        args = [
            "$(location {})".format(exp_file),
            "$(location {})".format(actual_file),
        ],
        data = [
            exp_file,
            actual_file,
        ],
        size = "small",
        tags = [],
    )

    return (test_name, exp_file)

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
    for input_file in input_files:
        genrule_name = "gen_test/{}.actual".format(input_file)
        actual_file = "{}.actual".format(input_file)
        native.genrule(
            name = genrule_name,
            srcs = [input_file],
            outs = [actual_file],
            tools = ["//src:as-tree"],
            cmd = "$(location //src:as-tree) < $(location {input_file}) > $(location {actual_file})".format(
                input_file = input_file,
                actual_file = actual_file,
            ),
            testonly = True,

            # This is manual to avoid being caught with `//...`
            tags = ["manual"],
        )

        (test_name, exp_file) = _diff_one(input_file, actual_file)
        # update_name = _update_one(input_file, actual_file, exp_file)

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
    for input_file in input_files:
        sh_binary_name = input_file[:-3]
        sh_binary_tool = ":{}".format(sh_binary_name)
        native.sh_binary(
            name = sh_binary_name,
            srcs = [input_file],
        )

        genrule_name = "gen_{}.actual".format(input_file)
        actual_file = "{}.actual".format(input_file)
        native.genrule(
            name = genrule_name,
            srcs = [],
            tools = [sh_binary_tool, "//src:as-tree"],
            outs = [actual_file],
            cmd = "$(location {sh_binary_tool}) $(location //src:as-tree) > $(location {actual_file})".format(
                sh_binary_tool = sh_binary_tool,
                actual_file = actual_file,
            ),
            testonly = True,

            # This is manual to avoid being caught with `//...`
            tags = ["manual"],
        )

        (test_name, exp_file) = _diff_one(input_file, actual_file)
        # update_name = _update_one(input_file, actual_file, exp_file)

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
