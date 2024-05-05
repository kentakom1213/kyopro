--              D - Choose Me              
-- ----------------------------------------
-- 問題
-- https://atcoder.jp/contests/abc187/tasks/abc187_d
-- ----------------------------------------

{-
## 演説に行った時
takahashi = a + b
aoki = -a

takahashi - aoki = 2*a + b
-}

import Control.Monad (replicateM)
import Data.List (sort)

main = do
  n <- readLn
  ab <- map (map (\x -> read x :: Int) . words) <$> replicateM n getLine

  print $ map (\[a,b] -> 2*a + b) ab

solve ab adv 
  where
    (a_s, b_s) = unzip ab
    diff = sum a_s - sum b_s
