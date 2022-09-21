extern crate clipboard;
use clipboard::{ClipboardContext, ClipboardProvider};

fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    let empty_string = "".to_string();
    let mut last_contents = [
        empty_string.clone(),
        empty_string.clone(),
        empty_string.clone(),
        empty_string.clone(),
        empty_string,
    ];
    let mut num_previous = 0;

    loop {
        if let Ok(content) = ctx.get_contents() {
            if !last_contents.contains(&content) {
                let index = num_previous % last_contents.len();
                last_contents[index] = content.to_string();
                println!("{}", content);
            }
            num_previous += 1;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
