
-- A - When?

main = do
  x <- readLn
  putStrLn $ show (21 + x `div` 60) ++ ":" ++ (if x `mod` 60 <= 9 then "0" else "") ++ show (x `mod` 60)

