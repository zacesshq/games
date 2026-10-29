-- Learn to Code: the SQL track. Each level teaches one idea with a short lesson and a few
-- lines of code with blanks marked {1}, {2} and so on. A blank lists the answers that work,
-- a hint and the mistakes people often make, each with a message that explains it. The output
-- is what the finished code prints or does.
-- Keywords may be in any case and runs of spaces count as one. Text in quotes keeps its case.
-- The tables: parts(id, name, volts, kind_id) and kinds(id, name).
-- Answers are compared inside the game; nothing typed is ever run.

return {
  language = [==[SQL]==],
  id = [==[sql]==],
  keep_spaces = true,
  ignore_case = true,
  levels = {
    {
      name = [==[Picking a column]==], topic = [==[Select]==],
      lesson = { [==[SELECT names the columns to show. FROM names the table.]==] },
      code = { [==[{1} name FROM parts;]==] },
      blanks = {
        { answers = { [==[select]==] }, hint = [==[The first word of a query that reads data.]==], wrong = { { [==[get]==], [==[SQL reads data with SELECT.]==] }, { [==[show]==], [==[SHOW lists tables in some databases. Rows are read with SELECT.]==] } } },
      },
      output = { [==[every part's name]==] },
    },
    {
      name = [==[Every column]==], topic = [==[Select]==],
      lesson = { [==[* stands for every column.]==] },
      code = { [==[SELECT {1} FROM parts;]==] },
      blanks = {
        { answers = { [==[*]==] }, hint = [==[One symbol meaning all columns.]==], wrong = { { [==[all]==], [==[ALL is a different keyword. Every column is *.]==] } } },
      },
      output = { [==[every column of every part]==] },
    },
    {
      name = [==[Only some rows]==], topic = [==[Filtering]==],
      lesson = { [==[WHERE keeps only the rows whose test is true.]==] },
      code = { [==[SELECT name FROM parts {1} volts > 5;]==] },
      blanks = {
        { answers = { [==[where]==] }, hint = [==[The keyword that filters rows.]==], wrong = { { [==[if]==], [==[SQL filters rows with WHERE.]==] }, { [==[having]==], [==[HAVING filters groups after GROUP BY. Rows are filtered with WHERE.]==] } } },
      },
      output = { [==[the names of parts above 5 volts]==] },
    },
    {
      name = [==[Two tests at once]==], topic = [==[Filtering]==],
      lesson = { [==[AND needs both tests true. OR needs either.]==] },
      code = { [==[SELECT name FROM parts WHERE volts > 1 {1} volts < 5;]==] },
      blanks = {
        { answers = { [==[and]==] }, hint = [==[Both tests must hold.]==], wrong = { { [==[&&]==], [==[SQL joins tests with the word AND.]==] }, { [==[or]==], [==[OR would keep every part, since every value is above 1 or below 5.]==] } } },
      },
      output = { [==[parts between 1 and 5 volts]==] },
    },
    {
      name = [==[Highest first]==], topic = [==[Sorting]==],
      lesson = { [==[ORDER BY sorts the rows. ASC is lowest first and DESC highest first.]==] },
      code = { [==[SELECT name, volts FROM parts ORDER BY volts {1};]==] },
      blanks = {
        { answers = { [==[desc]==] }, hint = [==[Short for descending.]==], wrong = { { [==[asc]==], [==[ASC puts the lowest first.]==] }, { [==[descending]==], [==[SQL shortens it to DESC.]==] } } },
      },
      output = { [==[parts from the highest voltage down]==] },
    },
    {
      name = [==[Just a few]==], topic = [==[Sorting]==],
      lesson = { [==[LIMIT keeps only the first rows.]==] },
      code = { [==[SELECT name FROM parts ORDER BY volts DESC LIMIT {1};]==] },
      blanks = {
        { answers = { [==[3]==] }, hint = [==[The output is the top three.]==], wrong = { { [==[top 3]==], [==[TOP is SQL Server's form. With LIMIT give just the number.]==] } } },
      },
      output = { [==[the three highest voltage parts]==] },
    },
    {
      name = [==[How many?]==], topic = [==[Aggregates]==],
      lesson = { [==[COUNT(*) counts rows. SUM, AVG, MIN and MAX work on a column.]==] },
      code = { [==[SELECT {1}(*) FROM parts;]==] },
      blanks = {
        { answers = { [==[count]==] }, hint = [==[The function that counts rows.]==], wrong = { { [==[sum]==], [==[SUM adds up values. COUNT counts rows.]==] }, { [==[len]==], [==[len is Python. SQL counts with COUNT.]==] } } },
      },
      output = { [==[the number of parts]==] },
    },
    {
      name = [==[Counting by kind]==], topic = [==[Grouping]==],
      lesson = { [==[GROUP BY puts rows with the same value together so an aggregate runs once for each group.]==] },
      code = { [==[SELECT kind_id, COUNT(*) FROM parts {1} kind_id;]==] },
      blanks = {
        { answers = { [==[group by]==] }, hint = [==[Two words that gather rows into groups.]==], wrong = { { [==[order by]==], [==[ORDER BY only sorts. GROUP BY gathers rows into groups.]==] }, { [==[groupby]==], [==[It is two words: GROUP BY.]==] } } },
      },
      output = { [==[one row for each kind with its count]==] },
    },
    {
      name = [==[Big groups only]==], topic = [==[Grouping]==],
      lesson = { [==[HAVING filters groups after they are made. WHERE filters rows before.]==] },
      code = { [==[SELECT kind_id, COUNT(*) FROM parts GROUP BY kind_id {1} COUNT(*) > 2;]==] },
      blanks = {
        { answers = { [==[having]==] }, hint = [==[The filter for groups.]==], wrong = { { [==[where]==], [==[WHERE runs before grouping, so it cannot test COUNT(*). Use HAVING.]==] } } },
      },
      output = { [==[kinds with more than two parts]==] },
    },
    {
      name = [==[Matching tables]==], topic = [==[Joins]==],
      lesson = { [==[A JOIN puts rows from two tables side by side where the ON test matches.]==] },
      code = { [==[SELECT parts.name, kinds.name]==], [==[FROM parts {1} kinds ON parts.kind_id = kinds.id;]==] },
      blanks = {
        { answers = { [==[join]==], [==[inner join]==] }, hint = [==[The keyword that combines two tables.]==], wrong = { { [==[and]==], [==[Tables are combined with JOIN.]==] }, { [==[with]==], [==[Tables are combined with JOIN.]==] } } },
      },
      output = { [==[each part with its kind's name]==] },
    },
    {
      name = [==[Keeping the unmatched]==], topic = [==[Joins]==],
      lesson = { [==[LEFT JOIN keeps every row of the left table, with empty values where nothing matches.]==] },
      code = { [==[SELECT kinds.name, parts.name]==], [==[FROM kinds {1} parts ON parts.kind_id = kinds.id;]==] },
      blanks = {
        { answers = { [==[left join]==], [==[left outer join]==] }, hint = [==[A join that keeps every kind even with no parts.]==], wrong = { { [==[join]==], [==[A plain JOIN drops kinds that have no parts. LEFT JOIN keeps them.]==] }, { [==[right join]==], [==[RIGHT JOIN keeps every part instead.]==] } } },
      },
      output = { [==[every kind, including those with no parts]==] },
    },
    {
      name = [==[A query inside a query]==], topic = [==[Subqueries]==],
      lesson = { [==[A subquery in brackets works out a value the outer query uses.]==] },
      code = { [==[SELECT name FROM parts]==], [==[WHERE volts > (SELECT {1}(volts) FROM parts);]==] },
      blanks = {
        { answers = { [==[avg]==] }, hint = [==[The function for the average.]==], wrong = { { [==[average]==], [==[SQL shortens it to AVG.]==] }, { [==[mean]==], [==[SQL calls the mean AVG.]==] } } },
      },
      output = { [==[parts above the average voltage]==] },
    },
    {
      name = [==[Adding a row]==], topic = [==[Changing data]==],
      lesson = { [==[INSERT INTO adds a row with VALUES in the order of the columns listed.]==] },
      code = { [==[{1} INTO parts (name, volts) VALUES ('LED', 2.0);]==] },
      blanks = {
        { answers = { [==[insert]==] }, hint = [==[The keyword that adds rows.]==], wrong = { { [==[add]==], [==[SQL adds rows with INSERT INTO.]==] }, { [==[update]==], [==[UPDATE changes rows that are already there.]==] } } },
      },
      output = { [==[a new LED row]==] },
    },
    {
      name = [==[Changing a row]==], topic = [==[Changing data]==],
      lesson = { [==[UPDATE changes rows. SET gives the new values. Always add a WHERE.]==] },
      code = { [==[UPDATE parts {1} volts = 3.3 WHERE id = 4;]==] },
      blanks = {
        { answers = { [==[set]==] }, hint = [==[The keyword that gives new values.]==], wrong = { { [==[=]==], [==[The column and = follow SET.]==] } } },
      },
      output = { [==[part 4 now runs at 3.3 volts]==] },
    },
    {
      name = [==[Removing a row]==], topic = [==[Changing data]==],
      lesson = { [==[DELETE FROM removes rows. Without WHERE it removes them all.]==] },
      code = { [==[{1} FROM parts WHERE id = 4;]==] },
      blanks = {
        { answers = { [==[delete]==] }, hint = [==[The keyword that removes rows.]==], wrong = { { [==[drop]==], [==[DROP removes a whole table. DELETE removes rows.]==] }, { [==[remove]==], [==[SQL removes rows with DELETE.]==] } } },
      },
      output = { [==[part 4 is gone]==] },
    },
    {
      name = [==[Missing values]==], topic = [==[Nulls]==],
      lesson = { [==[NULL means unknown. A test with = NULL is never true; use IS NULL.]==] },
      code = { [==[SELECT name FROM parts WHERE volts IS {1};]==] },
      blanks = {
        { answers = { [==[null]==] }, hint = [==[The word for a missing value.]==], wrong = { { [==[0]==], [==[0 is a value. A missing value is NULL.]==] }, { [==[empty]==], [==[SQL calls a missing value NULL.]==] } } },
      },
      output = { [==[parts with no voltage recorded]==] },
    },
    {
      name = [==[Faster lookups]==], topic = [==[Indexes]==],
      lesson = { [==[An index lets the database find rows by a column without reading the whole table.]==] },
      code = { [==[CREATE {1} idx_parts_name ON parts (name);]==] },
      blanks = {
        { answers = { [==[index]==], [==[unique index]==] }, hint = [==[The kind of object that speeds up lookups.]==], wrong = { { [==[key]==], [==[Keys are declared on a table. A separate lookup structure is an INDEX.]==] } } },
      },
      output = { [==[searches by name are faster]==] },
    },
    {
      name = [==[All or nothing]==], topic = [==[Transactions]==],
      lesson = { [==[A transaction groups changes. COMMIT keeps them all. ROLLBACK undoes them all.]==] },
      code = { [==[BEGIN;]==], [==[UPDATE parts SET volts = 5 WHERE id = 1;]==], [==[UPDATE parts SET volts = 3.3 WHERE id = 2;]==], [==[{1};]==] },
      blanks = {
        { answers = { [==[commit]==] }, hint = [==[The word that keeps the changes.]==], wrong = { { [==[rollback]==], [==[ROLLBACK undoes the changes. COMMIT keeps them.]==] }, { [==[end]==], [==[END works in some databases, but COMMIT is the standard word.]==] } } },
      },
      output = { [==[both changes are kept together]==] },
    },
  },
}
