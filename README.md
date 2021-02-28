# mersenne-rust

## What are Mersenne Numbers?

To explain what mersenne numbers are:
They are primes that take the form (2^p)-1 where a p is a positive integer.
Although a mersenne number can only be prime if p is another prime number.
The short-hand notation for these large numbers is M[p] where p is the power of two.
The current largest mersenne prime is M82,589,933 which has 24,862,048 digits.
To learn more about mersenne primes follow this link: 
https://www.mersenne.org/
or do the google.

## Important Algorithms

### Lucas-Lehmer Algorithm

There are various ways to know if a mersenne number is prime or not. The first critera is that the value of p must be a prime.
One brute force method to test if a mersenne number is prime is to run it through a prime checker, much like the one used to check if p is prime.
However there is a much better approach to testing these large numbers.

The Lucas-Lehmer Primality Test is test developed initially buy the French mathematician Lucas.
Lucas developed a sequence of numbers following the following rule:
#### s(n) = s(n-1)^2 - 2 where s(0) = 4
In order to prove M[p] is prime, M[p] must be a factor of the (p-1)th number in the sequence Lucas developed.
To make the algorithm faster, the entire Lucas number is not stored, instead, it is stored as s[p-1] % M[p] after s[p] is computed.
After the algorithm is run to the (p-1)th element, the algorithm checks if that last element is 0 or not.
However, the greatest potential for optimization is in the multiplication algorithm used to compute the square required.
This C program uses the divide-and-conquer method for now.
In the future, the Schönhage-Strassen algorithm will be implemented.

Using this method, Lucas proved that M[127] is prime *by hand*.
This mersenne number is the largest ever to be proven prime by hand.
He also proved that M[67] is not prime without ever knowing what other factors besides itself and 1 were.
