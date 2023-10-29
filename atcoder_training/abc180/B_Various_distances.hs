--          B - Various distances          
-- ----------------------------------------
-- 問題
-- https://atcoder.jp/contests/abc180/tasks/abc180_b
-- ----------------------------------------

import GHC.Float (int2Double)

main = do
  _ <- getLine
  xs <- map (\x -> read x :: Int) . words <$> getLine

  print $ sum [abs x | x <- xs]
  print $ sqrt $ sum [int2Double x ** 2 | x <- xs]
  print $ maximum [abs x | x <- xs]
