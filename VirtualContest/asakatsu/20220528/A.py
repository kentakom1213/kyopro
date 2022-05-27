
s = [c=="#" for c in input()] + [c=="#" for c in input()]

if sum(s)==2 and (s[0]&s[3] or s[1]&s[2]):
    print("No")
else:
    print("Yes")
