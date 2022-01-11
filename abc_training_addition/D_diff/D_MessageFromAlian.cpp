//         D - 宇宙人からのメッセージ
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/zone2021/tasks/zone2021_d

// 文字列操作はdeque多い気がする

// AC
// ----------------------------------------

// |S| <= 5*10^5 だから文字列操作をしていると間に合わない
// 反転されているかどうかを保存する変数を持っておく
// deque使えばいける？

#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

int main() {
    string S; cin >> S;
    bool isReversed = false;
    deque<char> deq;
    string res = "";

    for (auto c : S) {
        if (c == 'R') isReversed = !isReversed;
        else {
            if (isReversed) {
                if (!deq.empty() && deq.front() == c) deq.pop_front();
                else deq.push_front(c);
            }
            else {
                if (!deq.empty() && deq.back() == c) deq.pop_back();
                else deq.push_back(c);
            }
        }
    }
    
    if (isReversed) {
        while ( !deq.empty() ) res.push_back(deq.back()), deq.pop_back();
    }
    else {
        while ( !deq.empty() ) res.push_back(deq.front()), deq.pop_front();
    }

    cout << res << endl;
}