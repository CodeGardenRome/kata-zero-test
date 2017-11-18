module Katas.KataZero (
  stepA,stepB,stepC,
  stepD,stepE,
  stepF,stepG,stepH,
  stepI,stepJ,
  stepK,stepL,stepM
)  where

import Prelude as P
import Data.Char
import Data.List
import Data.Set as S
import Data.Map as M
import System.Random

stepA :: [Int]
stepA = [1..3]

stepB :: [Int]
stepB = [2, 4..10]

stepC :: [Int]
stepC = [7, 7*2..100]

stepD :: [String] -> [String]
stepD xs = Data.List.filter (beginWithC) xs

beginWithC (c:_) = c == 'c' || c == 'C'

stepE :: Float
modx x = mod x 8
multipli = [(fromIntegral u)|u<-[1..100], 0 == (mod u 8)]
stepE = Data.List.foldl (+) 0 multipli / fromIntegral (length multipli)

stepF :: Int
stepF = Data.List.foldl (+) 0 [u |u<-[6..1000], 0 == mod u 6]

stepG :: [String] -> [String]
stepG xs = Data.List.sort xs

stepH :: IO Int
stepH = return (41) :: IO Int

stepI :: [String] -> String
interv x y = x ++ ", "
stepI xs = P.foldl (interv) "" xs

stepJ :: [String] -> Set String
stepJ xs = S.fromList xs

stepK :: [String] -> Map Int String
stepK xs = M.fromList []

stepL :: [String] -> [Int]
stepL xs = Data.List.map (length) xs

stepM :: [String] -> [String]
stepM xs = P.map (take 1) xs
