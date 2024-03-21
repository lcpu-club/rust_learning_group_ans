import cyaron
import random
for idx in range(1, 26):
    io = cyaron.IO(file_prefix="test_", data_id=idx,
                   input_suffix=".in", output_suffix=".ans")
    n = 0
    m = 0
    w = 0
    if idx < 5:
        (n, m, w) = (10, 9, 10)
    elif idx < 10:
        (n, m, w) = (10, 10, 10)
    elif idx < 15:
        (n, m, w) = (10, 20, 10)
    elif idx < 20:
        (n, m, w) = (1000, 5000, 1000)
    else:
        (n, m, w) = (10000, 500000, 1000)
    if idx == 25 or idx == 24:
        graph = cyaron.Graph.hack_spfa(n, weight_limit=w, directed=True)
        m = 0
        for e in graph.iterate_edges():
            m += 1
    else:
        graph = cyaron.Graph.graph(n, m, weight_limit=w, directed=True)
    s = 0

    io.input_writeln(n, m, s)
    for edges in graph.iterate_edges():
        io.input_writeln(edges.start-1, edges.end-1, edges.weight)

    io.output_gen("cargo run --bin refcell")
