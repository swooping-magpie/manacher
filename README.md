# manacher
A implementation of Manacher's algorithm


# the problem:
Given a string find the longest palindrome substring

# classic solution:
Classic solution is to the consider the 2N+1 possible palindrome centres and to slowly expand out each centre until a palindrome can no longer be formed. This runs in O(N^2) time

# Manacher's solution:
Manacher's solution is a little more complicated: We once again consider the 2N+1 possible palindrome centres as the characters themselves. On each node we guarantee the user that we find the maximum possible palindrome centred around that point. We then loop with the following precondition that at each stage in the loop we have calculated the maximum radiuses of the palindromes centered at point i in the loop and below. 

At each node we find the maximum palindrome starting around that particular point BUT we also check ahead to all nodes that lie within the scope of the palindrome. We then check the mirror version of the node that lies beneath our i. Here we have 3 scenarios:

- Either the mirror palindrome is completely encapsulated in our current palindrome in which case we can set the value of the max palindrome at that node and advance our iterator forward
- If the mirror palindrome is strictly larger than the current palindrome then we also can gaurantee that we have the largest palindrome for this node since the current palindrome is known to be the largest, we cannot have mirror copies on either side larger than the current value. 
- If however the mirror palindrome is stopped at the exact same time as the current palindrome then we have a lack of information in the following sense. Then we break out of this loop and this forms the next centre for which we restart the process. 

This process is linear since it is guaranteed that we jump forward at least once per iteration of either the big loop or the inner loop. 

# Tries solution
To be done.