-- Learn to Code: the Data Structures and Algorithms track. Each level teaches one idea with a short lesson and a few
-- lines of code with blanks marked {1}, {2} and so on. A blank lists the answers that work,
-- a hint and the mistakes people often make, each with a message that explains it. The output
-- is what the finished code prints or does.
-- These levels work in any language: the code is short pseudocode and most blanks are
-- words or Big O terms. Case does not matter and runs of spaces count as one.
-- Answers are compared inside the game; nothing typed is ever run.

return {
  language = [==[Data Structures and Algorithms]==],
  id = [==[dsa]==],
  keep_spaces = true,
  ignore_case = true,
  levels = {
    {
      name = [==[Reading by index]==], topic = [==[Big O and arrays]==],
      lesson = { [==[An array stores items side by side, so item i is found by arithmetic on its address. Big O describes how time grows with the input size n.]==] },
      code = { [==[x = items[i]   # time: {1}]==] },
      blanks = {
        { answers = { [==[O(1)]==], [==[constant]==] }, hint = [==[The time does not depend on how many items there are.]==], wrong = { { [==[O(n)]==], [==[No loop is needed: the address of item i is worked out directly.]==] } } },
      },
      output = { [==[one step, whatever n is]==] },
    },
    {
      name = [==[Looking at every item]==], topic = [==[Big O and searching]==],
      lesson = { [==[A linear search checks items one by one until it finds the target.]==] },
      code = { [==[for x in items:]==], [==[    if x == target: return True   # worst case: {1}]==] },
      blanks = {
        { answers = { [==[O(n)]==], [==[linear]==] }, hint = [==[In the worst case every item is checked once.]==], wrong = { { [==[O(1)]==], [==[If the target is last or missing, every item is checked.]==] } } },
      },
      output = { [==[n checks in the worst case]==] },
    },
    {
      name = [==[Halving each time]==], topic = [==[Searching]==],
      lesson = { [==[Binary search compares with the middle item and throws half away each step. It only works on sorted data.]==] },
      code = { [==[# items must be {1}]==], [==[mid = (lo + hi) // 2]==] },
      blanks = {
        { answers = { [==[sorted]==], [==[in order]==] }, hint = [==[What must be true of the items before halving makes sense?]==], wrong = { { [==[unique]==], [==[Duplicates are fine. The items must be sorted.]==] } } },
      },
      output = { [==[each step halves the range]==] },
    },
    {
      name = [==[How fast is halving?]==], topic = [==[Searching]==],
      lesson = { [==[Halving n until one item is left takes about log2(n) steps.]==] },
      code = { [==[binary search worst case: {1}]==] },
      blanks = {
        { answers = { [==[O(log n)]==], [==[O(logn)]==], [==[logarithmic]==] }, hint = [==[How many times can you halve n?]==], wrong = { { [==[O(n)]==], [==[Each step throws half away, so far fewer than n steps are needed.]==] } } },
      },
      output = { [==[about 20 steps for a million items]==] },
    },
    {
      name = [==[Abstract data types]==], topic = [==[Abstract data types]==],
      lesson = { [==[An abstract data type says what operations do, not how they are stored. A list, a stack and a map are ADTs; an array or a linked list can implement them.]==] },
      code = { [==[a stack promises push, pop and peek. it is an {1} data type]==] },
      blanks = {
        { answers = { [==[abstract]==] }, hint = [==[Described by what it does, not how it is built.]==], wrong = { { [==[concrete]==], [==[An array is concrete. An ADT is abstract: only its operations are fixed.]==] } } },
      },
      output = { [==[any structure that keeps the promises will do]==] },
    },
    {
      name = [==[Last in, first out]==], topic = [==[Stacks]==],
      lesson = { [==[A stack adds and removes at the same end, the top. The last item pushed is the first popped.]==] },
      code = { [==[push 1, push 2, push 3]==], [==[pop gives {1}]==] },
      blanks = {
        { answers = { [==[3]==] }, hint = [==[The last one pushed.]==], wrong = { { [==[1]==], [==[1 was pushed first, so it comes out last.]==] } } },
      },
      output = { [==[3]==] },
    },
    {
      name = [==[First in, first out]==], topic = [==[Queues]==],
      lesson = { [==[A queue adds at the back and removes from the front, like people waiting in line.]==] },
      code = { [==[enqueue 1, enqueue 2, enqueue 3]==], [==[dequeue gives {1}]==] },
      blanks = {
        { answers = { [==[1]==] }, hint = [==[The first one added.]==], wrong = { { [==[3]==], [==[3 joined last, so it leaves last.]==] } } },
      },
      output = { [==[1]==] },
    },
    {
      name = [==[A chain of nodes]==], topic = [==[Linked lists]==],
      lesson = { [==[A linked list stores each item in a node that points to the next. Adding at the head only changes one pointer.]==] },
      code = { [==[new.next = head]==], [==[head = new   # time: {1}]==] },
      blanks = {
        { answers = { [==[O(1)]==], [==[constant]==] }, hint = [==[Only two pointers change, however long the list is.]==], wrong = { { [==[O(n)]==], [==[No walk through the list is needed to add at the head.]==] } } },
      },
      output = { [==[constant time]==] },
    },
    {
      name = [==[Hashing]==], topic = [==[Hash tables]==],
      lesson = { [==[A hash function turns a key into a number that picks a bucket. Equal keys must give equal hashes. Different keys can share a bucket: a collision.]==] },
      code = { [==[two different keys landing in the same bucket is a {1}]==] },
      blanks = {
        { answers = { [==[collision]==] }, hint = [==[When keys bump into each other.]==], wrong = { { [==[clash]==], [==[The usual term is a collision.]==] } } },
      },
      output = { [==[handled by chaining or probing]==] },
    },
    {
      name = [==[Fast lookups]==], topic = [==[Hash tables]==],
      lesson = { [==[With a good hash function and room to spare, a hash table finds a key without searching.]==] },
      code = { [==[average lookup in a hash table: {1}]==] },
      blanks = {
        { answers = { [==[O(1)]==], [==[constant]==] }, hint = [==[Average case with few collisions.]==], wrong = { { [==[O(log n)]==], [==[That is a balanced tree. A hash table jumps straight to the bucket.]==] } } },
      },
      output = { [==[constant on average, O(n) if everything collides]==] },
    },
    {
      name = [==[Each one once]==], topic = [==[Sets]==],
      lesson = { [==[A set holds each value at most once and answers is x in it quickly. Turning a list into a set removes duplicates.]==] },
      code = { [==[seen = set()]==], [==[for x in items:]==], [==[    if x in seen: return True   # found a {1}]==], [==[    seen.add(x)]==] },
      blanks = {
        { answers = { [==[duplicate]==], [==[repeat]==] }, hint = [==[A value that appears twice.]==], wrong = { { [==[collision]==], [==[A collision is two keys in one bucket. Here the same value appears twice: a duplicate.]==] } } },
      },
      output = { [==[finds a repeated item in O(n)]==] },
    },
    {
      name = [==[Counting words]==], topic = [==[Maps]==],
      lesson = { [==[A map (dictionary) stores key and value pairs. Counting how often each word appears is a classic use.]==] },
      code = { [==[counts = map()]==], [==[for w in words:]==], [==[    counts[w] = counts.get(w, {1}) + 1]==] },
      blanks = {
        { answers = { [==[0]==] }, hint = [==[The count for a word not seen yet.]==], wrong = { { [==[1]==], [==[Starting at 1 and then adding 1 would count every word twice on its first visit.]==] } } },
      },
      output = { [==[each word with its number of uses]==] },
    },
    {
      name = [==[Sorted on the way in]==], topic = [==[Trees]==],
      lesson = { [==[In a binary search tree smaller keys go left and larger keys go right. Visiting left, node, right gives the keys in order.]==] },
      code = { [==[in-order traversal of a BST gives the keys {1}]==] },
      blanks = {
        { answers = { [==[sorted]==], [==[in order]==], [==[ascending]==] }, hint = [==[In what order?]==], wrong = { { [==[reversed]==], [==[Left, node, right gives smallest to largest.]==] } } },
      },
      output = { [==[1 3 4 6 8]==] },
    },
    {
      name = [==[The smallest on top]==], topic = [==[Heaps]==],
      lesson = { [==[A min-heap keeps the smallest item at the root. Taking it out and fixing the heap costs O(log n).]==] },
      code = { [==[the root of a min-heap is the {1} item]==] },
      blanks = {
        { answers = { [==[smallest]==], [==[minimum]==] }, hint = [==[Min heap.]==], wrong = { { [==[largest]==], [==[That is a max-heap.]==] } } },
      },
      output = { [==[used for priority queues]==] },
    },
    {
      name = [==[Level by level]==], topic = [==[Graphs]==],
      lesson = { [==[Breadth-first search visits all neighbours before going deeper. It finds the fewest edges to each node.]==] },
      code = { [==[BFS keeps the nodes to visit in a {1}]==] },
      blanks = {
        { answers = { [==[queue]==] }, hint = [==[First in, first out.]==], wrong = { { [==[stack]==], [==[A stack gives depth-first search. BFS uses a queue.]==] } } },
      },
      output = { [==[visits nodes in order of distance]==] },
    },
    {
      name = [==[As deep as possible]==], topic = [==[Graphs]==],
      lesson = { [==[Depth-first search follows one path as far as it goes, then backs up. Recursion or a stack does the backing up.]==] },
      code = { [==[DFS keeps the nodes to visit in a {1}]==] },
      blanks = {
        { answers = { [==[stack]==] }, hint = [==[Last in, first out.]==], wrong = { { [==[queue]==], [==[A queue gives breadth-first search. DFS uses a stack.]==] } } },
      },
      output = { [==[explores a whole branch before the next]==] },
    },
    {
      name = [==[Shortest paths]==], topic = [==[Graphs]==],
      lesson = { [==[Dijkstra's algorithm finds shortest paths when no edge weight is negative. It always expands the closest unfinished node next.]==] },
      code = { [==[Dijkstra picks the closest node with a {1}]==] },
      blanks = {
        { answers = { [==[priority queue]==], [==[heap]==], [==[min-heap]==] }, hint = [==[A structure that always gives the smallest first.]==], wrong = { { [==[queue]==], [==[A plain queue ignores distances. Dijkstra needs a priority queue.]==] } } },
      },
      output = { [==[O((V + E) log V) with a binary heap]==] },
    },
    {
      name = [==[Calling itself]==], topic = [==[Recursion]==],
      lesson = { [==[A recursive function calls itself on a smaller problem. A base case stops it.]==] },
      code = { [==[def fact(n):]==], [==[    if n <= {1}: return 1]==], [==[    return n * fact(n - 1)]==] },
      blanks = {
        { answers = { [==[1]==], [==[0]==] }, hint = [==[The smallest n whose factorial is 1.]==], wrong = { { [==[-1]==], [==[fact(0) would then call fact(-1) and the result would be 0.]==] } } },
      },
      output = { [==[fact(5) is 120]==] },
    },
    {
      name = [==[Swapping neighbours]==], topic = [==[Sorting]==],
      lesson = { [==[Bubble sort and insertion sort compare items pairwise in nested loops.]==] },
      code = { [==[bubble sort, worst case: {1}]==] },
      blanks = {
        { answers = { [==[O(n^2)]==], [==[O(n²)]==], [==[O(n*n)]==], [==[quadratic]==] }, hint = [==[A loop inside a loop over n items.]==], wrong = { { [==[O(n log n)]==], [==[That is merge sort. Nested loops over n give n squared.]==] } } },
      },
      output = { [==[fine for tiny lists only]==] },
    },
    {
      name = [==[Split and merge]==], topic = [==[Sorting]==],
      lesson = { [==[Merge sort splits the list in half, sorts each half and merges them. Quick sort partitions around a pivot.]==] },
      code = { [==[merge sort, every case: {1}]==] },
      blanks = {
        { answers = { [==[O(n log n)]==], [==[O(nlogn)]==], [==[O(n log(n))]==] }, hint = [==[log n levels of splitting, n work to merge on each.]==], wrong = { { [==[O(n^2)]==], [==[That is quick sort's worst case. Merge sort is always n log n.]==] } } },
      },
      output = { [==[the usual best for comparison sorts]==] },
    },
    {
      name = [==[Built-in sorting]==], topic = [==[Sorting]==],
      lesson = { [==[Libraries sort for you: sorted() in Python, Arrays.sort and Collections.sort in Java. Objects sort by a comparison you give.]==] },
      code = { [==[Collections.{1}(names)   // Java]==] },
      blanks = {
        { answers = { [==[sort]==] }, hint = [==[The method name.]==], wrong = { { [==[sorted]==], [==[sorted is Python. In Java Collections.sort sorts a list in place.]==] } } },
      },
      output = { [==[names in alphabetical order]==] },
    },
    {
      name = [==[Two pointers]==], topic = [==[Two pointers]==],
      lesson = { [==[In a sorted list, two pointers from each end can find a pair with a given sum in one pass.]==] },
      code = { [==[if a[l] + a[r] < target: l += 1   # the sum is too {1}]==], [==[else: r -= 1]==] },
      blanks = {
        { answers = { [==[small]==], [==[low]==] }, hint = [==[Moving the left pointer right makes the sum bigger.]==], wrong = { { [==[big]==], [==[When the sum is too big the right pointer moves left instead.]==] } } },
      },
      output = { [==[O(n) instead of O(n^2)]==] },
    },
    {
      name = [==[A moving window]==], topic = [==[Sliding window]==],
      lesson = { [==[To sum every run of k items, add the item that enters the window and subtract the one that leaves.]==] },
      code = { [==[total += a[i] - a[i - {1}]]==] },
      blanks = {
        { answers = { [==[k]==] }, hint = [==[How far back the item that leaves is.]==], wrong = { { [==[1]==], [==[a[i - 1] is still inside the window. The one leaving is k back.]==] } } },
      },
      output = { [==[each window sum in O(1)]==] },
    },
    {
      name = [==[Remembering answers]==], topic = [==[Dynamic programming]==],
      lesson = { [==[Dynamic programming stores answers to smaller problems so each is worked out once.]==] },
      code = { [==[memo[n] = fib(n - 1) + fib(n - {1})]==] },
      blanks = {
        { answers = { [==[2]==] }, hint = [==[Each Fibonacci number is the sum of the two before it.]==], wrong = { { [==[1]==], [==[fib(n - 1) is already there. The other one is n - 2.]==] } } },
      },
      output = { [==[fib(50) in microseconds instead of hours]==] },
    },
    {
      name = [==[Interview: first repeat]==], topic = [==[Interview questions]==],
      lesson = { [==[Question: find the first value that repeats in a list in one pass. Use a structure with O(1) membership tests.]==] },
      code = { [==[seen = {1}()]==], [==[for x in items:]==], [==[    if x in seen: return x]==], [==[    seen.add(x)]==] },
      blanks = {
        { answers = { [==[set]==], [==[hashset]==] }, hint = [==[Each value once, fast lookups.]==], wrong = { { [==[list]==], [==[x in a list is O(n), making the whole thing O(n^2). A set is O(1).]==] } } },
      },
      output = { [==[O(n) time, O(n) space]==] },
    },
    {
      name = [==[Interview: reverse a list]==], topic = [==[Interview questions]==],
      lesson = { [==[Question: reverse a linked list in place. Walk it once, turning each next pointer round.]==] },
      code = { [==[prev = None]==], [==[while node:]==], [==[    nxt = node.next]==], [==[    node.next = {1}]==], [==[    prev, node = node, nxt]==] },
      blanks = {
        { answers = { [==[prev]==] }, hint = [==[Point back at the node before.]==], wrong = { { [==[nxt]==], [==[That keeps it pointing forward. It must point back at prev.]==] } } },
      },
      output = { [==[O(n) time, O(1) extra space]==] },
    },
  },
}
