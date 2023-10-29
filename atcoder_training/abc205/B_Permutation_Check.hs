--          B - Permutation Check          
-- ----------------------------------------
-- 問題
-- https://atcoder.jp/contests/abc205/tasks/abc205_b
-- ----------------------------------------

import Data.List

main = do
  _ <- getLine
  a_s <- map (\x -> read x :: Int) . words <$> getLine

  putStrLn $ if and [a == i | (a, i) <- zip (sort a_s) [1..]] then "Yes" else "No"
