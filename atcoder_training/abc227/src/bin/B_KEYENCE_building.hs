--           B - KEYENCE building          
-- ----------------------------------------
-- 問題
-- https://atcoder.jp/contests/abc227/tasks/abc227_b
-- ----------------------------------------

import qualified Data.Set as S

main :: IO ()
main = do
  _ <- getLine
  ss <- map read . words <$> getLine
  print $ solve ss

solve :: [Int] -> Int
solve ss = sum [1 - fromEnum (S.member x set) | x <- ss]

set :: S.Set Int
set = S.fromList [4*a*b + 3*a + 3*b | a <- [1..334], b <- [1..334]]
