def main():
    S, T = input(), input()

    for i, (s, t) in enumerate(zip(S, T), 1):
        if s != t:
            print(i)
            break
    else:
        print(len(T))

main()
