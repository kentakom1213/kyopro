#           D - Snuke Panic (1D)          
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc266/tasks/abc266_d
# ----------------------------------------

# xから移動可能な範囲

# _ _ _ _ o ]
# _ _ _ o o ]  ]
# _ _ o o o ]1 ]  ]
# _ o o o o ]  ]2 ]  ]
# x o o o o ]  ]  ]3 ]  ]
# _ o o o o    ]  ]  ]4 ]
# _ _ o o o       ]  ]  ]5
# _ _ _ o o          ]  ]
# _ _ _ _ o             ]


from collections import defaultdict

MAX_TIME = 50
N = int(input())

dp = [[0]*5 for _ in range(MAX_TIME)]
sunuke = defaultdict(int)

for i in range(N):
    t, x, a = map(int, input().split())
    sunuke[(t, x)] = a

for t in range(N):
    for x in range(5):

        print()
        arr = [[0]*5 for _ in range(5)]
        arr[0][x] = 1

        # 遷移先
        for dt in range(5):
            for dx in range(-5, 5):
                nt = t + dt
                nx = x + dx

                if nx < 0 or 5 <= nx: continue
                # if abs(dx) > dt: continue

                arr[dt][dx] = (dt, nx)

                # 更新
                dp[nt][nx] = max(
                    dp[nt][nx],
                    dp[t][x] + sunuke[(nt, nx)]
                )
        
        print(*arr, sep="\n")

# print(*dp, sep="\n")
