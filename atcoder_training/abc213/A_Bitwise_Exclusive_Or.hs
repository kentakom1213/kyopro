--         A - Bitwise Exclusive Or        
-- ----------------------------------------
-- 問題
-- https://atcoder.jp/contests/abc213/tasks/abc213_a
-- ----------------------------------------

-- むずいみたい

import Data.Bits (Bits(xor))

main = do
  -- [a, b] <- map read . words <$> getLine
  print (6 `xor` 10)
