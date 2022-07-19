# https://atcoder.jp/contests/abc175/tasks/abc175_c

def main():
    x, k, d = map(int, input().split())
    if abs(x) > k*d:
        print(abs(x) - k*d)
        return
    
    pos_min = abs(x) - abs(x)//d*d
    rem = k - abs(x)//d
    if rem & 1:
        print(abs(pos_min - d))
    else:
        print(pos_min)

if __name__ == "__main__":
    main()