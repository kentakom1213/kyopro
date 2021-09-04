# B - AtCoder Quiz
# AC

strings = [input() for _ in range(3)]

contests = ["ABC", "ARC", "AGC", "AHC"]

for s in strings:
    contests.remove(s)

print(contests[0])
