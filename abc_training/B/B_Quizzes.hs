--               B - Quizzes               
-- ----------------------------------------
-- 問題
-- https://atcoder.jp/contests/abc184/tasks/abc184_b
-- ----------------------------------------

main = do
  [_, x] <- map read . words <$> getLine
  s <- getLine
  print $ foldl (\x c -> max 0 $ x + (if c == 'o' then 1 else -1)) x s
