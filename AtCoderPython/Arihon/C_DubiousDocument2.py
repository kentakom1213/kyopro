#         C - Dubious Document 2
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc076/tasks/abc076_c

# 参考
# https://scrapbox.io/pocala-kyopro/C_-_Dubious_Document_2
# https://ikatakos.com/pot/programming_algorithm/contest_history/atcoder/2017/1028_abc076

# memo: セイウチ演算子でアンパック代入はできない
# AC
# ----------------------------------------

# input
S_ = input()
T = input()

# solve
def match(string, pattern):
    """ if pattern in string -> True """
    i = len(string) - len(pattern)  # pointer of "string"
    while i >= 0:
        for j in range(len(pattern)):  # j is pointer of "pattern"
            # print(i+j, j, string[i+j], pattern[j])  # test
            if string[i+j] == pattern[j] or string[i+j] == "?":
                continue
            else:
                break
        else:
            return (i, i+j)
        i -= 1
    return ()

def replace(string, pattern):
    if match(string, pattern):
        start, end = match(string, pattern)
        result = string[:start] + pattern + string[end+1:]
        return result.replace("?", "a")
    else:
        return "UNRESTORABLE"


print(replace(S_, T))

# print(match(S_, T))  # test
