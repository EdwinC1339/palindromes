# How many palindrome integers are there from 1 to 10<sup>n</sup>?

We can assume that all decimal palindrome integers will take the form  
10<sup>0</sup> * d<sub>0</sub> + 10<sup>1</sup> * d<sub>1</sub> + 10<sup>2</sup> * d<sub>2</sub> ...  
\+ 10<sup>floor(i / 2)</sup> * d<sub>floor(i / 2)</sub> + 10<sup>floor(i / 2 + 1)</sup> * d<sub>floor(i / 2) - 1</sub> ...  
\+ 10<sup>i</sup> * d<sub>0</sub>  
For all i <= n, d<sub>0</sub> in {1,2,3,4,5,6,7,8,9} and all other d<sub>x</sub> in {0,1,2,3,4,5,6,7,8,9}

For example, with n = 5, we may have palindromes of the forms:

| i | Pattern  | Example | 
| - | -------- | --- |
| 1 | d<sub>0</sub>      | 6
| 2 | d<sub>0</sub>d<sub>0</sub>  | 22
| 3 | d<sub>0</sub>d<sub>1</sub>d<sub>0</sub> | 131
| 4 | d<sub>0</sub>d<sub>1</sub>d<sub>1</sub>d<sub>0</sub> | 8448
| 5 | d<sub>0</sub>d<sub>1</sub>d<sub>2</sub>d<sub>1</sub>d<sub>0</sub> | 90909

<sup>Note that i is always equal to the number of digits.</sup>

To identify how many palindromes can be constructed with each pattern, we just need to take the product of the cardinalities of the sets of digits each of them correspond to.

Since d<sub>0</sub> may be any of 9 digits, and all other d<sub>x</sub> may be one of 10 digits, we derive,

p<sub>i</sub> = 9 * 10<sup>j - 1</sup>

Where j is the number of possible different digits (the maximum i in d<sub>i</sub>).

Then, we may derive:

j = ceil(i / 2)

We can substitute that into the expression for p and find:

p<sub>i</sub> = 9 * 10<sup>ceil(i / 2) - 1</sup>

To answer the question, we need to find the sum of p<sub>i</sub> from i = 1 to i = n, so our final answer is

∑<sub><sub>i=1</sub></sub><sup><sup>n</sup></sup> 9⋅10<sup>⌈i /2⌉−1</sup>