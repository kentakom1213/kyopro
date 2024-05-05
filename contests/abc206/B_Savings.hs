--               B - Savings               
-- ----------------------------------------
-- 問題
-- https://atcoder.jp/contests/abc206/tasks/abc206_b
-- ----------------------------------------

{-
n <= k*(k+1)/2 < n+1 を満たすk
-}

main = readLn >>= (print >> solve)

solve n = n * n
-- solve ok ng n
--   | ng - ok < 1 = ok
--   | otherwise = 
--     if n <= mid * (mid+1) `div` 2
--     then solve mid ng n
--     else solve ok mid n
--   where mid = (ok + ng) `div` 2 