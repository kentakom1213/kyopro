# https://atcoder.jp/contests/code-formula-2014-quala/tasks/code_formula_2014_qualA_b

a, b = map(int, input().split())
P = list(map(int, input().split()))
Q = list(map(int, input().split()))

pins = [-1] * 10

for p in P:
    pins[p] = 0

for q in Q:
    pins[q] = 1


def show_pins(pins):
    icon = ".ox"
    s = list(map(lambda c: icon[c], pins))

    print(f"{s[7]} {s[8]} {s[9]} {s[0]}\n {s[4]} {s[5]} {s[6]}\n  {s[2]} {s[3]}\n   {s[1]}")

show_pins(pins)
