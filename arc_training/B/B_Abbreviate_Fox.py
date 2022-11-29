#            B - Abbreviate Fox           
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc108/tasks/arc108_b
# ----------------------------------------

N = int(input())
S = input()

st = []
for c in S:
    st.append(c)
    if len(st) >= 3 and st[-3:] == list("fox"):
        st.pop()
        st.pop()
        st.pop()

print(len(st))
