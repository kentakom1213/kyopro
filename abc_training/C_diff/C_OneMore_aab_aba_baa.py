#         C - One More aab aba baa
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc215/tasks/abc215_c

# AC
# ----------------------------------------

def get_perms(string):
    if len(string) == 1: yield string
    else:
        for i, char in enumerate(string):
            rest = get_perms(string[:i] + string[i+1:])
            yield from (char + rest_str for rest_str in rest)

def perms_with_no_duplicate(string):
    return sorted(set(get_perms(string)))


if __name__ == "__main__":
    inputs = input().split()
    S, K = inputs[0], int(inputs[1])

    print(perms_with_no_duplicate(S)[K-1])