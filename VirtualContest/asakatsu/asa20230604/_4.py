from string import ascii_lowercase as chars

N = int(input())

res = "a"

def rec(arr, maxi):
    if len(arr) == N:
        print("".join(arr))
        return
    for i in range(maxi + 1):
        rec(arr + chars[i], maxi if i < maxi else maxi + 1)

rec(res, 1)
