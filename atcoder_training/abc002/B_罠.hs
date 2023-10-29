--                  B - 罠                  
-- ----------------------------------------
-- 問題
-- https://atcoder.jp/contests/abc002/tasks/abc002_2
-- ----------------------------------------

main = do
  w <- getLine
  putStrLn $ [c | c <- w, c/='a', c/='i', c/='u', c/='e', c/='o']
