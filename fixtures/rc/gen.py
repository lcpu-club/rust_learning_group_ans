import cyaron
import random
for idx in range(1, 26):
    io = cyaron.IO(file_prefix="test_", data_id=idx,
                   input_suffix=".in", output_suffix=".ans")
    n = 0
    m = 0
    if idx < 5:
        (n, m) = (10, 9)
    elif idx < 10:
        (n, m) = (10, 10)
    elif idx < 15:
        (n, m) = (10, 20)
    else:
        (n, m) = (1000, 5000)
    graph = cyaron.Graph.DAG(n, m)
    io.input_writeln(n)
    deg = [0 for i in range(n)]
    e = {i: [] for i in range(n)}
    for edges in graph.iterate_edges():
        deg[edges.end - 1] += 1
        e[edges.start - 1].append(edges.end - 1)
    order = []
    while len(order) < n:
        for i in range(n):
            if deg[i] == 0:
                order.append(i)
                deg[i] = -1
                for j in e[i]:
                    deg[j] -= 1
    mapp = {order[i]: n-i-1 for i in range(n)}

    dict = {i: [] for i in range(n)}
    for i in range(n):
        dict[i].append(random.randint(0, 2*n))
    for edges in graph.iterate_edges():
        dict[mapp[edges.start - 1]].append(mapp[edges.end - 1])
    for i in range(n):
        io.input_writeln(" ".join(map(str, dict[i])))

    io.output_gen("cargo run --bin rc")
