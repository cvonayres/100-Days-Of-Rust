## Finding Nemo

You're given a string of words. You need to find the word "Nemo", and return a string like this: `I found Nemo at [the order of the word you find nemo]!`.

If you can't find Nemo, return `I can't find Nemo :(`.

#### Examples

```text
findNemo("I am finding Nemo !") ➞ "I found Nemo at 4!"

findNemo("Nemo is me") ➞ "I found Nemo at 1!"

findNemo("I Nemo am") ➞ "I found Nemo at 2!"
```

---

### Notes

- `! , ? .` are always separated from the last word.
- Nemo will always look like Nemo, and not NeMo or other capital variations.
- Nemo's, or anything that says Nemo with something behind it, doesn't count as Finding Nemo.
- If there are multiple Nemo's in the sentence, only return for the first one.


### Comments 

Really liked this one, rewrote it around 5 times. 

Started with a simple implementation in main with a .split_whitespace() and a for loop with a if statement in it, got it down to the main and a single function using the iter methods and a match and finally in what you see today. 

I wanted to add the tests decided to create a lib and most of the functions there, this allowed me to expand on main [taking insperation from day 1] to allow you to give a word and a sentence and it print the correct response.

Finally split the function in lib.rs into 2 functions so I could implement the quit using the same funcality. 

NOTE: Need to use git more and push before each main change as it would be nice to see the progression.
