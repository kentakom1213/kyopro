
if __name__ == "__main__":
    S = input()
    n = len(set(S))

    if n == 1:
        print(1)
    elif n == 2:
        print(3)
    else:
        print(6)