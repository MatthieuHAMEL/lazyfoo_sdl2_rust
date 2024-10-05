Tutorial link: https://lazyfoo.net/tutorials/SDL/01_hello_SDL/index2.php

The goal of this first tutorial is to init the SDL and to display an empty, white window.

Only "tricky" thing in the code : Window::surface() function borrows the application's event pump to prevent the window from being resized during the lifetime of the window surface, otherwise we'd have a dangling pointer on the surface. So I create a fake event pump (we don't need it in this tutorial) just to comply with that API.

Error handling in programming, and the relevance of error messages, is a subject that's important to me, and I believe it's the No. 1 solution for developping and maintaining larger systems efficiently. So I've also added a function that throws a MessageBox in the event of an error that I consider fatal, during initialization. This may make it easier - always bearing in mind a larger system, where error sources are legion - to recognize configuration problems, as opposed to a crash or a simple program exit. 
