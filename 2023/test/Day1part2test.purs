module Test.D1p2 (d1p2test) where

import D1p2
import Prelude

import Data.Either (Either(..))
import Effect (Effect)
import Effect.Class.Console (log)
import Parsing (runParser)
import Test.Assert (assertEqual)





d1p2test :: Effect Unit
d1p2test = do
  log "Running Day 1 Part 2 test."
  let input = """1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"""
  let result = runParser input parser 
  assertEqual {actual: result, expected: Right 142}
  let input = "four53lz\n"
  let result = runParser input parser
  assertEqual {actual: result, expected: Right 43}
  let input = """two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"""
  let result = runParser input parser
  assertEqual {actual: result, expected: Right 281}

 
