# 最長共通部分列 (LongestCommonSubsequence) を求める

# 参考
# https://qiita.com/ophhdn/items/dce35806b2458d59e086
# https://maku.blog/p/a3jyhwd/

def exprint(x): print(*x, sep="\n")
def init_array(i, j, val=0): return [[val]*j for _ in range(i)]


### アルゴリズム ###
# dp[i][j] は下記の最大値:
#   * dp[i][j-1]        ＃左から進んでくるケース（文字列 1 から一文字削除）
#   * dp[i-1][j]        ＃上から進んでくるケース（文字列 2 から一文字削除）
#   * dp[i-1][j-1] + 1  ＃LCS の候補とみなした場合（同じ文字の場合のみ）

def getLCS_table(a, b):
    la, lb = len(a), len(b)
    DP = init_array(la+1, lb+1)

    for i in range(la):
        for j in range(lb):
            if a[i] == b[j]:
                DP[i+1][j+1] = DP[i][j] + 1
            else:
                DP[i+1][j+1] = max(DP[i+1][j], DP[i][j+1])

    return DP


# この実装だとダメかな
def getLCS_string(a, b):
    table = getLCS_table(a, b)
    common = []
    counter = 0

    for i in range(1, len(a)+1):
        for j in range(1, len(b)+1):
            if table[i][j] > counter:
                common.append((i-1, j-1))
                counter += 1

    return common


def diff_common(a, b, pad):
    ca, cb = zip(*([(0, 0)] + getLCS_string(a, b) + [(len(a), len(b))]))
    sa, sb = "", ""

    for n in range(1, len(ca)):
        diff_a = (cb[n] - cb[n-1]) - (ca[n] - ca[n-1])
        diff_b = - diff_a
        sa += a[ca[n-1] : ca[n]] + pad * diff_a
        sb += b[cb[n-1] : cb[n]] + pad * diff_b

    print(sa)
    print(sb)
    return sa, sb



if __name__ == "__main__":
    S1 = "東京都", "京都府"
    S2 = "東京工業大学", "京都工芸繊維大学"
    S3 = "kenta komoto", "ryota komoto"
    S4 = "さんごうか", "さんばか"

    # diff_common(*S1, "＿")
    # diff_common(*S2, "＿")
    # diff_common(*S3, "_")
    exprint(S4)
    exprint(getLCS_table(*S4))




    # exprint(get_LCS_table(*S2))
