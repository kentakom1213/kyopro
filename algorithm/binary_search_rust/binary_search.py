from bisect import bisect_left, bisect_right

vec = [1, 5, 5, 10, 11, 11, 15]

def lower_bound(n):
    l = -1
    r = len(vec)
    while (r - l) > 1:
        mid = (l + r) // 2
        if n <= vec[mid]:
            r = mid
        else:
            l = mid
    return r

def upper_bound(n):
    l = -1
    r = len(vec)
    while (r - l) > 1:
        mid = (l + r) // 2
        if n < vec[mid]:
            r = mid
        else:
            l = mid
    return r

print(f"{vec=}")

print("\n# bisect_left")
print(f"{bisect_left(vec, -1)=}")
print(f"{bisect_left(vec, 1)=}")
print(f"{bisect_left(vec, 3)=}")
print(f"{bisect_left(vec, 5)=}")
print(f"{bisect_left(vec, 7)=}")
print(f"{bisect_left(vec, 11)=}")
print(f"{bisect_left(vec, 100)=}")

print("\n# lower_bound")
print(f"{lower_bound(-1)=}")
print(f"{lower_bound(1)=}")
print(f"{lower_bound(3)=}")
print(f"{lower_bound(5)=}")
print(f"{lower_bound(7)=}")
print(f"{lower_bound(11)=}")
print(f"{lower_bound(100)=}")

print("\n# bisect_right")
print(f"{bisect_right(vec, -1)=}")
print(f"{bisect_right(vec, 1)=}")
print(f"{bisect_right(vec, 3)=}")
print(f"{bisect_right(vec, 5)=}")
print(f"{bisect_right(vec, 7)=}")
print(f"{bisect_right(vec, 11)=}")
print(f"{bisect_right(vec, 100)=}")

print("\n# upper_bound")
print(f"{upper_bound(-1)=}")
print(f"{upper_bound(1)=}")
print(f"{upper_bound(3)=}")
print(f"{upper_bound(5)=}")
print(f"{upper_bound(7)=}")
print(f"{upper_bound(11)=}")
print(f"{upper_bound(100)=}")
