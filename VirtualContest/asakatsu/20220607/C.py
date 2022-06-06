# https://atcoder.jp/contests/agc007/tasks/agc007_a

# 右または下にのみ動くdfs → 全て塗り潰せるか
# 最短経路かどうかを判定すればよい

H, W = map(int, input().split())
print("Posible" if sum(map(lambda c:c=="#", sum([input() for _ in range(H)], ""))) == H+W-1 else "Impossible")
