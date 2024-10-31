Ultimate Goal
    Be able to type the following command
    davinC picture.blah ...
    And have it spit out an executable defined by the picture

What picture format to support?
    There are many different file formats, support the most popular or stick with one?
    It's simpler to stick with one that's lossless but it is more itneresting to be able to
    uplaod any picture

Discrete or Explicit?
    Do I want you to be able to look at the code picture and see that its definitively
    meaningful or have it be discrete and embeddable into any picture?
    I like the idea of it being discrete because in the end of the day discrete can be explicit
    if you just point it out, and the inverse is not true.

    I've decided on Discrete for the reasoning above, Discrete -> Explicit is always possible
    but Explicit -> Discrete is not always. Plus the reveal is pretty neat, picture of Mona
    Lisa suddenly the background fades and you see this complex web of non sense.

    Nevermind, I want to to explicit, if you went to an ancient tomb and saw just a tomb wall that isn't interesting.
    I want it to be like oh shit this means nothing and then you can gradually decipher it. I want it you show the executable
    and then they're like can I see the code and you open up a indecipherable mess but it's clear it's something. When it's all
    in front of you I almost think it's more unbelievable than if it's hidden. Therefor I'm gona make it explicit to achieve my desired affect.
    It's important to point out that a Discrete Approach would also be too arduous to code in because it would likely involve color picking for nearly every pixel
    you draw or implementing an intermediary burn step that burns your code into the image and then you can't read it either. It would also seem to be really easy to implement,
    It's easy to be like if there is a 1 pixel difference here than move, 2 then create variable, etc. I think it's more fantastical to see the spectacle.

Interpreter or Compiler?
    Interpreter is leagues easier but I'm learning assembly so I 100% want to make a compiler.
    But it will be easier to debug if I make an interpreter first and then turn that into a
    compiler, so I think that will likely be the route that I go down. I would rather have a
    working language that I can bug test and then do a compiler than trying to figure out if
    it's an issue with the language or the compiler.

Translater or include Assembler and Linker?
    This is a later in the line decision but do I want to just make it translate into assembly
    or also assemble and link it. I don't have to answer this quickly because in the end of the
    day I will have to translate it before I assemble and link it, but it's something to take
    note of

Default print() prints a .png
    I think it would be hilarious if the print function returned a picture with the text drawn in it.
    It would be even funnier if the writing to terminal was like type() or something.

Written in DavinC
    This is just a note but the end goal of this project is to be able to write the compiler /
    interpreter in DavinC so I should make sure I design for that in mind. 

I am not allowed to use my mouse during this process
    It's time I finally just get used to VIM keybindings and get fluid to make my coding
    faster, no matter how time consuming or arduous it becomes

Iterations
    It's important to note that I have remade this 4 times now and I intend for this to be the last time.
    This means that I will not stop doing this until I can type davinc picture.blah ... and get that thing to print hello world.

To target LLVM or not
    I think it would be hilarious to make it super optimized and target LLVM, but I want to write the compiler.
    I especially wanna write the compiler in davinC and I won't be able to create LLVM in davinC within 1000 years.

Language Features:

The main design decision is how the hell am I gonna put these symbols into a picture...
Could be lame and just make some sort of direct color to meaning translation in these random spots means different instructions based on the difference.
However, that would be incredibly lame and I would have to stare at a translation table all day to code
The only conceivable way for me to achieve this then is to be forced to have some sort of symbols, and some way to go between those symbols.
I guess that's the basis of any programming language when you think of it, you need a way of representing information and a way to infer order.

Symbols will have to be able to be recognized at a bare minimum in order to interpret the instructions to be executed. (I know that's an obtuse way to put it)
Therefor before I identify how I'm gonna represent information no matter what, I'm gonna have to create the identifier and definition system first.
Although it would be simpler to form the identifier around the symbols, if I can make just a universal identifier than that would fit any symbols.
It seems like this is a lot more work than just figuring out the symbols first and then specifying  it seems like an interesting problem so I want to do it.
Essentially what I'm looking for is a 2d Regex, that you can define a set of rules and putting a 2d data in will say yes or no.

So time to learn how Regex Works!!!!

https://en.wikipedia.org/wiki/Regular_expression
https://github.com/luvit/pcre/tree/master
This is a good resource
https://rust-leipzig.github.io/regex/2017/03/28/comparison-of-regex-engines/
https://swtch.com/~rsc/regexp/regexp1.html
https://swtch.com/~rsc/regexp/regexp2.html


Language Resources
https://mukulrathi.com/create-your-own-programming-language/llvm-ir-cpp-api-tutorial/
