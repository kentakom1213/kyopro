
from string import ascii_lowercase

def diff_char(s, t):
    return (ascii_lowercase.index(s) - ascii_lowercase.index(t)) % 26


S, T = input(), input()

isOK = True

diff = diff_char(S[0], T[0])
for s, t in zip(S, T):
    isOK &= diff == diff_char(s, t)


print("Yes" if isOK else "No")