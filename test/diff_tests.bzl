def diff_one(input_file, actual_file):
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

    return (test_name, exp_file)

def update_one(input_file, actual_file, exp_file):
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

    return update_name
