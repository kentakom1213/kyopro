
if __name__ == "__main__":
    N, Q = map(int, input().split())
    queries = [tuple(map(int, input().split())) for _ in range(Q)]

    trains = [i for i in range(N)]

    for query in queries:
        q, *val = query
        
        if q == 1:
            x, y = val[0]-1, val[1]-1
            trains[x] = y

        elif q == 2:
            x, y = val[0]-1, val[1]-1
            trains[x] = x
        
        else:
            ptr = val[0]-1
            print(ptr+1, end=" ")
            while trains[ptr] != ptr:
                ptr = trains[ptr]
                print(ptr+1, end=" ")
            print()

        print(trains)