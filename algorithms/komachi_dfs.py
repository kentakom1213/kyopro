def dfs(nums, r=None):
    if r == None:
        r = len(nums)
    if r == 1:
        yield nums[0]
    else:
        for i, v in enumerate(nums):
            rests = dfs(nums[:i] + nums[i+1:], r-1)
            for rest_v in rests:
                # yield [v] + rest_v
                yield v + rest_v
                yield v - rest_v
                yield rest_v - v
                yield v * rest_v

                if v != 0:
                    yield rest_v / v
                if rest_v != 0:
                    yield v / rest_v

if __name__ == "__main__":
    nums = list(map(int, input().split()))
    isOK = False

    res = dfs(nums)
    for v in res:
        isOK |= v == 10
    
    print("Yes" if isOK else "No")