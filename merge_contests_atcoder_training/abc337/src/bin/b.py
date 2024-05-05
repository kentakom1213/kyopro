import re

S = input()

if re.fullmatch('A*B*C*', S):
    print("Yes")
else:
    print("No")
