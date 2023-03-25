## What is Genetic Programming? [DRAFT]

Genetic Programming can be seen as a branch of Machine Learning where you solve problems by "evolving" a bunch of computer programs to solve a problem. To my knowledge, it was first introduced by John Koza in his book "".

The principal concepts of Genetic Programming are simple and straightforward:

1. You create a population of random programs which are unfit.
2. You select a subset of this population, usually a majority of them to "marry" eachother and generate new "offsprings". One method is that the indiviudal with the higher "fitness" will have a better chance of getting married. In this repository, however, this selection is completely random.
3. You select a subset of this population, usually a minority of them, to reproduce. Usually very "fit" programs(or individuals) will be chosen for reproduction.
4. You select a few of this population to do small mutations on them.
5. Discard the most unfit programs among this population to have only as many individuals as we had in Step 1
6. You repeat the steps through 2 to 5 till some termination condition is met. Usual conditions are based on generation count, time elasped and reaching some pre-defined fitness.

As you have seen, there is a property, "fitness" which each program has. A fitness is a positive number which indicates how much is a certain program "fit" for the job it is supposed to do. The higher the fitness number is, the better the program is. However too high fitness might mean your programs are "overfit".

## What is Stack Genetic Programming?

There are many ways to represent these programs. One of the traditional methods were Tree data structures. Later, a researcher proposued stacks to represent programs. There are many other ways to represent programs, too. For instance, PADO, EP and GNP use something like Graph data structures.

## What is Symbolic Regresssion?

Symbolic Regression, or SR for short, is one of the problems one can solve with the traditional GP and Stack GP. This repository aims to solve a simple SR problem with Stack GP and with high performance. In plain English, you have a dataset:

```
i = 0, f(i) = 0
i = 1, f(i) = 2
i = 2, f(i) = 4
i = 3, f(i) = 6
```

And you want to find out what is the expression of this "f". In this example, it is obviously `2*i`. But in real world examples the answer is not obvious or a fully accurate answer might not exist at all, or not achievable in a reasonable time. And one of methods to find answers with good accuracy to a SR problem is Genetic Programming.

Rust programming language has been chosen to achieve high performance without needing to deal with memory management stuff like you do in C/C++. In this repository, I haven't used GPU at all. Nevertheless, in real world applications, where there is a big population and a big dataset with at least thousands of datapoints in it, one could benefit from GPU for computing fitness of programs.

I'm told GPU is very perfect for a huge number of jobs each with little or no branching. And computing fitness of programs in GP seem to be one of them.
