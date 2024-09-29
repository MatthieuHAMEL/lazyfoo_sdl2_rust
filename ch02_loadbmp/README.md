Tutorial link: https://lazyfoo.net/tutorials/SDL/02_getting_an_image_on_the_screen/index.php

You'll need to create a “data” folder in which to place a test.bmp bitmap image (see source code). 
Otherwise, this will be an opportunity to test my function (outside the scope of this chapter) which prompts a messagebox in error cases that I consider to be "fatal".

Unlike other ports I've seen, I'm sticking to creating SDL_Surfaces and playing with SDL_Surfaces (Surface and SurfaceRef in Rust) - in keeping with the aims of the tutorial. 

The Window::surface() function borrows the application's event pump to prevent the window from being resized during the lifetime of the window surface, otherwise we'd have a dangling pointer on the surface. So I create a fake event pump to comply with the Rust API. 
