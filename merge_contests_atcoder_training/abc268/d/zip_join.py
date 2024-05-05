def zip_join(a, b):
    b.append("")
    n = len(a)
    res = ""
    for i in range(n):
        res += a[i] + b[i]

    return res

if __name__ == "__main__":
    a = ["a", "b", "cd"]
    b = ["__", "_"]

    print( zip_join(a, b) )
