S = input()
T = input()

edges = {
    "AB": 1,
    "BC": 1,
    "CD": 1,
    "DE": 1,
    "EA": 1,
    "AC": 2,
    "BD": 2,
    "CE": 2,
    "DA": 2,
    "EB": 2,
}

all_edges = {}

for k, v in edges.items():
    all_edges[k] = v
    all_edges[''.join(k[::-1])] = v

if all_edges[S] == all_edges[T]:
    print("Yes")
else:
    print("No")
