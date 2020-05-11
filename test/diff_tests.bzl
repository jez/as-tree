def diff_tests(input_files):
    tests = []
    updates = []
    for input_file in input_files:
        genrule_name = "gen_{}.actual".format(input_file)
        actual_file = "{}.actual".format(input_file)
        native.genrule(
            name = genrule_name,
            srcs = [input_file],
            outs = [actual_file],
            tools = ["//main:as-tree"],
            cmd = "$(location //main:as-tree) < $(location {input_file}) > $(location {actual_file})".format(
                input_file = input_file,
                actual_file = actual_file,
            ),
            testonly = True,

            # This is manual to avoid being caught with `//...`
            tags = ["manual"],
        )

        test_name = "test_{}".format(input_file)
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

        update_name = "update_{}".format(input_file)
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

        tests.append(test_name)
        updates.append(update_name)

    native.test_suite(
        name = "test",
        tests = tests,
    )

    native.test_suite(
        name = "update",
        tests = updates,
        tags = ["manual"],
    )
