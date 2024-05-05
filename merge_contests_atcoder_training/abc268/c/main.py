from collections import Counter

N = int(input())
P = list(map(int, input().split()))

# p_i - i
diff = [P[i] - i for i in range(N)]

# それぞれの個数をカウントする
cnt = Counter(diff)

# k回シフトしたときのスコアを計算
ans = 0
for k in range(N):
    tmp = cnt[k] + cnt[-N+k]
    tmp += cnt[k-1] + cnt[-N+k-1]
    tmp += cnt[k+1] + cnt[-N+k+1]
    
    if ans < tmp:
        ans = tmp

print(ans)
