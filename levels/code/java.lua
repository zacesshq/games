-- Learn to Code: the Java track. Each level teaches one idea with a short lesson and a few
-- lines of code with blanks marked {1}, {2} and so on. A blank lists the answers that work,
-- a hint and the mistakes people often make, each with a message that explains it. The output
-- is what the finished code prints or does.
-- The lines run inside a main method unless a level shows a class of its own.
-- Answers are compared inside the game; nothing typed is ever run.

return {
  language = [==[Java]==],
  id = [==[java]==],
  levels = {
    {
      name = [==[Hello, output]==], topic = [==[Output]==],
      lesson = { [==[System.out.println prints a value and ends the line.]==], [==[Text is a String and goes inside double quotes. Every statement ends with ;.]==] },
      code = { [==[System.out.println({1});]==] },
      blanks = {
        { answers = { [==["Hello, world"]==] }, hint = [==[Put Hello, world inside double quotes.]==], wrong = { { [==['Hello, world']==], [==[In Java single quotes hold one char. A String needs double quotes.]==] }, { [==[Hello, world]==], [==[Text needs double quotes around it. Without them Java reads it as names.]==] } } },
      },
      output = { [==[Hello, world]==] },
    },
    {
      name = [==[Typed boxes]==], topic = [==[Variables and types]==],
      lesson = { [==[Java names a variable's type when it is made: int for whole numbers, double for decimals.]==], [==[An int times a double gives a double.]==] },
      code = { [==[{1} volts = 12;]==], [==[double amps = 0.5;]==], [==[System.out.println(volts * amps);]==] },
      blanks = {
        { answers = { [==[int]==], [==[long]==], [==[double]==], [==[float]==], [==[var]==] }, hint = [==[The type for a whole number, written in lower case.]==], wrong = { { [==[Int]==], [==[Java types are case sensitive: int is lower case.]==] }, { [==[integer]==], [==[Java's whole number type is spelled int.]==] }, { [==[String]==], [==[A String holds text; 12 here is a number.]==] } } },
      },
      output = { [==[6.0]==] },
    },
    {
      name = [==[Reading a line]==], topic = [==[Input]==],
      lesson = { [==[A Scanner reads what is typed. nextLine() gives back the whole line as a String.]==], [==[Here the person types Ada.]==] },
      code = { [==[Scanner in = new Scanner(System.in);]==], [==[String name = in.{1}();]==], [==[System.out.println("Hi " + name);]==] },
      blanks = {
        { answers = { [==[nextLine]==], [==[next]==] }, hint = [==[The Scanner method that reads the next line.]==], wrong = { { [==[readLine]==], [==[readLine belongs to BufferedReader. A Scanner uses nextLine.]==] }, { [==[input]==], [==[input is Python. A Scanner reads with nextLine.]==] } } },
      },
      output = { [==[Hi Ada]==] },
    },
    {
      name = [==[Remainders]==], topic = [==[Operators]==],
      lesson = { [==[Between two ints / drops the remainder. % gives just the remainder.]==] },
      code = { [==[System.out.println(17 {1} 5);]==] },
      blanks = {
        { answers = { [==[%]==] }, hint = [==[Which operator gives what is left over?]==], wrong = { { [==[/]==], [==[17 / 5 between ints is 3, the whole part. % gives the remainder.]==] } } },
      },
      output = { [==[2]==] },
    },
    {
      name = [==[Too hot?]==], topic = [==[Conditions]==],
      lesson = { [==[if runs its block when the test in brackets is true.]==], [==[Tests compare values: > greater, < less, >= at least and == equal.]==] },
      code = { [==[int temp = 72;]==], [==[if (temp {1} 70) {]==], [==[    System.out.println("Too hot");]==], [==[}]==] },
      blanks = {
        { answers = { [==[>]==], [==[>=]==], [==[!=]==] }, hint = [==[72 is more than 70. Which sign tests for more than?]==], wrong = { { [==[=>]==], [==[Java writes greater than or equal as >=, with the = second.]==] }, { [==[=]==], [==[One = stores a value. To compare use > here (or == for equal).]==] } } },
      },
      output = { [==[Too hot]==] },
    },
    {
      name = [==[Grade bands]==], topic = [==[Conditions]==],
      lesson = { [==[else if adds another test. The first true test wins.]==] },
      code = { [==[int mark = 65;]==], [==[if (mark >= 70) {]==], [==[    System.out.println("First");]==], [==[} {1} (mark >= 60) {]==], [==[    System.out.println("2:1");]==], [==[}]==] },
      blanks = {
        { answers = { [==[else if]==] }, hint = [==[Two words: what to try when the first test fails.]==], wrong = { { [==[elif]==], [==[elif is Python. Java writes else if.]==] } } },
      },
      output = { [==[2:1]==] },
    },
    {
      name = [==[Counting up]==], topic = [==[Loops]==],
      lesson = { [==[A for loop has three parts: a start, a test checked before each turn and a step.]==], [==[i++ adds one to i after each turn.]==] },
      code = { [==[for (int i = 0; i {1} 3; i++) {]==], [==[    System.out.println(i);]==], [==[}]==] },
      blanks = {
        { answers = { [==[<]==], [==[!=]==] }, hint = [==[The loop prints 0, 1 and 2, so it must stop before i reaches 3.]==], wrong = { { [==[<=]==], [==[i <= 3 also runs for 3, printing four numbers.]==] }, { [==[>]==], [==[0 > 3 is false at once, so the loop never runs.]==] } } },
      },
      output = { [==[0]==], [==[1]==], [==[2]==] },
    },
    {
      name = [==[Countdown]==], topic = [==[Loops]==],
      lesson = { [==[while repeats its block as long as its test stays true.]==], [==[Change the value it tests inside the loop. Otherwise it never ends.]==] },
      code = { [==[int count = 3;]==], [==[while (count > 0) {]==], [==[    System.out.println(count);]==], [==[    count{1};]==], [==[}]==] },
      blanks = {
        { answers = { [==[--]==], [==[-=1]==], [==[=count-1]==] }, hint = [==[Take one off count. Java has a short way: two minus signs.]==], wrong = { { [==[++]==], [==[Adding makes count grow for ever, so the loop never ends.]==] }, { [==[-1]==], [==[count-1 works out a value but never stores it. Use count-- or count -= 1.]==] } } },
      },
      output = { [==[3]==], [==[2]==], [==[1]==] },
    },
    {
      name = [==[Twice as much]==], topic = [==[Methods]==],
      lesson = { [==[A method is a named block. static int twice(int n) takes an int and gives back an int.]==], [==[return hands the value back to the caller.]==] },
      code = { [==[static int twice(int n) {]==], [==[    {1} n * 2;]==], [==[}]==], [==[// in main:]==], [==[System.out.println(twice(21));]==] },
      blanks = {
        { answers = { [==[return]==] }, hint = [==[The keyword that gives a value back from a method.]==], wrong = { { [==[System.out.println(]==], [==[Printing inside the method gives nothing back. An int method must return an int.]==] } } },
      },
      output = { [==[42]==] },
    },
    {
      name = [==[A row of pins]==], topic = [==[Arrays]==],
      lesson = { [==[An array holds a fixed number of values of one type: int[] pins = {2, 3, 5, 7};]==], [==[Indexes start at 0. An array knows its size in the field length.]==] },
      code = { [==[int[] pins = {2, 3, 5, 7};]==], [==[System.out.println(pins[{1}]);]==], [==[System.out.println(pins.{2});]==] },
      blanks = {
        { answers = { [==[0]==] }, hint = [==[The output's first line is 2, the first item.]==], wrong = { { [==[1]==], [==[Indexes start at 0, so pins[1] is 3, the second item.]==] } } },
        { answers = { [==[length]==] }, hint = [==[The field that holds how many items an array has.]==], wrong = { { [==[length()]==], [==[For an array length is a field, so it has no brackets. Strings use length().]==] }, { [==[size()]==], [==[size() belongs to lists such as ArrayList. Arrays use length.]==] } } },
      },
      output = { [==[2]==], [==[4]==] },
    },
    {
      name = [==[Every pin]==], topic = [==[Loops]==],
      lesson = { [==[The for-each loop visits every item: for (int p : pins).]==] },
      code = { [==[int[] pins = {2, 3, 5};]==], [==[for (int p {1} pins) {]==], [==[    System.out.println(p);]==], [==[}]==] },
      blanks = {
        { answers = { [==[:]==] }, hint = [==[One punctuation mark between the name and the array.]==], wrong = { { [==[in]==], [==[Java's for-each uses a colon: for (int p : pins).]==] }, { [==[of]==], [==[for...of is JavaScript. Java uses a colon.]==] } } },
      },
      output = { [==[2]==], [==[3]==], [==[5]==] },
    },
    {
      name = [==[Shouting text]==], topic = [==[Strings]==],
      lesson = { [==[A String has methods: toUpperCase() gives a new String in capitals.]==], [==[length() counts its characters.]==] },
      code = { [==[String part = "diode";]==], [==[System.out.println(part.{1}());]==], [==[System.out.println(part.length());]==] },
      blanks = {
        { answers = { [==[toUpperCase]==] }, hint = [==[Its name starts with to and ends with Case.]==], wrong = { { [==[upper]==], [==[That is Python. Java spells it toUpperCase.]==] }, { [==[touppercase]==], [==[Java names are case sensitive: toUpperCase has a capital U and C.]==] } } },
      },
      output = { [==[DIODE]==], [==[5]==] },
    },
    {
      name = [==[A growing list]==], topic = [==[Collections]==],
      lesson = { [==[An ArrayList grows as items are added. add puts one on the end and size() counts them.]==] },
      code = { [==[ArrayList<Integer> pins = new ArrayList<>();]==], [==[pins.{1}(13);]==], [==[System.out.println(pins.size());]==] },
      blanks = {
        { answers = { [==[add]==] }, hint = [==[The ArrayList method that puts an item on the end.]==], wrong = { { [==[append]==], [==[append is Python. An ArrayList uses add.]==] }, { [==[push]==], [==[push is JavaScript. An ArrayList uses add.]==] } } },
      },
      output = { [==[1]==] },
    },
    {
      name = [==[Parts by name]==], topic = [==[Collections]==],
      lesson = { [==[A HashMap maps keys to values. put stores a pair and get looks one up.]==] },
      code = { [==[HashMap<String, Double> volts = new HashMap<>();]==], [==[volts.put("diode", 0.7);]==], [==[System.out.println(volts.{1}("diode"));]==] },
      blanks = {
        { answers = { [==[get]==] }, hint = [==[The method that looks a value up by its key.]==], wrong = { { [==[find]==], [==[A HashMap looks a key up with get.]==] }, { [==[put]==], [==[put stores a pair. get reads one back.]==] } } },
      },
      output = { [==[0.7]==] },
    },
    {
      name = [==[When things go wrong]==], topic = [==[Exceptions]==],
      lesson = { [==[Code that may fail goes in try. A matching catch block runs when it throws.]==] },
      code = { [==[try {]==], [==[    int n = Integer.parseInt("five");]==], [==[} {1} (NumberFormatException e) {]==], [==[    System.out.println("Not a number");]==], [==[}]==] },
      blanks = {
        { answers = { [==[catch]==] }, hint = [==[The keyword that catches an exception in Java.]==], wrong = { { [==[except]==], [==[except is Python. Java catches with catch.]==] }, { [==[Catch]==], [==[Java keywords are lower case: catch.]==] } } },
      },
      output = { [==[Not a number]==] },
    },
    {
      name = [==[Reading a file]==], topic = [==[Files]==],
      lesson = { [==[Files.readAllLines reads a whole file into a List of lines.]==], [==[Here notes.txt starts with the line: Buy resistors.]==] },
      code = { [==[List<String> lines = Files.{1}(Path.of("notes.txt"));]==], [==[System.out.println(lines.get(0));]==] },
      blanks = {
        { answers = { [==[readAllLines]==] }, hint = [==[Read all lines: three words joined, each after the first with a capital.]==], wrong = { { [==[readLines]==], [==[The method is readAllLines.]==] }, { [==[open]==], [==[open is Python. Java's Files class reads with readAllLines.]==] } } },
      },
      output = { [==[Buy resistors]==] },
    },
    {
      name = [==[An LED object]==], topic = [==[Classes and objects]==],
      lesson = { [==[A class describes a kind of object. Led has a field pin and a constructor that sets it.]==], [==[new makes an object from a class. A dot reaches its fields.]==] },
      code = { [==[class Led {]==], [==[    int pin;]==], [==[    Led(int p) { pin = p; }]==], [==[}]==], [==[// in main:]==], [==[Led red = {1} Led(13);]==], [==[System.out.println(red.{2});]==] },
      blanks = {
        { answers = { [==[new]==] }, hint = [==[The keyword that builds a new object.]==], wrong = { { [==[New]==], [==[Java keywords are lower case: new.]==] } } },
        { answers = { [==[pin]==] }, hint = [==[The name of the field that holds 13.]==], wrong = { { [==[p]==], [==[p is only the constructor's parameter. The object keeps the value in pin.]==] } } },
      },
      output = { [==[13]==] },
    },
    {
      name = [==[Keeping fields safe]==], topic = [==[Encapsulation]==],
      lesson = { [==[A private field can only be used inside its own class. Others reach it through methods such as getPin().]==] },
      code = { [==[class Led {]==], [==[    {1} int pin = 13;]==], [==[    int getPin() { return pin; }]==], [==[}]==], [==[// in main:]==], [==[System.out.println(new Led().getPin());]==] },
      blanks = {
        { answers = { [==[private]==] }, hint = [==[The access word that hides a field from other classes.]==], wrong = { { [==[public]==], [==[public lets every class change pin directly. private keeps it inside Led.]==] }, { [==[protected]==], [==[protected still lets subclasses and the package in. private keeps it to Led.]==] } } },
      },
      output = { [==[13]==] },
    },
    {
      name = [==[A kind of sensor]==], topic = [==[Inheritance]==],
      lesson = { [==[A class extends another to get all of its methods.]==] },
      code = { [==[class Sensor { int read() { return 0; } }]==], [==[class Thermometer {1} Sensor { }]==], [==[// in main:]==], [==[System.out.println(new Thermometer().read());]==] },
      blanks = {
        { answers = { [==[extends]==] }, hint = [==[The keyword for inheriting from a class.]==], wrong = { { [==[implements]==], [==[implements is for interfaces. A class extends another class.]==] }, { [==[:]==], [==[A colon is how C++ and C# inherit. Java uses extends.]==] } } },
      },
      output = { [==[0]==] },
    },
    {
      name = [==[A promise to provide]==], topic = [==[Interfaces]==],
      lesson = { [==[An interface lists methods. A class that implements it must provide them.]==] },
      code = { [==[interface Shape { double area(); }]==], [==[class Square {1} Shape {]==], [==[    public double area() { return 4.0; }]==], [==[}]==], [==[// in main:]==], [==[System.out.println(new Square().area());]==] },
      blanks = {
        { answers = { [==[implements]==] }, hint = [==[The keyword a class uses to take on an interface.]==], wrong = { { [==[extends]==], [==[A class extends a class but implements an interface.]==] } } },
      },
      output = { [==[4.0]==] },
    },
    {
      name = [==[Lists of one type]==], topic = [==[Generics]==],
      lesson = { [==[Generics name the type a collection holds: List<String> holds only Strings.]==] },
      code = { [==[List<{1}> names = new ArrayList<>();]==], [==[names.add("Ada");]==], [==[System.out.println(names.get(0));]==] },
      blanks = {
        { answers = { [==[String]==] }, hint = [==[The type for text, with a capital S.]==], wrong = { { [==[string]==], [==[Java's text type is String, with a capital S.]==] }, { [==[str]==], [==[str is Python. Java's text type is String.]==] } } },
      },
      output = { [==[Ada]==] },
    },
    {
      name = [==[Saying what, not how]==], topic = [==[Abstract data types]==],
      lesson = { [==[An abstract class or interface fixes the operations of a data type. Code written against List works with ArrayList or LinkedList alike.]==] },
      code = { [==[{1}<Integer> pins = new LinkedList<>();]==], [==[pins.add(13);]==], [==[System.out.println(pins.get(0));]==] },
      blanks = {
        { answers = { [==[List]==] }, hint = [==[The interface both ArrayList and LinkedList implement.]==], wrong = { { [==[Array]==], [==[Array is not a collection interface. Program to List.]==] } } },
      },
      output = { [==[13]==] },
    },
    {
      name = [==[Last in, first out]==], topic = [==[Stacks]==],
      lesson = { [==[ArrayDeque works as a stack: push adds on top and pop takes the top off.]==] },
      code = { [==[Deque<Integer> stack = new ArrayDeque<>();]==], [==[stack.push(1);]==], [==[stack.push(2);]==], [==[System.out.println(stack.{1}());]==] },
      blanks = {
        { answers = { [==[pop]==], [==[poll]==] }, hint = [==[The method that takes the top item off.]==], wrong = { { [==[get]==], [==[Deque has no get. A stack pops.]==] } } },
      },
      output = { [==[2]==] },
    },
    {
      name = [==[First in, first out]==], topic = [==[Queues]==],
      lesson = { [==[A Queue adds with offer at the back and removes with poll from the front.]==] },
      code = { [==[Queue<String> line = new ArrayDeque<>();]==], [==[line.offer("Ada");]==], [==[line.offer("Bo");]==], [==[System.out.println(line.{1}());]==] },
      blanks = {
        { answers = { [==[poll]==], [==[remove]==] }, hint = [==[The method that takes from the front.]==], wrong = { { [==[pop]==], [==[pop treats the deque as a stack. A queue uses poll.]==] }, { [==[peek]==], [==[peek looks without removing.]==] } } },
      },
      output = { [==[Ada]==] },
    },
    {
      name = [==[Equal objects, equal hashes]==], topic = [==[Hashing]==],
      lesson = { [==[A HashMap or HashSet finds an object by hashCode, then confirms with equals. If you override equals you must override hashCode too.]==] },
      code = { [==[class Part {]==], [==[    String name;]==], [==[    @Override public boolean equals(Object o) { return o instanceof Part p && p.name.equals(name); }]==], [==[    @Override public int {1}() { return name.hashCode(); }]==], [==[}]==] },
      blanks = {
        { answers = { [==[hashCode]==] }, hint = [==[The method a hash table calls first.]==], wrong = { { [==[hash]==], [==[The method is hashCode.]==] }, { [==[hashcode]==], [==[Java names are case sensitive: hashCode.]==] } } },
      },
      output = { [==[two Parts with the same name land in the same bucket]==] },
    },
    {
      name = [==[Each one once]==], topic = [==[Sets]==],
      lesson = { [==[A HashSet keeps each value once. add gives false when the value is already there.]==] },
      code = { [==[Set<String> seen = new HashSet<>();]==], [==[seen.add("led");]==], [==[System.out.println(seen.add({1}));]==] },
      blanks = {
        { answers = { [==["led"]==] }, hint = [==[Add the same value again to see false.]==], wrong = { { [==[led]==], [==[The value is a String, so it needs double quotes.]==] } } },
      },
      output = { [==[false]==] },
    },
    {
      name = [==[Sorted keys]==], topic = [==[Maps]==],
      lesson = { [==[A TreeMap keeps its keys sorted, so firstKey gives the smallest.]==] },
      code = { [==[TreeMap<String, Integer> stock = new {1}<>();]==], [==[stock.put("resistor", 50);]==], [==[stock.put("diode", 20);]==], [==[System.out.println(stock.firstKey());]==] },
      blanks = {
        { answers = { [==[TreeMap]==] }, hint = [==[The map that keeps keys sorted.]==], wrong = { { [==[HashMap]==], [==[A HashMap has no order and no firstKey.]==] }, { [==[SortedMap]==], [==[SortedMap is an interface. The class is TreeMap.]==] } } },
      },
      output = { [==[diode]==] },
    },
    {
      name = [==[Natural order]==], topic = [==[Sorting]==],
      lesson = { [==[A class that implements Comparable gives its own natural order through compareTo. Collections.sort then uses it.]==] },
      code = { [==[class Part implements {1}<Part> {]==], [==[    int volts;]==], [==[    public int compareTo(Part o) { return Integer.compare(volts, o.volts); }]==], [==[}]==] },
      blanks = {
        { answers = { [==[Comparable]==] }, hint = [==[The interface for a natural order.]==], wrong = { { [==[Comparator]==], [==[A Comparator is a separate object. A class orders itself with Comparable.]==] } } },
      },
      output = { [==[Collections.sort(parts) sorts them by volts]==] },
    },
    {
      name = [==[Searching a sorted list]==], topic = [==[Searching]==],
      lesson = { [==[Collections.binarySearch finds an item in a sorted list in O(log n) and gives back its index.]==] },
      code = { [==[List<Integer> pins = List.of(2, 3, 5, 7, 11);]==], [==[System.out.println(Collections.{1}(pins, 7));]==] },
      blanks = {
        { answers = { [==[binarySearch]==] }, hint = [==[binary, then Search with a capital S.]==], wrong = { { [==[indexOf]==], [==[indexOf gives 3 as well but checks every item, O(n). Use binarySearch, which halves each step.]==] } } },
      },
      output = { [==[3]==] },
    },
    {
      name = [==[How it grows]==], topic = [==[Big O]==],
      lesson = { [==[ArrayList.get(i) takes the same time for any i. LinkedList.get(i) has to walk i nodes.]==] },
      code = { [==[// time for LinkedList.get(i), in Big O: {1}]==] },
      blanks = {
        { answers = { [==[O(n)]==], [==[linear]==] }, hint = [==[It walks the nodes one by one.]==], wrong = { { [==[O(1)]==], [==[That is ArrayList.get. A linked list walks to the item.]==] } } },
      },
      output = { [==[ArrayList for lookups by index, LinkedList for adding at the ends]==] },
    },
  },
}
