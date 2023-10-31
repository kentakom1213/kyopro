--                B - Pasta                
-- ----------------------------------------
-- 問題
-- https://atcoder.jp/contests/abc241/tasks/abc241_b
-- ----------------------------------------

import Data.List

main = do
  [n, m] <- map (\x -> read x :: Int) . words <$> getLine
  a_s <- map (\x -> read x :: Int) . words <$> getLine
  b_s <- map (\x -> read x :: Int) . words <$> getLine


  putStrLn $ solve(sort a_s) (sort b_s)

solve :: [Int] -> [Int] -> String
solve a_s b_s
  | null b_s = "Yes"
  | null a_s = "No"
  | head a_s == head b_s = solve (tail a_s) (tail b_s)
  | otherwise = solve (tail a_s) b_s

