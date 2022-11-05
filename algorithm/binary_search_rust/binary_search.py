from bisect import bisect_left, bisect_right

vec = [1, 5, 5, 10, 11, 11, 15]

print(f"{bisect_left(vec, -1)=}")
print(f"{bisect_left(vec, 1)=}")
print(f"{bisect_left(vec, 3)=}")
print(f"{bisect_left(vec, 5)=}")
print(f"{bisect_left(vec, 7)=}")
print(f"{bisect_left(vec, 11)=}")
print(f"{bisect_left(vec, 100)=}")

print(f"{bisect_right(vec, -1)=}")
print(f"{bisect_right(vec, 1)=}")
print(f"{bisect_right(vec, 3)=}")
print(f"{bisect_right(vec, 5)=}")
print(f"{bisect_right(vec, 7)=}")
print(f"{bisect_right(vec, 11)=}")
print(f"{bisect_right(vec, 100)=}")
