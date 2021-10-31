#        067 - Base 8 to 9（★2）
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/typical90/tasks/typical90_bo

# 境界条件をよく見極めよう
# ----------------------------------------

# 基数変換に O(log(n))だから全体の計算量は　O(K log(N))

def oct2hept(n):
    dec = int(n, 8)
    res = ""
    while dec:
        res += str(dec % 9)
        dec //= 9
    return res[::-1] if res else "0"

if __name__ == "__main__":
    N, K = map(int, input().split())
    res = str(N)

    for _ in range(K):
        res = oct2hept(res).replace("8", "5")
    
    print(res)