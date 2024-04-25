from ast import Str
from io import StringIO
import subprocess
import numpy as np
import glob

for idx in range(1, 11):
    rng = np.random.default_rng(idx)

    thread_count = rng.choice([4, 8, 16])
    chunk_size = rng.integers(1, 11)

    data_count = thread_count * chunk_size

    n = rng.integers(1, 101, size=data_count)

    chunk = n.reshape(thread_count, -1)
    scatter = chunk.mean(axis=1)

    test_in = StringIO()
    test_ans = StringIO()

    print(thread_count, 0, file=test_in)
    print(" ".join(map(str, n)), file=test_in)

    for i, m in enumerate(scatter):
        print(i, f"{m:.3f}", file=test_ans)

    with open(f"test_{idx*2-1}.in", "w") as f:
        f.write(test_in.getvalue())

    with open(f"test_{idx*2-1}.ans", "w") as f:
        f.write(test_ans.getvalue())

    mean = n.mean()

    # Write the input and output to files
    with open(f"test_{idx*2}.in", "w") as f:
        print(thread_count, 1, file=f)
        print(" ".join(map(str, n)), file=f)
    
    with open(f"test_{idx*2}.ans", "w") as f:
        print(0, f"{mean:.3f}", file=f)


for idx in range(1, 21):
    with open(f"test_{idx}.in") as f_in, open(f"test_{idx}.ans") as f_ans, subprocess.Popen(["cargo", "run", "--release", "--bin", "ring_mean"], stdin=subprocess.PIPE, stdout=subprocess.PIPE) as proc:
        proc.stdin.write(f_in.read().encode())
        proc.stdin.close()

        ans = proc.stdout.read().decode()
        ref = f_ans.read()
        try:
            assert ref == ans
        except AssertionError:
            print(f"Test {idx} failed")
            print("Expected:")
            print(ref)
            print("Got:")
            print(ans)
            break
