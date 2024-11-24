use termion::cursor;

pub fn draw(x: usize, y: usize) -> String {
    println!("{}", cursor::Goto(x.try_into().unwrap(), y.try_into().unwrap()));
    println!("{:?}", std::time::SystemTime::now());
    let mut output = String::from("");
    return output;
}
