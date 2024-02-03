"""
Author: Jesse Warner
Date: August 1, 2023

For this challenge, you will practice working with tuples. For each activity to 
succeed, you must get your indexing correct, or you may get an exception and 
crash the program. 
"""

# Tuple containing elements with different data types to work with
tuple1 = ("hi", 47, 3.14, True)

# Create a second tuple by slicing the second and third element from the first 
# tuple.
tuple2 = tuple1[1:2]
# Create a third tuple by concatenating the first and second tuple.
tuple3 = tuple1 + tuple2
# Output all three tuples.
print(tuple1, tuple2, tuple3)
# Output the last element in the first tuple.
print(tuuple1[-1])
# Using indexing, output the number of times the first element from the second 
# tuple occurs in the third tuple (should be more than 1) and output it.
print(tuple3.count(tuple2[0]))
# Find the index of where the second tuple's second element occurs in the 
# first tuple and output it.
print(tuple1.index(tuple2[1]))
# Concatenate the three tuples and output the result
print(tuple1 + tuple2 +tuple3)
# Pack the three tuples into a `result` variable and output it.
result = (tuple1, tuple2, tuple3)
# Output the second tuple repeated 4 times
print(tuple2 * 4)