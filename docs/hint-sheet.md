# Hint sheet: solving a cipher

A flembo-cipher code is a simple substitution cipher. Every letter of the phrase
has been swapped for a different letter, and the same swap is used throughout.
Spaces are kept, so you can see how long each word is.

These techniques are listed roughly in the order solvers reach for them. Most
puzzles are solved by combining several: one technique gives you a letter or
two, and those letters make other techniques work.

Examples show code letters in lowercase and real letters in UPPERCASE.

## 1. One-letter words

A one-letter word is almost always **A** or **I**.

- If a phrase has two different one-letter words, one is A and the other is I.
- I is usually followed by a verb, and A by a noun or an adjective.

## 2. Two-letter words

The commonest are **OF, TO, IN, IT, IS, BE, AS, AT, SO, WE, HE, BY, OR, ON, DO,
IF, ME, MY, UP, AN, GO, NO, US**.

- Almost every two-letter word contains a vowel.
- If a one-letter word is solved, its letter often appears in a two-letter word
  too: A gives **AN, AS, AT**, and I gives **IN, IS, IT, IF**.
- Two two-letter words that are each other reversed (`xy` and `yx`) are
  usually **NO** and **ON**.

## 3. Three- and four-letter words

- **THE** and **AND** are by far the most common three-letter words. A
  three-letter word that appears more than once is very likely THE.
- Other common three-letter words: **FOR, ARE, BUT, NOT, YOU, ALL, ANY, CAN, HAD,
  HER, WAS, ONE, OUR, OUT, HIS, HOW, NOW, SEE, TWO, WHO, BOY, DID, ITS, LET, PUT,
  SAY, SHE, TOO, USE**.
- Common four-letter words: **THAT, WITH, HAVE, THIS, WILL, YOUR, FROM, THEY,
  KNOW, WANT, BEEN, GOOD, MUCH, SOME, TIME, VERY, WHEN, COME, HERE, JUST, LIKE,
  LONG, MAKE, MANY, MORE, ONLY, OVER, SUCH, TAKE, THAN, THEM, WELL, WERE, WHAT**.
- Once THE is solved, words starting with the same two code letters are often
  **THEY, THEM, THEN, THERE, THESE, THEIR**.

## 4. Letter frequency

In ordinary English, the most common letters are roughly
**E T A O I N S H R D L U**, in that order. The rarest are **Z, Q, X, J**, then
**K** and **V**.

- The most common code letter in a long phrase is probably E, T or A.
- In short phrases frequency is unreliable, so use it to choose between
  guesses, not to make them.
- **Q** is almost always followed by **U**.
- **H** usually follows **T, S, C** or **W**.

## 5. Word shapes

Write each word's pattern of repeated letters, using a new capital for each new
letter. Words with repeated letters have few possible answers.

| Pattern | Examples |
| --- | --- |
| A B C A | THAT |
| A B C C B D | LETTER |
| A B C B D | NEVER |
| A B C D C | THERE, WHERE |
| A B C A D B | PEOPLE |
| A B B C | BOOK, FEET, TOOK |

A word with no repeated letters matches thousands of words, so leave it until
you have some of its letters.

## 6. Double letters

- The commonest doubles are **LL, EE, SS, OO, TT, FF, RR, NN, PP, CC**.
- **LL, SS, EE** and **FF** often end a word: WILL, LESS, SEE, OFF.
- **AA, II, UU** and **YY** almost never appear, and nor do doubled rare
  letters such as **QQ, JJ, XX**.

## 7. Word endings

The last two letters of a word give a lot away.

- **S** and **E** are the commonest last letters. A code letter that ends many
  words is probably one of them.
- **S** and **E** are also common in second-to-last place: **-ES, -ED, -ER,
  -SE, -ST**.
- Other common endings: **-ING, -ED, -LY, -ER, -EST, -TION, -NESS, -MENT,
  -FUL**.
- If several words end in the same three code letters, try **-ING** first.

## 8. Word beginnings

- **T** is the commonest first letter, then **A, O, S, W, I**.
- Common prefixes: **UN-, RE-, IN-, DIS-, CON-**.

## 9. Vowels

- Every word has a vowel (A, E, I, O, U) or a Y.
- A letter that sits next to many different letters is probably a vowel.
- Letters that only appear next to the same few letters are probably
  consonants.

## 10. No letter stands for itself

flembo-cipher never swaps a letter for itself, so a code `a` is never a real A,
a code `e` is never a real E, and so on. This rules out one option for every
letter.

## 11. Fill in and follow through

Each solved letter can be filled in everywhere it appears. That turns other
words into partly-solved words, which are much easier to guess. Keep going
round the techniques above with the letters you have.

- When a guess leads to an impossible word, undo the guess, not the letters you
  found before it.
- No two code letters stand for the same real letter. If you've already found
  E, no other code letter can be E.

## Note: apostrophes and punctuation

flembo-cipher removes apostrophes and punctuation, so `don't` appears as one
four-letter word (`dont`). Hyphens become spaces, so `well-known` appears as two
words. Contraction tricks that rely on the apostrophe don't apply.

## Using this sheet for grading

Each technique above is something we can measure in a phrase. These are the
inputs for automatic difficulty grading
([issue #4](https://github.com/bojectify/flembo-cipher/issues/4)).

| Technique | What to measure | Easier when |
| --- | --- | --- |
| 1. One-letter words | Number of one-letter words | There are some |
| 2–3. Short common words | Share of words in a common-word list | The share is high |
| 4. Letter frequency | Phrase length, and how closely its letter counts match English | Long and typical |
| 5. Word shapes | For each word, how many dictionary words share its pattern | Few matches |
| 6. Double letters | Number of doubled letters | There are some |
| 7–8. Endings and beginnings | Words with common endings or prefixes; final S or E | Many |
| 9. Vowels | (Not measured directly; supports the others) | n/a |
| 10. No letter stands for itself | Same for every phrase | n/a |
| 11. Follow through | How much each solved word unlocks in the others | Words share many letters |
