--                 A - Star                
-- ----------------------------------------
-- 問題
-- https://atcoder.jp/contests/abc192/tasks/abc192_a
-- ----------------------------------------

main = do
  x <- readLn
  print $ if x `mod` 100 == 0 then 100 else 100 - x `mod` 100
