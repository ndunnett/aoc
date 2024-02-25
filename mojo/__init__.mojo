from python import Python
from time import now


fn fmt(s: String, *vars: String) -> String:
    var f = s

    for v in vars:
        let i = f.find("{}")
        f = f[:i] + v[] + f[i + 2 :]

    return f.replace("\\{", "{").replace("\\}", "}")


fn load_input(year: Int, day: Int) raises -> String:
    """Load puzzle input from cache or download it."""
    Python.add_to_path("/home/dev/repo/python")
    return String(Python.import_module("run").load_input(year, day))


fn format_time(time: Int) raises -> String:
    """Format time into a readable string with units."""
    Python.add_to_path("/home/dev/repo/python")
    return String(Python.import_module("run").format_time(Float64(time) / 1_000_000_000))


fn runner(prepend: String, year: Int, day: Int, solver: fn (input: String) raises -> String) raises:
    """Call solver with given input and print results."""
    let input = load_input(year, day)
    let start = now()
    let result = solver(input)
    print(fmt("{} {} ({})", prepend, result, format_time(now() - start)))
