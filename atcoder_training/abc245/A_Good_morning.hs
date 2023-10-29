--             A - Good morning            
-- ----------------------------------------
-- 問題
-- https://atcoder.jp/contests/abc245/tasks/abc245_a
-- ----------------------------------------


solve :: Int -> Int -> Int -> Int -> String
solve a b c d
  | a == c && b <= d || a < c = "Takahashi"
  | otherwise                 = "Aoki"

main = do
  [a, b, c, d] <- map read . words <$> getLine
  putStrLn $ solve a b c d
