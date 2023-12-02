module Test.Main where

import Prelude
import Effect (Effect)

import Test.D1p1 (d1p1test)
import Test.D1p2 (d1p2test)


main :: Effect Unit
main = do
  d1p1test
  d1p2test
