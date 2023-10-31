--            A - Digit Machine            
-- ----------------------------------------
-- 問題
-- https://atcoder.jp/contests/abc241/tasks/abc241_a
-- ----------------------------------------

main = do
  a_s <- map read . words <$> getLine
  print $ a_s !! (a_s !! head a_s)
