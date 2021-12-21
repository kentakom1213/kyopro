#         D - Send More Money
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc198/tasks/abc198_d

# 解説
# https://atcoder.jp/contests/abc198/editorial/1050

# これはできたな...

# AC
# ----------------------------------------

# 全ての文字に割り当てて調べてみる (10! = 3628800通り)

gResult = [0, 0, 0]  # 解を保存
def dfs(s1, s2, s3, nums, rest_letters):
    # 全ての文字を代入し終えた場合
    if len(rest_letters) == 0:
        if int(s1) + int(s2) == int(s3):
            gResult[0] = int(s1)
            gResult[1] = int(s2)
            gResult[2] = int(s3)

    else:
        c = rest_letters[0]
        for i, n in enumerate(nums):
            if c == S1[0] and n == 0 \
                or c == S2[0] and n == 0 \
                or c == S3[0] and n == 0:
                continue

            dfs(
                s1.replace(c, str(n)),
                s2.replace(c, str(n)),
                s3.replace(c, str(n)),
                nums[:i] + nums[i+1:],
                rest_letters[1:]
            )


if __name__ == "__main__":
    S1 = input()
    S2 = input()
    S3 = input()

    can_solve = True

    # 文字が11種類以上あるときはUNSOLVABLE
    rest_letters = list(set(S1 + S2 + S3))  # 文字
    can_solve = len(rest_letters) <= 10

    nums = list(range(10))
    dfs(S1, S2, S3, nums, rest_letters)

    if can_solve and sum(gResult) > 0:
        print(*gResult, sep="\n")
    else:
        print("UNSOLVABLE")