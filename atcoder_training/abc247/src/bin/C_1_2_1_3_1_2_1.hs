--            C - 1 2 1 3 1 2 1            
-- ----------------------------------------
-- 問題
-- https://atcoder.jp/contests/abc247/tasks/abc247_c
-- ----------------------------------------

main = do
  n <- readLn
  mapM_ (putStr . (\c -> show c ++ " ")) $ solve n
  putStrLn ""

solve :: (Eq t, Num t) => t -> [t]
solve n
  | n == 1 = [n]
  | otherwise = pre ++ n : pre
  where pre = solve $ n-1
