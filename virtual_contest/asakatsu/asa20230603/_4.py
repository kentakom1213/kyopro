# https://atcoder.jp/contests/arc010/tasks/arc010_2

DAYS = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
DAYS[1] += 1 # 2012年は閏年

N = int(input())
holidays = set(tuple(map(int, input().split("/"))) for _ in range(N))

# 時系列順に計算
day = 6 # 日曜日
idx = 0
delay = 0 # 振替休日
is_holiday = [False] * 366
for m, days in enumerate(DAYS):
    for d in range(days):
        date = (m + 1, d + 1)

        if date in holidays or day in [5, 6]:
            if date in holidays and day in [5, 6]:
                delay += 1
            is_holiday[idx] = True
        elif delay:
            is_holiday[idx] = True
            delay -= 1

        # 曜日を更新
        day = (day + 1) % 7

        # インデックスを更新
        idx += 1

# 祝日の連続の累積をとる
acc_holiday = [0] * 367

for i in range(366):
    if is_holiday[i]:
        acc_holiday[i + 1] = acc_holiday[i] + 1

print(max(acc_holiday))

