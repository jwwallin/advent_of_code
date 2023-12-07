module D1p2
  (main, 
  parser, 
  parserHelper, 
  lineParse, 
  parseZero, 
  parseOne, 
  parseTwo, 
  parseThree, 
  parseFour, 
  parseFive, 
  parseSix, 
  parseSeven, 
  parseEight, 
  parseNine
) where

import Prelude

import Data.Foldable (sum)
import Data.Int (fromString)
import Data.List.Partial (head, last)
import Data.Maybe (Maybe(..))
import Data.String.CodeUnits (fromCharArray)
import Effect (Effect)
import Effect.Console (log)
import Node.Encoding (Encoding(..))
import Node.FS.Sync (readTextFile)
import Parsing (Parser, runParser)
import Parsing.Combinators (empty, many, optional, try, (<|>))
import Parsing.String (anyChar, anyCodePoint, char, string)
import Parsing.String.Basic (letter)
import Partial.Unsafe (unsafePartial)

main :: Effect Unit
main = do
  input <- readTextFile UTF8 "data/day1.txt"
  log "Finished."
  let result = runParser input parser
  log $ show result



parserHelper :: Parser String Int
parserHelper = do
  num <-  try parseZero <|> 
          try parseOne <|>
          try parseTwo <|>
          try parseThree <|>
          try parseFour <|>
          try parseFive <|>
          try parseSix <|>
          try parseSeven <|>
          try parseEight <|>
          try parseNine <|> anyChar
  case fromString $ fromCharArray [ num ] of
    Nothing -> empty
    Just i -> pure i

lineParse :: Parser String Int
lineParse = do
  decimalList <- many $ try parserHelper
  _ <- char '\n'
  let firstNum = unsafePartial $ head decimalList
  let lastNum = unsafePartial $ last decimalList
  pure $ firstNum * 10 + lastNum

parser :: Parser String Int
parser = do
  decimalList <- many $ try lineParse
  pure $ sum decimalList

parseZero :: Parser String Char
parseZero = do
  _ <- string "zero" <|> string "0"
  pure '0'
parseOne :: Parser String Char
parseOne = do
  _ <- string "one" <|> string "1"
  pure '1'
parseTwo :: Parser String Char
parseTwo = do
  _ <- string "two" <|> string "2"
  pure '2'
parseThree :: Parser String Char
parseThree = do
  _ <- string "three" <|> string "3"
  pure '3'
parseFour :: Parser String Char
parseFour = do
  _ <- string "four" <|> string "4"
  pure '4'
parseFive :: Parser String Char
parseFive = do
  _ <- string "five" <|> string "5"
  pure '5'
parseSix :: Parser String Char
parseSix = do
  _ <- string "six" <|> string "6"
  pure '6'
parseSeven :: Parser String Char
parseSeven = do
  _ <- string "seven" <|> string "7"
  pure '7'
parseEight :: Parser String Char
parseEight = do
  _ <- string "eight" <|> string "8"
  pure '8'
parseNine :: Parser String Char
parseNine = do
  _ <- string "nine" <|> string "9"
  pure '9'






