--  OPEN REQUEST
--  Write a SQL query to get a list of all buildings and the number of open requests (`Requests` in which `status` equals 'Open')

SELECT BuildingName, ISNULL(Count, 0) as 'Count'
FROM Buildings
LEFT JOIN
  (SELECT Apartments.BuildingID, count(*) as 'Count'
    FROM Requests INNER JOIN Apartments
    ON Requests.AptID = Apartments.AptID
    WHERE Requests.Status = 'Open'
    GROUP BY Apartments.BuildingID) ReqCounts
ON ReqCounts.BuildingID = Buildings.BuildingID;

--  This problem uses a straightforward join on `Requests` and `Apartments` to
--  get a list of building IDs and the number of open requests. Once we have
--  this list, we join it again with the `Buildings` table.
--  Queries like this that utilize sub-queries should be thoroughly tested, even
--  when coding by hand. It may be useful to test the inner part of the query
--  first, and then test the other part.
