from string import ascii_lowercase
from collections import Counter

S, T = input(), input()

SC = Counter(S)
TC = Counter(T)

is_ok = True

# 全ての文字についてみる
for c in ascii_lowercase:
    if SC[c] != TC[c]:
        if c not in "atcoder":
            is_ok = False
            break
        if SC[c] < TC[c]:
            SC["@"] -= TC[c] - SC[c]
            is_ok &= SC["@"] >= 0
        if SC[c] > TC[c]:
            TC["@"] -= SC[c] - TC[c]
            is_ok &= TC["@"] >= 0

if is_ok:
    print("Yes")
else:
    print("No")
