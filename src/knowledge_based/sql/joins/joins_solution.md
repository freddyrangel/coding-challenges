# JOINS

## Question

What are the different types of joins? Please explain how they differ and why
certain types are better in certain situations.

## Solution

In SQL, the JOIN keyword is used to combine rows from two or more tables based
on a related column between them. There are several types of JOINs available in
SQL:

- INNER JOIN: This type of JOIN returns only the rows that match the join
  condition. An INNER JOIN is the default type of JOIN if the JOIN keyword is
  not specified. An INNER JOIN is usually the most efficient type of JOIN, as it
  only returns the rows that match the join condition. It is the most commonly
  used type of JOIN.

- OUTER JOIN: This type of JOIN returns all rows from both tables, whether or
  not they match the join condition. There are two types of outer JOINs:

- LEFT JOIN or LEFT OUTER JOIN: This type of JOIN returns all rows from the left
  table (the first table in the FROM clause), even if there is no match in the
  right table (the second table in the FROM clause). If there is no match, NULL
  values are returned for right table's columns. A LEFT JOIN is useful when you
  want to include all rows from the left table in the result, even if there is
  no match in the right table.

- RIGHT JOIN or RIGHT OUTER JOIN: This type of JOIN returns all rows from the
right table (the second table in the FROM clause), even if there is no match in
the left table (the first table in the FROM clause). If there is no match, NULL
values are returned for left table's columns. A RIGHT JOIN is similar to a LEFT
JOIN, but the tables are reversed. It is useful when you want to include all
rows from the right table in the result, even if there is no match in the left
table.

- FULL JOIN or FULL OUTER JOIN: This type of JOIN returns all rows from both
tables, whether or not they match the join condition. If there is no match, NULL
values are returned for the non-matching columns. A FULL JOIN is similar to a
LEFT JOIN and a RIGHT JOIN, but it includes all rows from both tables,
regardless of whether there is a match in the other table. It is useful when you
want to include all rows from both tables in the result, regardless of whether
there is a match in the other table.

In summary, INNER JOIN returns only the rows that match the join condition,
while OUTER JOIN returns all rows, including those that do not match the join
condition. The three types of OUTER JOINs (LEFT JOIN, RIGHT JOIN, and FULL JOIN)
differ in which table's rows are included in the result, even if there is no
match in the other table. The type of JOIN you choose will depend on your
specific needs and the data you are working with.
