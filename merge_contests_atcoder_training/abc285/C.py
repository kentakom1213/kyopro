def idx(char):
    """ 大文字アルファベットの番号を返す """
    return ord(char) - ord('A')

S = input()
N = len(S)

# N文字未満の文字列の数
before = sum(26 ** i for i in range(1, N))

after = 0
for i, c in enumerate(S):
    after += idx(c) * 26 ** (N - i - 1)

ans = before + after + 1

print(ans)
