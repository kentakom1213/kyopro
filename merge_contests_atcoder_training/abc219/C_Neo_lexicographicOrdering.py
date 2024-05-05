# AC

from string import ascii_lowercase as aslow

def exprint(x): print(*x, sep="\n")

X = input()
N = int(input())
strings = [input() for _ in range(N)]

new_order = {c:aslow[i] for i, c in enumerate(X)}

def decode(string):
    res = ""
    for c in string:
        res += new_order[c]
    return res

strings.sort(key=decode)
exprint(strings)