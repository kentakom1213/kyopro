--             A - Maxi-Buying             
-- ----------------------------------------
-- 問題
-- https://atcoder.jp/contests/abc206/tasks/abc206_a
-- ----------------------------------------

main = do
  n <- readLn
  putStrLn $ solve n 

solve :: RealFrac a => a -> [Char]
solve n
  | price < 206 = "Yey!"
  | price == 206 = "so-so"
  | otherwise = ":("
  where price = floor $ 1.08 * n
