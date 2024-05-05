
# AC

if __name__ == "__main__":
    X = input()
    dot = X.index(".")

    res = int(float(X)) + 1 if int(X[dot+1]) >= 5 else int(float(X))
    print(res)