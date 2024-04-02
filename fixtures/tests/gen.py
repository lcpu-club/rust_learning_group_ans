import cyaron
io = cyaron.IO(file_prefix="test_", data_id=1,
               input_suffix=".in", output_suffix=".ans")
inputs = ["floor", "audio", "crane", "wanna", "hello",
          "boost", "cargo", "loops", "loose", "stood", "boost", "abuse", "wanna", "sleep", "crane"]
anss = ["cargo", "cargo", "cargo", "cargo", "cargo",
        "cargo", "cargo", "boost", "boost", "boost", "boost", "crane", "crane", "crane", "crane"]
n = len(inputs)
io.input_writeln(n)
for (input, ans) in zip(inputs, anss):
    io.input_writeln(input)
    io.input_writeln(ans)
io.output_gen("cargo run --bin tests")
