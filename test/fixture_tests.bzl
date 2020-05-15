load(":diff_tests.bzl", "diff_one", "update_one")

def fixture_tests(input_files):
    tests = []
    updates = []
    for input_file in input_files:
        genrule_name = "gen_{}.actual".format(input_file)
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

        (test_name, exp_file) = diff_one(input_file, actual_file)
        update_name = update_one(input_file, actual_file, exp_file)

        tests.append(test_name)
        updates.append(update_name)

    native.test_suite(
        name = "fixture",
        tests = tests,
    )

    native.test_suite(
        name = "fixture_update",
        tests = updates,
        tags = ["manual"],
    )
