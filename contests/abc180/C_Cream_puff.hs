--              C - Cream puff             
-- ----------------------------------------
-- 問題
-- https://atcoder.jp/contests/abc180/tasks/abc180_c
-- ----------------------------------------

import qualified Data.Set as S

main = do
  n <- readLn
  mapM_ print $ S.fromList $ [i | i <- [1..1010101], n `mod` i == 0] ++ [n `div` i  | i <- [1..1010101], n `mod` i == 0]
  