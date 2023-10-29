
import random
from pathlib import Path

cd = Path(__file__).parent / "in"

name_N_Q = [
    ("random01", 100, 100),
    ("random02", 200, 200),
    ("random03", 300, 300),
    ("random04", 400, 400),
    ("random05", 500, 500),
    ("random06", 2000, 2000),
    ("random07", 2000, 2000),
    ("random08", 2000, 2000),
]

swap_sorted = lambda a, b: (a, b) if a <= b else (b, a)

for name, N, Q in name_N_Q:

    # Aの生成
    A = " ".join(map(str, (random.randint(1, int(1e9)) for _ in range(N))))

    # queryの生成
    queries = [
        " ".join(map(str, swap_sorted(
            random.randint(1, N), random.randint(1, N)
        ))) for _ in range(Q)
    ]

    write_string = "{} {}\n{}\n{}".format(N, Q, A, "\n".join(queries))

    f = cd / (name + ".txt")
    f.touch(exist_ok=True)
    with f.open("w") as f:
        f.write(write_string)
