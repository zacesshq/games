-- Learn to Code: the Rust track. Each level teaches one idea with a short lesson and a few
-- lines of code with blanks marked {1}, {2} and so on. A blank lists the answers that work,
-- a hint and the mistakes people often make, each with a message that explains it. The output
-- is what the finished code prints or does.
-- The lines run inside fn main unless a level shows a function of its own.
-- Answers are compared inside the game; nothing typed is ever run.

return {
  language = [==[Rust]==],
  id = [==[rust]==],
  levels = {
    {
      name = [==[Hello, output]==], topic = [==[Output]==],
      lesson = { [==[println! is a macro that prints a line. Macros end in !.]==] },
      code = { [==[{1}("Hello, world");]==] },
      blanks = {
        { answers = { [==[println!]==] }, hint = [==[print, line and the mark every macro has.]==], wrong = { { [==[println]==], [==[println is a macro, so it is called with !: println!.]==] }, { [==[print]==], [==[print! would leave the line open. println! ends it.]==] } } },
      },
      output = { [==[Hello, world]==] },
    },
    {
      name = [==[Changing values]==], topic = [==[Variables]==],
      lesson = { [==[Variables cannot change unless declared with mut.]==] },
      code = { [==[let {1} count = 0;]==], [==[count += 1;]==], [==[println!("{}", count);]==] },
      blanks = {
        { answers = { [==[mut]==] }, hint = [==[Short for mutable.]==], wrong = { { [==[var]==], [==[Rust uses let mut, not var.]==] }, { [==[const]==], [==[const can never change.]==] } } },
      },
      output = { [==[1]==] },
    },
    {
      name = [==[Typed numbers]==], topic = [==[Types]==],
      lesson = { [==[Rust has sized number types: i32 is a 32-bit signed integer, u8 an unsigned byte and f64 a decimal.]==] },
      code = { [==[let volts: {1} = 12;]==], [==[println!("{}", volts / 2);]==] },
      blanks = {
        { answers = { [==[i32]==], [==[i64]==], [==[u8]==], [==[u16]==], [==[u32]==], [==[u64]==], [==[usize]==], [==[i16]==], [==[isize]==] }, hint = [==[A whole number type such as the default one.]==], wrong = { { [==[int]==], [==[Rust names integer types by size: i32.]==] }, { [==[f64]==], [==[f64 is a decimal, so 12 would need to be 12.0.]==] } } },
      },
      output = { [==[6]==] },
    },
    {
      name = [==[If gives a value]==], topic = [==[Conditions]==],
      lesson = { [==[if is an expression: both branches give a value.]==] },
      code = { [==[let temp = 72;]==], [==[let label = if temp > 70 { "hot" } {1} { "fine" };]==], [==[println!("{}", label);]==] },
      blanks = {
        { answers = { [==[else]==] }, hint = [==[The other branch.]==], wrong = { { [==[elif]==], [==[Rust writes else (and else if for more tests).]==] } } },
      },
      output = { [==[hot]==] },
    },
    {
      name = [==[Counting up]==], topic = [==[Loops]==],
      lesson = { [==[0..3 is a range from 0 up to but not including 3.]==] },
      code = { [==[for i in 0..{1} {]==], [==[    println!("{}", i);]==], [==[}]==] },
      blanks = {
        { answers = { [==[3]==] }, hint = [==[The range stops one before its end.]==], wrong = { { [==[2]==], [==[0..2 gives 0 and 1 only.]==] } } },
      },
      output = { [==[0]==], [==[1]==], [==[2]==] },
    },
    {
      name = [==[What comes back]==], topic = [==[Functions]==],
      lesson = { [==[The return type follows ->. The last expression without a semicolon is the value returned.]==] },
      code = { [==[fn twice(n: i32) {1} i32 {]==], [==[    n * 2]==], [==[}]==] },
      blanks = {
        { answers = { [==[->]==] }, hint = [==[A minus and a greater than sign.]==], wrong = { { [==[:]==], [==[Parameters use colons. The return type follows ->.]==] } } },
      },
      output = { [==[twice(21) is 42]==] },
    },
    {
      name = [==[A growing list]==], topic = [==[Collections]==],
      lesson = { [==[A Vec grows as items are pushed.]==] },
      code = { [==[let mut pins = vec![2, 3];]==], [==[pins.{1}(5);]==], [==[println!("{:?}", pins);]==] },
      blanks = {
        { answers = { [==[push]==] }, hint = [==[The method that adds to the end.]==], wrong = { { [==[append]==], [==[append joins two Vecs. push adds one item.]==] }, { [==[add]==], [==[Vec uses push.]==] } } },
      },
      output = { [==[[2, 3, 5]]==] },
    },
    {
      name = [==[Owned text]==], topic = [==[Strings]==],
      lesson = { [==[String is owned, growable text. String::from makes one from a literal.]==] },
      code = { [==[let s = String::{1}("led");]==], [==[println!("{}", s.len());]==] },
      blanks = {
        { answers = { [==[from]==] }, hint = [==[Make it from the literal.]==], wrong = { { [==[new]==], [==[String::new makes an empty String. String::from takes the text.]==] } } },
      },
      output = { [==[3]==] },
    },
    {
      name = [==[Moving ownership]==], topic = [==[Ownership]==],
      lesson = { [==[Each value has one owner. let b = a moves a String into b and a can no longer be used.]==] },
      code = { [==[let a = String::from("x");]==], [==[let b = a;]==], [==[println!("{}", {1});]==] },
      blanks = {
        { answers = { [==[b]==] }, hint = [==[Who owns the String now?]==], wrong = { { [==[a]==], [==[a was moved into b, so it can no longer be used.]==] } } },
      },
      output = { [==[x]==] },
    },
    {
      name = [==[Borrowing]==], topic = [==[Borrowing]==],
      lesson = { [==[& borrows a value without taking it. The owner keeps it afterwards.]==] },
      code = { [==[fn size(s: {1}String) -> usize { s.len() }]==], [==[// in main:]==], [==[let name = String::from("led");]==], [==[println!("{} {}", size(&name), name);]==] },
      blanks = {
        { answers = { [==[&]==] }, hint = [==[The sign for a reference.]==], wrong = { { [==[*]==], [==[* follows a reference. & makes one.]==] }, { [==[&mut]==], [==[&mut is only needed to change it.]==] } } },
      },
      output = { [==[3 led]==] },
    },
    {
      name = [==[Borrowing to change]==], topic = [==[Borrowing]==],
      lesson = { [==[&mut borrows a value so the function can change it. Only one &mut borrow may exist at a time.]==] },
      code = { [==[fn shout(s: {1} String) { s.push('!'); }]==], [==[// in main:]==], [==[let mut t = String::from("hi");]==], [==[shout(&mut t);]==], [==[println!("{}", t);]==] },
      blanks = {
        { answers = { [==[&mut]==] }, hint = [==[A reference that may change the value.]==], wrong = { { [==[&]==], [==[A plain & borrow cannot change the String.]==] }, { [==[mut]==], [==[It needs to be a reference: &mut.]==] } } },
      },
      output = { [==[hi!]==] },
    },
    {
      name = [==[Maybe a value]==], topic = [==[Option]==],
      lesson = { [==[Option is Some(value) or None. match must cover both.]==] },
      code = { [==[let pin: Option<u8> = None;]==], [==[let n = match pin {]==], [==[    Some(p) => p,]==], [==[    {1} => 0,]==], [==[};]==], [==[println!("{}", n);]==] },
      blanks = {
        { answers = { [==[None]==] }, hint = [==[The empty case.]==], wrong = { { [==[null]==], [==[Rust has no null. Option uses None.]==] }, { [==[_]==], [==[_ works too but None says what you mean.]==] } } },
      },
      output = { [==[0]==] },
    },
    {
      name = [==[Passing errors up]==], topic = [==[Error handling]==],
      lesson = { [==[A function returning Result can use ? to hand an error back to its caller.]==] },
      code = { [==[fn parse() -> Result<i32, std::num::ParseIntError> {]==], [==[    let n: i32 = "42".parse(){1};]==], [==[    Ok(n + 1)]==], [==[}]==] },
      blanks = {
        { answers = { [==[?]==] }, hint = [==[One mark that passes the error up.]==], wrong = { { [==[!]==], [==[! marks macros. ? passes an error up.]==] }, { [==[unwrap()]==], [==[unwrap would crash on an error instead of passing it up.]==] } } },
      },
      output = { [==[parse() is Ok(43)]==] },
    },
    {
      name = [==[Methods on a struct]==], topic = [==[Structs and traits]==],
      lesson = { [==[impl adds methods to a struct. &self borrows the value the method is called on.]==] },
      code = { [==[struct Led { pin: u8 }]==], [==[{1} Led {]==], [==[    fn pin(&self) -> u8 { self.pin }]==], [==[}]==] },
      blanks = {
        { answers = { [==[impl]==] }, hint = [==[Short for implementation.]==], wrong = { { [==[class]==], [==[Rust has no classes. Methods go in an impl block.]==] } } },
      },
      output = { [==[Led { pin: 13 }.pin() is 13]==] },
    },
    {
      name = [==[Shared behaviour]==], topic = [==[Structs and traits]==],
      lesson = { [==[A trait names methods types can share. impl Trait for Type provides them.]==] },
      code = { [==[trait Light { fn on(&self) -> bool; }]==], [==[impl Light {1} Led {]==], [==[    fn on(&self) -> bool { true }]==], [==[}]==] },
      blanks = {
        { answers = { [==[for]==] }, hint = [==[The word between the trait and the type.]==], wrong = { { [==[to]==], [==[The form is impl Light for Led.]==] } } },
      },
      output = { [==[any Led can be used as a Light]==] },
    },
    {
      name = [==[Iterators]==], topic = [==[Iterators]==],
      lesson = { [==[iter() walks a collection. map changes each item and collect gathers the results.]==] },
      code = { [==[let pins = vec![2, 3];]==], [==[let doubled: Vec<i32> = pins.iter().{1}(|p| p * 2).collect();]==], [==[println!("{:?}", doubled);]==] },
      blanks = {
        { answers = { [==[map]==] }, hint = [==[The adapter that changes every item.]==], wrong = { { [==[for_each]==], [==[for_each gives back nothing to collect. map does.]==] } } },
      },
      output = { [==[[4, 6]]==] },
    },
  },
}
