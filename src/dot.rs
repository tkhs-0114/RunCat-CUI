use termion::cursor::Goto;

pub fn create<'a>(pattern: &'a[&'a str], x: usize, y: usize) ->  String {
    let dot = '0';

    let base = '\u{2800}' as u32;

    let h: usize = pattern.len();
    let w: usize = pattern[0].len();

    let mut brailles = vec![vec![0; (w+1)/2]; (h+2)/3];

    /*
        000001 ->   1 0
                    0 0
                    0 0

        000010 ->   0 0
                    1 0
                    0 0

        010001 ->   1 0
                    0 1
                    0 0
    */
    for (i, s) in pattern.iter().enumerate() {
        for (j, c) in s.chars().enumerate() {
            if c == dot {
                let base: i32 = 2;
                let exp: u32 = ((i%3+1)+((j%2)*3) - 1).try_into().unwrap();
                brailles[i/3][j/2] += base.pow(exp);
            }
        }
    }
    let mut output = String::from("");
    for (i, s) in brailles.iter().enumerate() {
        output = format!("{}{}", output, Goto(x.try_into().unwrap(), (y+i).try_into().unwrap()));
        for c in s {
            let index: u32 = *c as u32;
            output.push_str(&std::char::from_u32(base + index).unwrap().to_string());
        }
    }
    output
}
