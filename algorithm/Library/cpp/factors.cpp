// 約数列挙
vector<vector<int>> d(N+1);
for (int i=1; i <= N; i++) {
    for (int j = i; j <= N; j += i) {
        d[j].emplace_back(i);
    }
}