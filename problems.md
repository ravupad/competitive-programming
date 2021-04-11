# Preparation

## March 29

### Rotate Matrix

### Kth Row of Pascal's Triangle

### Merge Intervals

### Add one to number

### N/3 Repeat Number

## March 30

### Max Distance

### Max Non Negative Subarray

### Wave Array

### Max Min

Compare in pairs (a[i], a[i+1]), pick max and min from these

### Grid Unique Paths

### Minimum domino rotations for equal row

### Split Array to minimize largest sum

This can be done by binary searching on the largest sum. For each sum only one scan of array
is required.

## March 31

### Campus Bikes

[Link](https://ttzztt.gitbooks.io/lc/content/sort/bucket-sort/campus-bikes.html)
Create all possible (distance, worker, bike) and sort it. Keep a list of all removed bikes and
workers. Keep removing the top element until a new bike, worker is found.

### Pairwise Hamming Distance


Keep count of 1's at bit position i for all the numbers. Then run a single loop on all the
elements in array.

### Guess the word

There is no determinate solution.

## April 9

### Count Complete Tree Nodes

Do a binary search on the last level. Suppose the last level is at height h.
To access ith child from left at last level, get the representation of i in binary.
For bit = 0, take left, and right for 1. Search for the first child at level i which
is not present. Accessing any child at height h is O(h) operation. Number of such access
requierd would be O(log(2^h)) = h. Total time complexity is h^2.

### Two Sum

Sort the array. Then suppose we have to find solution for [a1, a2, ..., an].
If a1+a2 = n, then solutino is found. 
If a1+an > n => a2 can never be a part of the solution. as an ai > a1 => ai + an > a1+an > n.
So we will search for solution in [a1, ..., an-1].
Similarly if a1+a2 < n, then we will search for solution in [a2, an].

### Compare Strings by Frequency of Smallest Character

Find f(w) for each word in W. Create an array A[i] = words with frequency i. Create an array B[i] = A[i] + ... + A[10]. For each string s answer is B[f(s)+1].

### Shortest way to form a string

Let dp[i] = (min number of substrings required for s[0..i+1], min end index of substring).

dp[0] = (1 if s[0] is present in S, first index where s[0] is present in S).
dp[i] = (dp[i-1].0, x) if s[i] is present at x > after S[dp[i-1].1+1, ..]) else (dp[i-1]+1, first index of s[i] in S).

For each character in S we can keep a map; m[char] = sorted list of occurrence of char in S. Then using binary search for smallest index in the sorted list > dp[i-1].1.

### License Key Formatting

Very Easy

### Brace Expansion II

R("a") = {"a"}

R("{a,b,c}") = {"a","b","c"}

R("{a,b}{c,d}") = {"ac","ad","bc","bd"}

### Expressive Words

Each group of characters can made longer by adding 3 characters or more.

Group all the characters and create an array of changing characters [c0, c1, c2, ...] and keep count of character lengths [n0, n1, n2, ...]. Now suppose a query is given, convert it to the same format [qc0, qc1, ...] and [qn0, qn1, ...]. Query can be stretched to S only if [qc0,...] = [c0,...] and for all i, ((qni = ni) or (qni < ni and ni >= 3)).

### String transforms into another String

Given two strings str1 and str2 of the same length, determine whether you can transform str1 into str2 by doing zero or more conversions.

In one conversion you can convert all occurrences of one character in str1 to any other lowercase English character.

Create another array a[i] = number of unique characters before first occurrence of s[i]. Similarly create b[i] for t[i]. s[i] and t[i] are interchangeable if a[i] = b[i].

### Confusing Number II

We can rotate digits by 180 degrees to form new digits. When 0, 1, 6, 8, 9 are rotated 180 degrees, they become 0, 1, 9, 8, 6 respectively. When 2, 3, 4, 5 and 7 are rotated 180 degrees, they become invalid.

A confusing number is a number that when rotated 180 degrees becomes a different number with each digit valid.(Note that the rotated number can be greater than the original number.)

Given a positive integer N, return the number of confusing numbers between 1 and N inclusive.

At least one of 6 and 9 is required. Any number of 0, 1 and 8 can be used.
f2(n) = number of numbers with digits including 0,1,6,8,9 = 5^n.
f1(n) = number of confusing number which are n digits = 5^n - 3^n
Suppose F(N) = number of confusing number bw 1 and N.
Let N = dn ... d4 d3 d2 d1.
F(N) = sum i = 0 to dn ( if i can't be inverted then 0, else if i = 6 or 9 then f2(n/10) else f1(n/10))

## April 10

### Optimal account balancing

Given a list of transactions between a group of people, return the minimum number of transactions required to settle the debt. Each transaction consits of only two person.

Note that we don't need a graph of transactions. Only a table of overall debt per person is needed and just that needs to be settled. A DFS will be run where A will give all debt to someone (will iterate over someone), ...
We are assuming that A giving all the debt to someone B, should give optimal solution. Whereas it's possible that A giving
debt to multiple people, e.g. B and C might give the optimal solution. But A->B and A->C ~ A->B and A->C with both 2 transactions.

### Smallest string containing all permutations

Given two integer N and D, the task is to find the size of the smallest string that contains all permutations of length N that can be formed using first D digits (0, 1, …, D-1)

This should be solvable using a DFS tranversal. The first character in the smallest string can be any character. Then using DFS find out all the possibilities.

## April 11

### Bulls and Cows

Simple Adhoc.

### Count of smaller numbers after self

* O(n^2)
  by looping for each element one by one.

* O(n^logn)
  by creating a tree using left scan and searching for elements smaller in tree at each step.

* O(n)
  scan from left. for the current index, find the first element at right < than the current.
  value for this current will be 1 + value at that right index.
  during the can keep of stack of increasing numbers.
  When a new number comes pop till a smaller element is at the top and then push the new number at top.

### Logger Rate Limiter

