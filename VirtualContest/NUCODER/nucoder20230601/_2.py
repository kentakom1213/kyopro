S = input()

# スタック
st = []

for c in S:
    if st and st[-1] != c:
        st.pop()
    else:
        st.append(c)

print(len(S) - len(st))
