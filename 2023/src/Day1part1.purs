module D1p1
  ( main,
    parser
  )
  where

import Parsing.Combinators (empty, many, try)
import Parsing.String (char)
import Parsing.String.Basic (digit, letter)
import Partial.Unsafe (unsafePartial)
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

main :: Effect Unit
main = do
  input <- readTextFile UTF8 "data/day1.txt"
  log "Finished."
  let result = runParser input parser
  log $ show result



parserHelper :: Parser String Int
parserHelper = do
  _ <- many letter
  num <- digit
  case fromString $ fromCharArray [ num ] of
    Nothing -> empty
    Just i -> pure i

lineParse :: Parser String Int
lineParse = do
  decimalList <- many $ try parserHelper
  _ <- many letter
  _ <- char '\n'
  let firstNum = unsafePartial $ head decimalList
  let lastNum = unsafePartial $ last decimalList
  pure $ firstNum * 10 + lastNum

parser :: Parser String Int
parser = do
  decimalList <- many $ try lineParse
  pure $ sum decimalList






