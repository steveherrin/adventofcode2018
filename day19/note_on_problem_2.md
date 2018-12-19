I solved problem 2 by inspecting the registers.

After some initialization, register 5 contains 10551381.

Inspecting register 0, it increments:

* 1
* 4
* 75
* 288
* ...

which by inspection are the sums of the first N prime factors
of 10551381.

The factors are 1, 3, 71, 213, 49537, 148611, 3517127, 10551381

* 1 = 1
* 4 = 1 + 3
* 75 = 1 + 3 + 71
* 288 = 1 + 3 + 71 + 213
* ...

So I'm guessing the program is summing the prime factors,
and so the answer is 14266944.
