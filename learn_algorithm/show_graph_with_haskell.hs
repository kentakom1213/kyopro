
main = do
  putStrLn "sin-curve"
  mapM_ print $ map (10^) [(+) 50 $ floor $ 50 * sin (n/7) | n <- [1..50]]


