-- Learn to Code: the C++ track. Each level teaches one idea with a short lesson and a few
-- lines of code with blanks marked {1}, {2} and so on. A blank lists the answers that work,
-- a hint and the mistakes people often make, each with a message that explains it. The output
-- is what the finished code prints or does.
-- The lines run inside main with <iostream>, <string>, <vector>, <map> and <memory> included.
-- Answers are compared inside the game; nothing typed is ever run.

return {
  language = [==[C++]==],
  id = [==[cpp]==],
  levels = {
    {
      name = [==[Hello, output]==], topic = [==[Output]==],
      lesson = { [==[std::cout writes to the screen. << sends each value to it and std::endl ends the line.]==] },
      code = { [==[std::cout {1} "Hello, world" << std::endl;]==] },
      blanks = {
        { answers = { [==[<<]==] }, hint = [==[Two less than signs: the value flows into cout.]==], wrong = { { [==[>>]==], [==[>> reads from a stream. << writes to one.]==] }, { [==[<]==], [==[One < compares. Writing to a stream takes two: <<.]==] } } },
      },
      output = { [==[Hello, world]==] },
    },
    {
      name = [==[Let the compiler work it out]==], topic = [==[Variables and types]==],
      lesson = { [==[Every variable has a type. auto lets the compiler pick it from the value.]==] },
      code = { [==[{1} volts = 12;]==], [==[std::cout << volts / 2 << '\n';]==] },
      blanks = {
        { answers = { [==[auto]==], [==[int]==], [==[long]==], [==[short]==] }, hint = [==[A whole number type or the word that lets C++ choose.]==], wrong = { { [==[var]==], [==[var is Java and C#. C++ uses auto.]==] }, { [==[let]==], [==[let is JavaScript and Rust. C++ uses auto.]==] } } },
      },
      output = { [==[6]==] },
    },
    {
      name = [==[Reading a number]==], topic = [==[Input]==],
      lesson = { [==[std::cin reads what is typed. >> sends it into a variable.]==], [==[Here the person types 20.]==] },
      code = { [==[int age;]==], [==[std::cin {1} age;]==], [==[std::cout << age + 1 << '\n';]==] },
      blanks = {
        { answers = { [==[>>]==] }, hint = [==[Two greater than signs: the value flows out of cin.]==], wrong = { { [==[<<]==], [==[<< writes to a stream. Reading uses >>.]==] } } },
      },
      output = { [==[21]==] },
    },
    {
      name = [==[Too hot?]==], topic = [==[Conditions]==],
      lesson = { [==[if runs its block when the test in brackets is true.]==] },
      code = { [==[int temp = 72;]==], [==[if (temp {1} 70) {]==], [==[    std::cout << "Too hot\n";]==], [==[}]==] },
      blanks = {
        { answers = { [==[>]==], [==[>=]==], [==[!=]==] }, hint = [==[Which sign tests for more than?]==], wrong = { { [==[=]==], [==[One = stores a value. To compare use > here.]==] } } },
      },
      output = { [==[Too hot]==] },
    },
    {
      name = [==[Every pin]==], topic = [==[Loops]==],
      lesson = { [==[A range-based for visits every item of a container: for (int p : pins).]==] },
      code = { [==[std::vector<int> pins = {2, 3, 5};]==], [==[for (int p {1} pins) {]==], [==[    std::cout << p << '\n';]==], [==[}]==] },
      blanks = {
        { answers = { [==[:]==] }, hint = [==[One punctuation mark between the name and the container.]==], wrong = { { [==[in]==], [==[in is Python. C++ uses a colon.]==] } } },
      },
      output = { [==[2]==], [==[3]==], [==[5]==] },
    },
    {
      name = [==[Changing the caller's value]==], topic = [==[Functions]==],
      lesson = { [==[A parameter with & is a reference: the function works on the caller's variable itself.]==] },
      code = { [==[void twice(int{1} n) { n *= 2; }]==], [==[// in main:]==], [==[int x = 21;]==], [==[twice(x);]==], [==[std::cout << x << '\n';]==] },
      blanks = {
        { answers = { [==[&]==] }, hint = [==[The sign that makes a parameter a reference.]==], wrong = { { [==[*]==], [==[* would make n a pointer. The call would then need &x. A reference uses &.]==] } } },
      },
      output = { [==[42]==] },
    },
    {
      name = [==[Text that knows its size]==], topic = [==[Strings]==],
      lesson = { [==[std::string holds text and knows its length: size() or length().]==] },
      code = { [==[std::string name = "led";]==], [==[std::cout << name.{1}() << '\n';]==] },
      blanks = {
        { answers = { [==[size]==], [==[length]==] }, hint = [==[The method that counts the characters.]==], wrong = { { [==[len]==], [==[len is Python. A std::string uses size() or length().]==] }, { [==[strlen]==], [==[strlen is for C strings. A std::string has size().]==] } } },
      },
      output = { [==[3]==] },
    },
    {
      name = [==[A growing list]==], topic = [==[Collections]==],
      lesson = { [==[std::vector is an array that grows. push_back adds an item to the end.]==] },
      code = { [==[std::vector<int> pins;]==], [==[pins.{1}(13);]==], [==[std::cout << pins.size() << '\n';]==] },
      blanks = {
        { answers = { [==[push_back]==], [==[emplace_back]==] }, hint = [==[Push it on the back.]==], wrong = { { [==[append]==], [==[append is Python. A vector uses push_back.]==] }, { [==[add]==], [==[add is Java. A vector uses push_back.]==] }, { [==[push]==], [==[push is JavaScript. A vector uses push_back.]==] } } },
      },
      output = { [==[1]==] },
    },
    {
      name = [==[Parts by name]==], topic = [==[Collections]==],
      lesson = { [==[std::map stores keys with values, sorted by key. Square brackets look a key up.]==] },
      code = { [==[std::map<std::string, double> volts = {{"diode", 0.7}};]==], [==[std::cout << volts[{1}] << '\n';]==] },
      blanks = {
        { answers = { [==["diode"]==] }, hint = [==[The key, written as text in double quotes.]==], wrong = { { [==[diode]==], [==[The keys are strings, so the key needs its quotes.]==] } } },
      },
      output = { [==[0.7]==] },
    },
    {
      name = [==[When things go wrong]==], topic = [==[Exceptions]==],
      lesson = { [==[throw raises an exception. A try block with a matching catch handles it.]==] },
      code = { [==[try {]==], [==[    {1} std::runtime_error("no signal");]==], [==[} catch (const std::exception& e) {]==], [==[    std::cout << e.what() << '\n';]==], [==[}]==] },
      blanks = {
        { answers = { [==[throw]==] }, hint = [==[The keyword that raises an exception.]==], wrong = { { [==[raise]==], [==[raise is Python. C++ uses throw.]==] }, { [==[throws]==], [==[throws is Java's word in a method signature. C++ uses throw here.]==] } } },
      },
      output = { [==[no signal]==] },
    },
    {
      name = [==[An LED object]==], topic = [==[Classes and objects]==],
      lesson = { [==[A constructor can set fields in a list after a colon: Led(int p) : pin(p) {}.]==] },
      code = { [==[class Led {]==], [==[public:]==], [==[    int pin;]==], [==[    Led(int p) : pin({1}) {}]==], [==[};]==], [==[// in main:]==], [==[Led red(13);]==], [==[std::cout << red.pin << '\n';]==] },
      blanks = {
        { answers = { [==[p]==] }, hint = [==[The constructor's parameter.]==], wrong = { { [==[pin]==], [==[pin(pin) would copy the field into itself. Use the parameter p.]==] } } },
      },
      output = { [==[13]==] },
    },
    {
      name = [==[A kind of sensor]==], topic = [==[Inheritance]==],
      lesson = { [==[A class inherits with a colon and an access word: class Thermometer : public Sensor.]==] },
      code = { [==[class Sensor { public: int read() { return 0; } };]==], [==[class Thermometer : {1} Sensor {};]==], [==[// in main:]==], [==[std::cout << Thermometer().read() << '\n';]==] },
      blanks = {
        { answers = { [==[public]==] }, hint = [==[The access word that keeps Sensor's public methods public.]==], wrong = { { [==[extends]==], [==[extends is Java. C++ writes : public Sensor.]==] }, { [==[private]==], [==[private inheritance would hide read from main.]==] } } },
      },
      output = { [==[0]==] },
    },
    {
      name = [==[Many shapes, one call]==], topic = [==[Polymorphism]==],
      lesson = { [==[A virtual method can be replaced in a derived class. A call through a base pointer runs the derived one.]==] },
      code = { [==[class Shape { public: {1} double area() { return 0; } };]==], [==[class Square : public Shape { public: double area() override { return 4; } };]==], [==[Shape* s = new Square();]==], [==[std::cout << s->area() << '\n';]==] },
      blanks = {
        { answers = { [==[virtual]==] }, hint = [==[The keyword that lets a derived class replace a method.]==], wrong = { { [==[override]==], [==[override goes on the derived method. The base one is marked virtual.]==] }, { [==[abstract]==], [==[abstract is Java. C++ marks it virtual.]==] } } },
      },
      output = { [==[4]==] },
    },
    {
      name = [==[Code for any type]==], topic = [==[Templates]==],
      lesson = { [==[A template works for any type T: template <typename T>.]==] },
      code = { [==[template <{1} T>]==], [==[T biggest(T a, T b) { return a > b ? a : b; }]==], [==[// in main:]==], [==[std::cout << biggest(3, 7) << '\n';]==] },
      blanks = {
        { answers = { [==[typename]==], [==[class]==] }, hint = [==[The word that introduces a type parameter.]==], wrong = { { [==[type]==], [==[C++ writes typename (or class) before T.]==] } } },
      },
      output = { [==[7]==] },
    },
    {
      name = [==[Memory by hand]==], topic = [==[Pointers and memory]==],
      lesson = { [==[new makes an object on the heap and gives back a pointer. Every new needs a matching delete.]==] },
      code = { [==[int *p = {1} int(7);]==], [==[std::cout << *p << '\n';]==], [==[delete p;]==] },
      blanks = {
        { answers = { [==[new]==] }, hint = [==[The keyword that allocates on the heap.]==], wrong = { { [==[malloc]==], [==[malloc is C. In C++ new also runs the constructor.]==] } } },
      },
      output = { [==[7]==] },
    },
    {
      name = [==[Memory that cleans up]==], topic = [==[RAII and smart pointers]==],
      lesson = { [==[RAII ties a resource to an object: it is released when the object goes out of scope.]==], [==[std::make_unique makes a heap object owned by a unique_ptr that deletes it for you.]==] },
      code = { [==[auto led = std::{1}<int>(13);]==], [==[std::cout << *led << '\n';]==] },
      blanks = {
        { answers = { [==[make_unique]==], [==[make_shared]==] }, hint = [==[make, then the kind of smart pointer.]==], wrong = { { [==[new]==], [==[new gives a raw pointer that you must delete yourself. make_unique cleans up for you.]==] } } },
      },
      output = { [==[13]==] },
    },
  },
}
