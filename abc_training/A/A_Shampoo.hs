--               A - Shampoo               
-- ----------------------------------------
-- 問題
-- https://atcoder.jp/contests/abc243/tasks/abc243_a
-- ----------------------------------------

solve :: Int -> Int -> [Int] -> Int
solve v i family
  | v < family !! i = i
  | otherwise = solve n_v n_i family
  where
    n_v = v - family !! i
    n_i = (i + 1) `mod` 3
        
main :: IO ()
main = do
  inputs @ [v, _, _, _] <- map (\x -> read x :: Int) . words <$> getLine
  let family = tail inputs
  
  putStrLn $ ["F", "M", "T"] !! solve v 0 family
