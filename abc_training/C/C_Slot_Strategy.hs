--            C - Slot Strategy            
-- ----------------------------------------
-- 問題
-- https://atcoder.jp/contests/abc252/tasks/abc252_c
-- ----------------------------------------

import Control.Monad (replicateM)
import Data.Char (chr, ord)

main = do
  n <- readLn
  ss <- replicateM n getLine
  print $ map (getIndex 8) ss
  print $ [getIndex i s | i <- [0..9], s <- ss]

-- Int to Char
int2char :: Int -> Char
int2char i = chr (ord '0' + i)

-- 文字列中のcのインデックスを調べる
getIndex :: Int -> String -> Int
getIndex c s = head [i | i <- [0..], s!!i == int2char c]

-- ss中のsについて文字kを揃えるのに必要な秒数
getIndexes :: [Int] -> [String -> Int]
getIndexes = map getIndex
