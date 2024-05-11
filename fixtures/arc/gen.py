import numpy as np

for idx in range(1, 11):
    rng = np.random.default_rng(idx)
    # Generate a random length array with random integers
    n = rng.integers(1, 101)
    a = rng.integers(-100, 101, n)

    # Calculate the sum of the squares of the array
    ans = np.sum(a ** 2)

    # Write the input and output to files
    with open(f"test_{idx}.in", "w") as f:
        str_a = " ".join(map(str, a))
        print(str_a, file=f)
    
    with open(f"test_{idx}.ans", "w") as f:
        print(ans, file=f)
