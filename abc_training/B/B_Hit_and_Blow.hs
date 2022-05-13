--             B - Hit and Blow            
-- ----------------------------------------
-- 問題
-- https://atcoder.jp/contests/abc243/tasks/abc243_b
-- ----------------------------------------

main = do
  n <- getLine
  aa <- map (\x -> read x :: Int) . words <$> getLine
  bb <- map (\x -> read x :: Int) . words <$> getLine

  -- 文字とその位置が一致
  print $ sum [1 | (a, b) <- zip aa bb, a == b]

  -- 文字のみが一致
  print $ sum [1 | a <- aa, a `elem` bb] - sum [1 | (a, b) <- zip aa bb, a == b]
