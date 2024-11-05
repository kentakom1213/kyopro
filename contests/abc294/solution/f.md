# f問題

あらかじめ $\mathit{CD}$ をソートして二分探索したい

$$
\begin{align*}
    &\frac{a + x}{b + y} < \frac{a + z}{b + w}\\
    \Leftrightarrow~ &(a + x)(b + w) < (a + z)(b + y)\\
    \Leftrightarrow~ &ab + aw + bx + wx < ab + ay + bz + yz\\
    \Leftrightarrow~ &aw + bx + wx < ay + bz + yz\\
\end{align*}
$$

難しそう → 濃度側を動かそう

---

「濃度 $x$ 以上の砂糖水を $k$ 個作れるか？」を高速に判定できればよい．

砂糖 $s$ [g]，水 $t$ [g] のとき濃度は $s / (s + t)$ これが $x$ になるような $s$ は，
$$
\frac{s}{s+t} = x \iff s = t\times\frac{x}{1-x}
$$
となる．ここで $u = tx / (1 - x)$ とおくと，この砂糖水に $u-s$ [g] の砂糖を追加（負の場合は減らす）すれば濃度がちょうど $x$ となる．

