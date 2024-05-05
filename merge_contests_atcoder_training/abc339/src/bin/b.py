H, W, N = map(int, input().split())

G = [['.'] * W for _ in range(H)]

next = lambda r, c: (c, -r)

cr, cc = 0, 0
dr, dc = -1, 0

for _ in range(N):
    # print(cr, cc, dr, dc)
    if G[cr][cc] == '.':
        G[cr][cc] = '#'
        dr, dc = next(dr, dc)
        cr = (cr + dr) % H
        cc = (cc + dc) % W
    else:
        G[cr][cc] = '.'
        dr, dc = next(*next(*next(dr, dc)))
        cr = (cr + dr) % H
        cc = (cc + dc) % W

for r in G:
    print(''.join(r))

