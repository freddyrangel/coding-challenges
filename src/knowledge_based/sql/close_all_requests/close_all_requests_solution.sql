--  CLOSE ALL REQUESTS
--  Building #11 is undergoing a major renovation. Implement a query to close
--  all requests from apartments in this building.

UPDATE REQUESTS
SET Status = 'Closed'
WHERE AptID IN (SELECT AptID FROM Apartments WHERE Building ID = 11);

--  UPDATE queries, like SELECT queries, can have WHERE clauses. To implement
--  this query, we get a list of all apartment IDs within building #11 and the
--  list of update requests from those apartments.
