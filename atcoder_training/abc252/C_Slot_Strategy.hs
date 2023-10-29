--            C - Slot Strategy            
-- ----------------------------------------
-- 問題
-- https://atcoder.jp/contests/abc252/tasks/abc252_c
-- ----------------------------------------

import Control.Monad (replicateM)
import Data.Char (chr, ord)

main :: IO ()
main = do
  n <- readLn
  ss <- replicateM n getLine
  print $ foldr min 10000000000 $ map getTime [[getIndex i s | s <- ss] | i <- [0..9]]

-- Int to Char
int2char :: Int -> Char
int2char i = chr (ord '0' + i)

-- 文字列中のcのインデックスを調べる
getIndex :: Int -> String -> Int
getIndex c s = head [i | i <- [0..], s!!i == int2char c]

-- 重複する要素が存在したら10を足した値に置き換える
getTime :: [Int] -> Int
getTime l = foldr max 0 [i - 10 + 10 * length [1 | v <- l, i==v] | i <- l]
