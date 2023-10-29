--             A - 世界のFizzBuzz             
-- ----------------------------------------
-- 問題
-- https://atcoder.jp/contests/abc006/tasks/abc006_1
-- ----------------------------------------

main = do
  n <- getLine
  putStrLn $ if '3' `elem` n || (\x -> read x :: Int) n `mod`3 == 0 then "YES" else "NO"
