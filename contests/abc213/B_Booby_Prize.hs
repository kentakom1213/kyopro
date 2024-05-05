--             B - Booby Prize             
-- ----------------------------------------
-- 問題
-- https://atcoder.jp/contests/abc213/tasks/abc213_b
-- ----------------------------------------

import Data.List

main = do
  n <- readLn
  a_s <- map (\x -> read x :: Int) . words <$> getLine

  print $ snd $ sort (zip a_s [1..]) !! (n - 2)
