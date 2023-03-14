

pub fn clear_screen() {
    println!("\x1b[2J\x1b[H\x1b[?25l");
}

pub fn clear_cursor_pos() {
    print!("\x1b[H");
}

pub fn show_cursor() {
    println!("\x1b[?25h");
}
