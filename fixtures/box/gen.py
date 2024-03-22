import cyaron

for idx in range(1, 26):
    io = cyaron.IO(file_prefix="test_", data_id=idx,
                   input_suffix=".in", output_suffix=".ans")
    if 1 <= idx <= 10:
        n = idx
    elif 11 <= idx <= 20:
        n = 50
    else:
        n = 2500
    output = cyaron.Vector.random(n, [(1, 10*n)])
    io.input_writeln(output)
    io.output_gen("cargo run --bin box")
