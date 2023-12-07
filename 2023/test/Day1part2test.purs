module Test.D1p2 (d1p2test) where

import D1p2
import Prelude

import Data.Either (Either(..))
import Data.Identity (Identity)
import Effect (Effect)
import Effect.Class.Console (log)
import Parsing (ParserT, runParser)
import Test.Assert (assertEqual)





d1p2test :: Effect Unit
d1p2test = do
  log "Running Day 1 Part 2 test."
  log "Individual number parsing test:"
  log "Test parseOne"
  testParse "1" '1' parseOne
  testParse "one" '1' parseOne
  log "Test parseTwo"
  testParse "2" '2' parseTwo
  testParse "two" '2' parseTwo
  log "Test parseThree"
  testParse "3" '3' parseThree
  testParse "three" '3' parseThree
  log "Test parseFour"
  testParse "4" '4' parseFour
  testParse "four" '4' parseFour
  log "Test parseFive"
  testParse "5" '5' parseFive
  testParse "five" '5' parseFive
  log "Test parseSix"
  testParse "6" '6' parseSix
  testParse "six" '6' parseSix
  log "Test parseSeven"
  testParse "7" '7' parseSeven
  testParse "seven" '7' parseSeven
  log "Test parseEight"
  testParse "8" '8' parseEight
  testParse "eight" '8' parseEight
  log "Test parseNine"
  testParse "9" '9' parseNine
  testParse "nine" '9' parseNine
  log "Test parseZero"
  testParse "0" '0' parseZero
  testParse "zero" '0' parseZero
  log "Individual number parsing test success"

  log "Test parserHelper:"
  log "Test 1/one"
  testParse "1abc" 1 parserHelper
  testParse "oneabc" 1 parserHelper
  log "Test 2/two"
  testParse "2abc" 2 parserHelper
  testParse "twoabc" 2 parserHelper
  log "Test 3/three"
  testParse "3abc" 3 parserHelper
  testParse "threeabc" 3 parserHelper
  log "Test 4/four"
  testParse "4abc" 4 parserHelper
  testParse "fourabc" 4 parserHelper
  log "Test 5/five"
  testParse "5abc" 5 parserHelper
  testParse "fiveabc" 5 parserHelper
  log "Test 6/six"
  testParse "6abc" 6 parserHelper
  testParse "sixabc" 6 parserHelper
  log "Test 7/seven"
  testParse "7abc" 7 parserHelper
  testParse "sevenabc" 7 parserHelper
  log "Test 8/eight"
  testParse "8abc" 8 parserHelper
  testParse "eightabc" 8 parserHelper
  log "Test 9/nine"
  testParse "9abc" 9 parserHelper
  testParse "nineabc" 9 parserHelper
  log "Test 0/zero"
  testParse "0abc" 0 parserHelper
  testParse "zeroabc" 0 parserHelper
  -- This next testParse fails. I'm not certain what should actually happen here...
  -- log "Test no number"
  -- testParse "abc" 0 parserHelper
  log "Test parserHelper success"

  log "Test lineParse:"
  log "Test \"onetwo\\n\" first"
  testParse "onetwo\n" 1 lineParseFirstNum
  log "Test \"onetwo\\n\" last"
  testParse "onetwo\n" 2 lineParseLastNum
  let result = runParser "onetwo\n" lineParseFullArray
  log $ "full array result: " <> show result
  log "Test \"onetwo\\n\""
  testParse "onetwo\n" 12 lineParse
  log "Test \"1two\\n\""
  testParse "1two\n" 12 lineParse
  log "Test \"one2\\n\""
  testParse "one2\n" 12 lineParse
  log "Test \"12\\n\""
  testParse "12\n" 12 lineParse
  log "Test lineParse success"

  -- Next test fails. It seems like the alternation with the anyChar is failing... All the other alternatives do actually work
  let input = "1abc2\n"
  let result = runParser input parser 
  assertEqual {actual: result, expected: Right 12}
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

testParse :: ∀ (s15 ∷ Type) (a16 ∷ Type). Eq a16 ⇒ Show a16 ⇒ s15 → a16 → ParserT s15 Identity a16 → Effect Unit
testParse input expected parserUnderTest = do
  let result = runParser input parserUnderTest 
  assertEqual {actual: result, expected: Right expected}

testParseFails input expected parserUnderTest = do
  let result = runParser input parserUnderTest 
  assertEqual {actual: result, expected: Left expected}

 
