/*
 * Test Edje module.
 *

 * This test requires a source Edje file.
 * Save the following code in a file with name 'edje_example.edc'
 * Compile with: edje_cc edje_example.edc
 * Run the test in the same directory of this file.


collections {
   group {
      name: "my_example"; // must be the same as in edje_example.c

      parts {
         part {
            name: "background";
            type: RECT; // plain boring rectangle
            mouse_events: 0; // we don't need any mouse event on the background

            // just one state "default"
            description {
               state: "default" 0.0; // must always exist
               color: 255 255 255 255; // white

               // define part coordinates:
               rel1 { // top-left point at (0, 0) [WIDTH * 0 + 0, HEIGHT * 0 + 0]
                  relative: 0.0 0.0;
                  offset: 0 0;
               }
               rel2 { // bottom-right point at (WIDTH * 1.0 - 1, HEIGHT * 1.0 - 1)
                  relative: 1.0 1.0;
                  offset: -1 -1;
               }
            }
         }

         part {
            name: "text";
            type: TEXT;
            mouse_events: 1; // we want to change the color on mouse-over

            // 2 states, one "default" and another "over" to be used
            // on mouse over effect

            description {
               state: "default" 0.0;
               color: 255 0 0 255; // red

               // define part coordinates:

               rel1 { // top-left at (WIDTH * 0.1 + 5, HEIGHT * 0.2 + 10)
                  relative: 0.1 0.2;
                  offset: 5 10;
               }
               rel2 { // bottom-right at (WIDTH * 0.9 - 6, HEIGHT * 0.8 - 11)
                  relative: 0.9 0.8;
                  offset: -6 -11;
               }

               // define text specific state details
               text {
                  font: "Sans"; // using fontconfig name!
                  size: 10;
                  text: "hello world";
               }
            }

            description {
               state: "over" 0.0;
               inherit: "default" 0.0; // copy everything from "default" at this point

               color: 0 255 0 255; // override color, now it is green
            }
         }

         // do programs to change color on text mouse in/out (over)
         programs {
            program {
               // what triggers this program:
               signal: "mouse,in";
               source: "text";

               // what this program does:
               action: STATE_SET "over" 0.0;
               target: "text";

               // do the state-set in a nice interpolation animation
               // using linear time in 0.1 second
               transition: LINEAR 0.1;
            }

            program {
               // what triggers this program:
               signal: "mouse,out";
               source: "text";

               // what this program does:
               action: STATE_SET "default" 0.0;
               target: "text";

               // do the state-set in a nice interpolation animation
               // using linear time in 0.1 second
               transition: LINEAR 0.1;
            }
         }
      }
   }
}

 */

extern crate efl;

use efl::types::int;
use efl::ecore;
use efl::edje;
use efl::evas;

static WIDTH: int = 640;
static HEIGHT: int = 480;


fn main() {

    ecore::evas_init();
    edje::init();

    let window: Box<ecore::EcoreEvas> =
        ecore::evas_new(None, 0, 0, WIDTH, HEIGHT, "");

    let canvas = ecore::evas_get(&window);
    
    /* Instantiate edje object */
    let edje: Box<evas::EvasObject> = edje::object_add(&canvas);

    edje::object_file_set(&edje, "./test.edj", "main");

    edje::object_part_text_set(&edje, "text", "Rust-Efl Edje Example!");

    evas::object_move(&edje, (0, 0));
    evas::object_resize(&edje, WIDTH, HEIGHT);
    evas::object_show(&edje);

    ecore::evas_show(&window);
    ecore::main_loop_begin();
    
    evas::object_del(&edje);
    ecore::evas_free(&window);
    
    edje::shutdown();
    ecore::evas_shutdown();

}
