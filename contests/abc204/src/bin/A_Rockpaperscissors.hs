--         A - Rock-paper-scissors         
-- ----------------------------------------
-- 問題
-- https://atcoder.jp/contests/abc204/tasks/abc204_a
-- ----------------------------------------

solve :: Int -> Int -> Int
solve x y
  | x == y = x
  | otherwise = 3 - x - y

main = do
    [x, y] <- map read . words <$> getLine
    print $ solve x y
