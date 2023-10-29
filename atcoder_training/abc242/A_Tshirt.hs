--               A - T-shirt               
-- ----------------------------------------
-- 問題
-- https://atcoder.jp/contests/abc242/tasks/abc242_a
-- ----------------------------------------

import qualified Numeric

main :: IO ()
main = do
  [a, b, c, x] <- map read . words <$> getLine
  putStrLn $ Numeric.showFFloat Nothing (solve a b c x) ""

solve :: (Ord p, Fractional p) => p -> p -> p -> p -> p
solve a b c x
  | x <= a = 1
  | x <= b = c / (b - a)
  | otherwise = 0
