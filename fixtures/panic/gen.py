import cyaron
import random

for idx in range(1, 6):
    io = cyaron.IO(file_prefix="test_", data_id=idx,
                   input_suffix=".in", output_suffix=".ans")
    num = random.randint(1, 100)
    for _ in range(num):
        io.input_writeln(random.randint(0, 100))
    if idx <= 2:
        io.input_writeln(random.randint(1000, 10000))
    elif idx == 3:
        io.input_writeln(random.randint(-1000, -10))
    elif idx == 4:
        io.input_writeln("abcd")
    else:
        io.input_writeln("rust and genshin impact")
    num = random.randint(1, 100)
    for _ in range(num):
        io.input_writeln(random.randint(0, 100))
    io.output_gen("cargo run --bin panic")
