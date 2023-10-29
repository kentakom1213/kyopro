input()
S = input()

isA, isB, isC = False, False, False

for i, c in enumerate(S):
    if c == 'A':
        isA = True
    if c == 'B':
        isB = True
    if c == 'C':
        isC = True

    if isA and isB and isC:
        print(i + 1)
        break

