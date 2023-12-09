import Data.Char
import Data.List

-- Part 1
first1 :: String -> Int
first1 (x : xs)
  | x `elem` ['1' .. '9'] = digitToInt x
  | otherwise = first1 xs
first1 [] = error "no digit found"

last1 :: String -> Int
last1 = first1 . reverse

cal1 :: String -> Int
cal1 x = 10 * first1 x + last1 x

part1 :: String -> Int
part1 = sum . map cal1 . lines

-- Part 2

main = do
  inp <- getContents
  (print . part1) inp
