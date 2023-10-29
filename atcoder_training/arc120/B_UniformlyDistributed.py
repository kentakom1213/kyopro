#        B - Uniformly Distributed
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc120/tasks/arc120_b
# ----------------------------------------

# ..R.
# .R..
# R...

# ↑こんなふうに斜めにRを配置できればいい。
# 斜めのラインは H+W-1 個存在する
# それぞれについて
# |- R, B両方で塗れる     -> 2
# |- RまたはBのみで塗れる  -> 1
# |_ 必ずどちらかが存在    -> 0
# として、掛けていけばよい
# 計算量は O(H+W)

MOD = 998244353

def main():
    H, W = map(int, input().split())
    S = [input() for _ in range(H)]

    ans = 1
    for i in range(H+W-1):
        # 斜めのラインを作成
        diagonal = set()
        for j in range(max(H, W)):
            r, c = i-j, j
            if 0 <= r < H and 0 <= c < W:
                diagonal.add(S[r][c])
        
        # R, B両方で塗れる
        if diagonal == {"."}:
            ans = (ans * 2) % MOD
        elif diagonal == {"R", "B"} or diagonal == {".", "R", "B"}:
            ans = 0
    
    print(ans)


if __name__ == "__main__":
    main()