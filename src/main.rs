extern crate rho;

use rho::Editor;
use std::char;
use std::default::Default;

fn main()
{
    let e = Editor::new();
    e.run();

    // let mut buffer = Buffer::new();
    // // let mut x = 0;
    // // let mut y = 0;
    // // let mut buffer = String::new();
    // // // getmaxyx

    // /* Setup ncurses. */
    // initscr();
    // raw();

    // // curs_set(CURSOR_VISIBILITY::CURSOR_VERY_VISIBLE);
    // /* Allow for extended keyboard (like F1). */
    // keypad(stdscr, true);
    // noecho();


    // /* Wait for input. */
    // loop {
    //     refresh();
    //     let ch = getch();
    //     match ch {
    //         0x1b => break, //Escape key
    //         KEY_F1 => break,
    //         KEY_EXIT => break,
    //         KEY_BACKSPACE => buffer.back_space(),
    //         KEY_ENTER => buffer.insert_line(),
    //         0xA => buffer.insert_line(),
    //         KEY_UP => buffer.move_up(),
    //         KEY_DOWN => buffer.move_down(),
    //         KEY_LEFT => buffer.move_left(),
    //         KEY_RIGHT => buffer.move_right(),
    //         _ => match char::from_u32(ch as u32) {
    //             Some(character) => buffer.insert(character),
    //             _ => ()
    //         }
    //     }

    //     // clear();
    //     mvprintw(0, 0, buffer.get_contents().as_ref());
    //     mv(buffer.get_y(), buffer.get_x());
    // }

    // endwin();
}
