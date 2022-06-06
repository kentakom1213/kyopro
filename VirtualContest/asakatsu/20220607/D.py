# https://atcoder.jp/contests/abc147/tasks/abc147_c

# N <= 15 だから、bit全探索で整合性を確かめる

def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

def main():
    N = int(input())
    shogen = init_array(N, N, -1)
    for i in range(N):
        a = int(input())
        for j in range(a):
            x, y = map(int, input().split())
            shogen[i][x-1] = y

    # bit全探索
    ans = 0
    for i in range(1 << N):
        isOK = True
        cnt = 0

        for j in range(N):  # j人目の証言者について調べる
            is_j_honest = (i >> j) & 1
            cnt += is_j_honest
            
            if not is_j_honest:
                continue

            for k in range(N):
                isOK &= shogen[j][k] == -1 or shogen[j][k] == (i >> k) & 1
        
        if isOK:
            ans = max(ans, cnt)
    
    print(ans)

if __name__ == "__main__":
    main()
