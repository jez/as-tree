load(":diff_tests.bzl", "diff_one", "update_one")

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

        (test_name, exp_file) = diff_one(input_file, actual_file)
        update_name = update_one(input_file, actual_file, exp_file)

        tests.append(test_name)
        updates.append(update_name)

    native.test_suite(
        name = "cli",
        tests = tests,
    )

    native.test_suite(
        name = "cli_update",
        tests = updates,
        tags = ["manual"],
    )
