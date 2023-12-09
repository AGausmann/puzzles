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
digitWords = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]

find2 :: [(Int, String)] -> String -> Int
find2 words xs@(x : xt)
  | x `elem` ['1' .. '9'] = digitToInt x
  | not (null wordNums) = head wordNums
  | otherwise = find2 words xt
  where
    wordNums = [i | (i, w) <- words, w `isPrefixOf` xs]
find2 words [] = error "no digit found"

first2 :: String -> Int
first2 = find2 (zip [1 ..] digitWords)

last2 :: String -> Int
last2 = find2 (zip [1 ..] (map reverse digitWords)) . reverse

cal2 :: String -> Int
cal2 x = 10 * first2 x + last2 x

part2 :: String -> Int
part2 = sum . map cal2 . lines

main = do
  inp <- getContents
  (print . part1) inp
  (print . part2) inp
