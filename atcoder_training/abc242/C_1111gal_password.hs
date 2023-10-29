--           C - 1111gal password          
-- ----------------------------------------
-- 問題
-- https://atcoder.jp/contests/abc242/tasks/abc242_c
-- ----------------------------------------

main = do
  n <- readLn
  print $ (sum . solve $ n) `mod` 998244353

solve :: (Integral a, Eq t, Num t) => t -> [a]
solve n
  | n == 1 = [1, 1, 1, 1, 1, 1, 1, 1, 1]
  | otherwise =
    [
      (dp!!i + (if i>0 then dp!!(i-1) else 0) + (if i<8 then dp!!(i+1) else 0)) `mod` 998244353
      | i <- [0..8]
    ]
  where
    mod_ = 998244353
    dp = solve $ n-1
