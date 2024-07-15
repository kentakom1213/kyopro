# https://mojacoder.app/users/shinnshinn/contests/ochacon02/tasks/1

T = int(input())

for _ in range(T):
    m, d, n = map(int, input().split())

    ans = (m + d * (n - 1)) % 12
    print(ans if ans != 0 else 12)
