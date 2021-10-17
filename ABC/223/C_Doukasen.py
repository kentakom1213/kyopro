
# AC

from bisect import bisect_left

N = int(input())
A, B = [], []
for _ in range(N):
    a, b = map(int, input().split())
    A.append(a)
    B.append(b)

lengthS = [0] * (N+1)
timeS = [0] * (N+1)
for i, (a, b) in enumerate(zip(A, B)):
    lengthS[i+1] = lengthS[i] + A[i]
    timeS[i+1] = timeS[i] + a / b

# print(timeS)
# print(lengthS)

half_time = timeS[-1] / 2
half_time_number = bisect_left(timeS, half_time)

# print("half", half_time)
# print("rest", half_time - timeS[half_time_number-1])
rest_rate = (half_time - timeS[half_time_number-1]) / (A[half_time_number-1] / B[half_time_number-1])
rest_length = A[half_time_number-1] * rest_rate

# print(rest_rate)
# print(rest_length)

print(lengthS[half_time_number-1] + rest_length)