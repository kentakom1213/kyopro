def is_ok(h, m):
    sh = f"{h:0>2}"
    sm = f"{m:0>2}"

    new_h = int(sh[0] + sm[0])
    new_m = int(sh[1] + sm[1])

    return 0 <= new_h < 24 and 0 <= new_m < 60

H, M = map(int, input().split())

for i in range(24):
    for j in range(60):
        if is_ok(H, M):
            print(H, M)
            exit()

        M = M + 1
        if M >= 60:
            M = 0
            H += 1
            break

    if H >= 24:
        H = 0
        continue
