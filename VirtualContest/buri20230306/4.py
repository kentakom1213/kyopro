N = int(input())
A = list(map(int, input().split()))

cnt = 0
st = []

for v in A:
    if st:
        if st[-1][0] == v:
            st[-1][1] += 1
            cnt += 1
            if st[-1][1] == v:
                st.pop()
                cnt -= v
        else:
            st.append([v, 1])
            cnt += 1
    else:
        st.append([v, 1])
        cnt += 1
    
    print(cnt)
