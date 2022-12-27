# 拡張ユークリッド互除法



## 逆元漸化式

$$
\begin{align}
	p = \left\lfloor \frac{p}{a} \right\rfloor \times a + (p \% a)\\
\end{align}
$$

ここで、両辺に関して$\mod p$ をとると、
$$
\begin{align}
	&\left\lfloor \frac{p}{a} \right\rfloor \times a + (p \% a) \equiv 0\\
	\Leftrightarrow~ &\left\lfloor \frac{p}{a} \right\rfloor + (a\% p)\times a^{-1} \equiv 0\\
	\Leftrightarrow~ &(a\% p)\times a^{-1} \equiv -\left\lfloor \frac{p}{a} \right\rfloor\\
	\Leftrightarrow~ &a^{-1} \equiv -\left\lfloor \frac{p}{a} \right\rfloor \times (a\% p)^{-1}
\end{align}
$$


### 参考

- [よくやる二項係数 (nCk mod. p)、逆元 (a^-1 mod. p) の求め方](https://drken1215.hatenablog.com/entry/2018/06/08/210000)
