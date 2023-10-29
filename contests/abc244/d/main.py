
S = input().split()
T = input().split()

s2t = T.index(S[0]), T.index(S[1]), T.index(S[2])

# 偶置換、奇置換の判定
odd = {
    (0, 2, 1),
    (1, 0, 2),
    (2, 1, 0),
}

if s2t in odd:
    print("No")
else:
    print("Yes")
