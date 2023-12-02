module Test.D1p1 (d1p1test) where

import D1p1
import Prelude

import Data.Either (Either(..))
import Effect (Effect)
import Effect.Class.Console (log)
import Parsing (ParseError(..), runParser)
import Test.Assert (assertEqual)

input = """1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"""

d1p1test :: Effect Unit
d1p1test = do
  log "Running Day 1 Part 1 test."
  let result = runParser input parser 
  assertEqual {actual: result, expected: Right 142}
  let input = "four53lz\n"
  let result = runParser input parser
  assertEqual {actual: result, expected: Right 53}

 
