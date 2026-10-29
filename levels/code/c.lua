-- Learn to Code: the C track. Each level teaches one idea with a short lesson and a few
-- lines of code with blanks marked {1}, {2} and so on. A blank lists the answers that work,
-- a hint and the mistakes people often make, each with a message that explains it. The output
-- is what the finished code prints or does.
-- The lines run inside main with <stdio.h>, <stdlib.h> and <string.h> included.
-- Answers are compared inside the game; nothing typed is ever run.

return {
  language = [==[C]==],
  id = [==[c]==],
  levels = {
    {
      name = [==[Hello, output]==], topic = [==[Output]==],
      lesson = { [==[printf prints text. \n ends the line.]==], [==[Text goes inside double quotes and every statement ends with ;.]==] },
      code = { [==[printf({1});]==] },
      blanks = {
        { answers = { [==["Hello, world\n"]==], [==["Hello, world"]==] }, hint = [==[Put Hello, world\n inside double quotes.]==], wrong = { { [==['Hello, world\n']==], [==[In C single quotes hold one character. Text needs double quotes.]==] }, { [==[Hello, world]==], [==[Text needs double quotes around it.]==] } } },
      },
      output = { [==[Hello, world]==] },
    },
    {
      name = [==[Typed boxes]==], topic = [==[Variables and types]==],
      lesson = { [==[C names a variable's type: int for whole numbers, float or double for decimals.]==], [==[printf's %.1f shows a decimal with one digit after the point.]==] },
      code = { [==[{1} volts = 12;]==], [==[float amps = 0.5;]==], [==[printf("%.1f\n", volts * amps);]==] },
      blanks = {
        { answers = { [==[int]==], [==[long]==], [==[short]==], [==[float]==], [==[double]==] }, hint = [==[The type for a whole number, in lower case.]==], wrong = { { [==[Int]==], [==[C types are lower case: int.]==] }, { [==[integer]==], [==[C's whole number type is spelled int.]==] }, { [==[var]==], [==[C has no var: every variable names its type.]==] } } },
      },
      output = { [==[6.0]==] },
    },
    {
      name = [==[Reading a number]==], topic = [==[Input]==],
      lesson = { [==[scanf reads what is typed. It needs the address of the variable to fill, written with &.]==], [==[Here the person types 20.]==] },
      code = { [==[int age;]==], [==[scanf("%d", {1}age);]==], [==[printf("%d\n", age + 1);]==] },
      blanks = {
        { answers = { [==[&]==] }, hint = [==[The sign that gives the address of a variable.]==], wrong = { { [==[*]==], [==[* reads through a pointer. scanf needs the address of age: &age.]==] } } },
      },
      output = { [==[21]==] },
    },
    {
      name = [==[Remainders]==], topic = [==[Operators]==],
      lesson = { [==[Between two ints / drops the remainder. % gives just the remainder.]==] },
      code = { [==[printf("%d\n", 17 {1} 5);]==] },
      blanks = {
        { answers = { [==[%]==] }, hint = [==[Which operator gives what is left over?]==], wrong = { { [==[/]==], [==[17 / 5 between ints is 3. % gives the remainder.]==] } } },
      },
      output = { [==[2]==] },
    },
    {
      name = [==[Too hot?]==], topic = [==[Conditions]==],
      lesson = { [==[if runs its block when the test in brackets is true. In C a true test is any value but 0.]==] },
      code = { [==[int temp = 72;]==], [==[if (temp {1} 70) {]==], [==[    printf("Too hot\n");]==], [==[}]==] },
      blanks = {
        { answers = { [==[>]==], [==[>=]==], [==[!=]==] }, hint = [==[72 is more than 70. Which sign tests for more than?]==], wrong = { { [==[=]==], [==[One = stores 70 in temp and the test is then always true. Compare with > here.]==] }, { [==[=>]==], [==[C writes greater than or equal as >=.]==] } } },
      },
      output = { [==[Too hot]==] },
    },
    {
      name = [==[Grade bands]==], topic = [==[Conditions]==],
      lesson = { [==[else if adds another test. The first true test wins.]==] },
      code = { [==[int mark = 65;]==], [==[if (mark >= 70) printf("First\n");]==], [==[{1} (mark >= 60) printf("2:1\n");]==], [==[else printf("Lower\n");]==] },
      blanks = {
        { answers = { [==[else if]==] }, hint = [==[Two words: what to try when the first test fails.]==], wrong = { { [==[elif]==], [==[elif is Python. C writes else if.]==] } } },
      },
      output = { [==[2:1]==] },
    },
    {
      name = [==[Counting up]==], topic = [==[Loops]==],
      lesson = { [==[A for loop has a start, a test checked before each turn and a step. i++ adds one.]==], [==[%d in printf shows an int.]==] },
      code = { [==[for (int i = 0; i {1} 3; i++) {]==], [==[    printf("%d\n", i);]==], [==[}]==] },
      blanks = {
        { answers = { [==[<]==], [==[!=]==] }, hint = [==[Stop before i reaches 3.]==], wrong = { { [==[<=]==], [==[i <= 3 also runs for 3, printing four numbers.]==] } } },
      },
      output = { [==[0]==], [==[1]==], [==[2]==] },
    },
    {
      name = [==[Countdown]==], topic = [==[Loops]==],
      lesson = { [==[while repeats its block as long as its test is true.]==] },
      code = { [==[int count = 3;]==], [==[while (count > 0) {]==], [==[    printf("%d\n", count);]==], [==[    count{1};]==], [==[}]==] },
      blanks = {
        { answers = { [==[--]==], [==[-=1]==], [==[=count-1]==] }, hint = [==[Take one off count: two minus signs.]==], wrong = { { [==[++]==], [==[Adding makes count grow for ever, so the loop never ends.]==] } } },
      },
      output = { [==[3]==], [==[2]==], [==[1]==] },
    },
    {
      name = [==[Twice as much]==], topic = [==[Functions]==],
      lesson = { [==[int twice(int n) is a function that takes an int and gives back an int.]==], [==[return hands the value back to the caller.]==] },
      code = { [==[int twice(int n) {]==], [==[    {1} n * 2;]==], [==[}]==], [==[// in main:]==], [==[printf("%d\n", twice(21));]==] },
      blanks = {
        { answers = { [==[return]==] }, hint = [==[The keyword that gives a value back from a function.]==], wrong = { { [==[printf(]==], [==[Printing inside the function gives nothing back to the caller.]==] } } },
      },
      output = { [==[42]==] },
    },
    {
      name = [==[A row of pins]==], topic = [==[Arrays]==],
      lesson = { [==[An array holds a fixed number of values: int pins[4] = {2, 3, 5, 7};]==], [==[The size counts the items. Indexes then run from 0 to size - 1.]==] },
      code = { [==[int pins[{1}] = {2, 3, 5, 7};]==], [==[printf("%d\n", pins[3]);]==] },
      blanks = {
        { answers = { [==[4]==] }, hint = [==[Count the numbers in the braces.]==], wrong = { { [==[3]==], [==[The size counts the items: four numbers need [4]. Indexes then run from 0 to 3.]==] } } },
      },
      output = { [==[7]==] },
    },
    {
      name = [==[Counting letters]==], topic = [==[Strings]==],
      lesson = { [==[A C string is an array of char ending with a hidden zero byte.]==], [==[strlen from <string.h> counts the characters before that zero.]==] },
      code = { [==[char name[] = "led";]==], [==[printf("%zu\n", {1}(name));]==] },
      blanks = {
        { answers = { [==[strlen]==] }, hint = [==[Its name is short for string length.]==], wrong = { { [==[len]==], [==[C has no len. strlen counts the characters.]==] }, { [==[sizeof]==], [==[sizeof name gives 4: it counts the hidden zero byte too.]==] } } },
      },
      output = { [==[3]==] },
    },
    {
      name = [==[Same text?]==], topic = [==[Strings]==],
      lesson = { [==[== on two strings compares their addresses, not their letters. strcmp compares the letters and gives 0 when they match.]==] },
      code = { [==[char a[] = "led", b[] = "led";]==], [==[if ({1}(a, b) == 0) {]==], [==[    printf("Same\n");]==], [==[}]==] },
      blanks = {
        { answers = { [==[strcmp]==] }, hint = [==[String compare, shortened to six letters.]==], wrong = { { [==[strcpy]==], [==[strcpy copies a string. strcmp compares two.]==] }, { [==[equals]==], [==[equals is Java. C compares strings with strcmp.]==] } } },
      },
      output = { [==[Same]==] },
    },
    {
      name = [==[Pointing at a pin]==], topic = [==[Pointers]==],
      lesson = { [==[A pointer holds the address of a variable. &pin is the address of pin.]==], [==[*p reaches the variable p points at, so *p = 7 changes pin.]==] },
      code = { [==[int pin = 13;]==], [==[int *p = {1}pin;]==], [==[*p = 7;]==], [==[printf("%d\n", pin);]==] },
      blanks = {
        { answers = { [==[&]==] }, hint = [==[The sign that gives the address of a variable.]==], wrong = { { [==[*]==], [==[*pin would read through pin as if it were a pointer. & takes its address.]==] } } },
      },
      output = { [==[7]==] },
    },
    {
      name = [==[Borrowing memory]==], topic = [==[Memory]==],
      lesson = { [==[malloc asks for memory while the program runs and gives back a pointer to it.]==], [==[Every block from malloc must be handed back with free when it is no longer needed.]==] },
      code = { [==[int *buf = malloc(4 * sizeof(int));]==], [==[buf[0] = 42;]==], [==[printf("%d\n", buf[0]);]==], [==[{1}(buf);]==] },
      blanks = {
        { answers = { [==[free]==] }, hint = [==[The function that gives memory back.]==], wrong = { { [==[delete]==], [==[delete is C++. In C memory from malloc goes back with free.]==] }, { [==[malloc]==], [==[malloc asks for more memory. free gives it back.]==] } } },
      },
      output = { [==[42]==] },
    },
    {
      name = [==[Grouping fields]==], topic = [==[Structs]==],
      lesson = { [==[A struct groups named fields. A dot reaches a field of a struct.]==] },
      code = { [==[struct Led { int pin; };]==], [==[struct Led red = { 13 };]==], [==[printf("%d\n", red{1}pin);]==] },
      blanks = {
        { answers = { [==[.]==] }, hint = [==[One punctuation mark between the struct and its field.]==], wrong = { { [==[->]==], [==[-> reaches a field through a pointer. red is a struct itself, so use a dot.]==] } } },
      },
      output = { [==[13]==] },
    },
    {
      name = [==[Through a pointer]==], topic = [==[Structs]==],
      lesson = { [==[When you hold a pointer to a struct, -> reaches its fields. p->pin is short for (*p).pin.]==] },
      code = { [==[struct Led red = { 13 };]==], [==[struct Led *p = &red;]==], [==[printf("%d\n", p{1}pin);]==] },
      blanks = {
        { answers = { [==[->]==] }, hint = [==[An arrow made of a minus and a greater than sign.]==], wrong = { { [==[.]==], [==[p is a pointer, so its field is reached with ->. (*p).pin works too.]==] } } },
      },
      output = { [==[13]==] },
    },
    {
      name = [==[A file that is not there]==], topic = [==[Error handling]==],
      lesson = { [==[fopen gives back NULL when it cannot open a file. Check before using it.]==], [==[Here missing.txt does not exist.]==] },
      code = { [==[FILE *f = fopen("missing.txt", "r");]==], [==[if (f == {1}) {]==], [==[    printf("Could not open\n");]==], [==[}]==] },
      blanks = {
        { answers = { [==[NULL]==], [==[0]==] }, hint = [==[The null pointer, in capitals.]==], wrong = { { [==[null]==], [==[C writes the null pointer in capitals: NULL.]==] }, { [==[None]==], [==[None is Python. C uses NULL.]==] } } },
      },
      output = { [==[Could not open]==] },
    },
    {
      name = [==[Writing a file]==], topic = [==[Files]==],
      lesson = { [==[fopen's second argument is the mode: "r" reads, "w" writes from the start and "a" adds to the end.]==] },
      code = { [==[FILE *f = fopen("log.txt", "{1}");]==], [==[fprintf(f, "%d\n", 42);]==], [==[fclose(f);]==] },
      blanks = {
        { answers = { [==[w]==], [==[a]==] }, hint = [==[One letter for writing.]==], wrong = { { [==[r]==], [==["r" opens for reading only, so fprintf would fail.]==] } } },
      },
      output = { [==[log.txt now holds 42]==] },
    },
    {
      name = [==[How big is it?]==], topic = [==[Memory]==],
      lesson = { [==[sizeof gives the size of a type in bytes. A char is always 1 byte.]==] },
      code = { [==[printf("%zu\n", sizeof({1}));]==] },
      blanks = {
        { answers = { [==[char]==] }, hint = [==[The type for one character.]==], wrong = { { [==[int]==], [==[An int is usually 4 bytes, so the output would be 4.]==] } } },
      },
      output = { [==[1]==] },
    },
  },
}
