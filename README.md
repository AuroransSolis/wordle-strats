# Really? Again?

Y'know those things that just haunt your brain periodically? They're not even
things worth worrying about, but they just show up every so often unannounced
and then take up a bunch of your brainpower? Yeah, that's Wordle stuff for me,
and I don't know why.

# How it do

Anyways, this time it's about figuring out whether there's good static
strategies for Wordle. By this, I mean playing `N` words so that on the
`N + 1`th guess you maximise your chances of getting it right. For this, I
apply the following process:

1. Choose one of the unique `N`-long combinations of allowed words.
2. Choose a word in the possible word list to be the secret word.
  1. Apply the strategy to the current secret word to get a list of filters.
  2. Count how many words from the possible solutions list match these filters.
3. Repeat step 2 until the complete list of possible solutions is exhausted and
   take the average and standard deviation of the counts found in 2.a.
4. Repeat steps 1-3 for all possible `N`-long combinations of allowed words.
5. Rank solutions first by their average, and if those are equal, next by their
   standard deviation.

# Other info

For this, it's important to note the distinction between "allowed" words and
"possible" words. "Possible" words are word that Wordle considers valid
solutions, and "allowed" words are words that you are allowed to enter, but
will never be considered a solution (with some exceptions, which need manual
trimming). As of writing this (2025-02-10), there are 1333 "possible" words and
12546 "allowed" words.

Anywho, the problem here is that the number of possible strategies is a
function of the factorial of the number of words in the complete list. More
specifically, `nCk(N, K)` (`N` choose `K`, a.k.a. binomial coefficient) where
`K` is the number of words you allow it to use, and `N` is the number of
allowed and possible words. So for the current (2025-02-10) NYT list of words,
there are 96139911 possible two word strategies. Compare that to the 84000241
pre-NYT. To get these lists, you:

1. Go to the Wordle website.
2. Open Inspect Element
3. Go to the `Debugger` tab (or equivalent)
4. Search all sources for the word `cigar` (the first Wordle solution).

Everything prior to this is in alphabetical order, and is the complete list of
allowed words. Everything to the right of (and including) this is the solutions
list, plus a bunch of garbage(?) data at the end. I haven't put any time into
finding out what it is, and I frankly can't be bothered because that's not
what tickles the brain real good about this whole thing. After you have those
things, it's necessary to run `find-duplicates.sh` on the files (adjust the
paths) to remove any duplicates. It'll save a lot of compute, trust me.

# What has this found?

I ran this on the pre-NYT word lists (`old-possible.txt`, `old-allowed.txt`)
to rank all one and two word strategies. On my desktop (Ryzen 9 7900X), the one
word strategies take ~25 seconds, and the two word strategies took about 5
days. I have since optimised this program and believe that now it would take
around 3-3.5 days to complete.

The top and bottom 10 for the one word strategies:
```
 rank, strat,     avg,     std
    1: roate,  60.425,  51.592
    2: raise,  61.001,  47.419
    3: raile,  61.331,  48.016
    4: soare,  62.301,  52.783
    5: arise,  63.726,  51.137
    6: irate,  63.779,  53.515
    7: orate,  63.891,  53.698
    8: ariel,  65.288,  49.682
    9: arose,  66.021,  52.908
   10: raine,  67.056,  54.850
```
```
12964: xylyl, 876.253, 542.668
12965: susus, 877.973, 594.223
12966: fuffy, 882.869, 596.787
12967: yukky, 885.969, 582.522
12968: gyppy, 892.310, 595.654
12969: jugum, 903.952, 633.088
12970: jujus, 906.639, 542.905
12971: qajaq, 957.337, 509.718
12972: immix, 992.129, 569.974
```
And the top and bottom 10 for the two word strategies:
```
   rank,          strat,     avg,     std
       1: carse + doilt,   4.296,   3.633
       2: riant + socle,   4.314,   4.299
       3: riant + close,   4.344,   4.301
       4: trice + salon,   4.363,   4.077
       5: clint + soare,   4.369,   4.512
       6: cline + roast,   4.371,   4.415
       7: clote + sarin,   4.376,   4.044
       8: roist + lance,   4.384,   4.139
       9: crine + loast,   4.384,   4.219
      10: socle + train,   4.387,   4.109
```
```
84129897: jaffa + qajaq, 774.057, 502.706
84129898: yuppy + puppy, 778.642, 572.089
84129899: cocco + zocco, 792.562, 569.836
84129900: muzzy + yummy, 794.307, 580.934
84129901: muzzy + mummy, 795.077, 579.996
84129902: bubby + buzzy, 817.062, 581.218
84129903: yummy + mummy, 821.478, 586.197
84129904: fuffy + fuzzy, 854.921, 592.432
84129905: jujus + susus, 857.856, 591.040
```
Yes, that is in fact 84 MILLION possible strategies. `nCk` grows fast. As such,
this program will need some adjustment in order to avoid collecting absurd
amounts of data for longer strategies. It's also interesting that the best
one word strategy is not part of the best two word strategy. I have guesses why
that is, but nothing super concrete.

That said, this is unacceptably long. Ideally, this would take under a day to
run. And on that note, I am currently also developing a FPGA implementation of
part of the analysis that I chose for Wordle strategies. I'm telling myself that
it's really a learning opportunity for Clash, floating point on FPGAs, and FPGA
optimisation, but let's be real: it's brainworms.

# Conclusions

Other than that, I think that's everything for this repo so far. Feel free to
keep things up-to-date with the instructions I wrote in for collecting the word
lists from the NYT's website and run the analysis yourself as you see fit - I
can't imagine that the results would change too much from day-to-day updates,
but the ~800 word difference that came with the NYT acquisition I think is
significant enough to warrant a re-run once I can find the time. This readme
will be updated once I have done so.

# License

This project is licensed under the European Union Public License 1.2.
