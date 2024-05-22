# RustEngine
 
***
## **Requirements**

To run the code a few libraries will need to be downloaded and copied to your Rust library folder.

The libraries folder for your computer may differ but will most likely be located at:

C:\Users\"user"\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib

Go to:
https://github.com/libsdl-org/SDL/releases/tag/release-2.30.3

And download SDL2-devel-2.30.3-VC.zip. Unzip and go to lib -> 64 and copy the .lib files into your rust library folder.

Go to:
https://github.com/libsdl-org/SDL_ttf/releases

And download SDL2_ttf-devel-2.22.0-VC.zip. Unzip and go to lib -> 64 and copy the .lib files into your rust library folder.

***
## **Playing**

From the folder where this read me exist, open a terminal and run "cargo run".

The terminal will first ask for your name. Type your name and press enter. The program will then open the game as a window.

Use your mouse cursor to click on the game window to control the player character.

### **- Controls**

Use A and D to move the player character left and right.

Use SPACE to make the player character jump into the air.

Use ESCAPE to close the game window.

***