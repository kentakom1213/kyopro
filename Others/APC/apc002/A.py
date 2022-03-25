
def main():
    H, W = map(int, input().split())
    A, B = map(int, input().split())
    P, Q = map(int, input().split())
    dr, dc = P-A, Q-B 

    if (dr, dc) == (0, 0):
        return 0

    if H == 1:
        if (dr, dc) in [(1, 0), (-1, -1), (-1, +1)]:
            return 1

        return -1

    if W == 1 and P < A:
        return -1

    if dr < 0:
        if (dr + dc) % 2 == 0:
            if dr >= dc:
                return abs(dr)
    if dr >= 0:
        dr, dc = abs(dr), abs(dc)
        return dr + 2*dc

if __name__ == "__main__":
    T = int(input())
    for _ in range(T):
        print(main())
