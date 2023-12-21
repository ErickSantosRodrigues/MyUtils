


pub enum Colors {
    Red,
    Blue,
    Green,
    White,
    Black,
}


pub fn to_color(s:&String, color: Colors) -> String {
    match color {
        Colors::Red => {
            return format!("\x1b[31m{}\x1b[0m", s);
        }
        Colors::Blue => {
            return format!("\x1b[34m{}\x1b[0m", s);
        }
        Colors::Green => {
            return format!("\x1b[32m{}\x1b[0m", s);
        }
        Colors::White => {
            return format!("\x1b[37m{}\x1b[0m", s);
        }
        Colors::Black => {
            return format!("\x1b[30m{}\x1b[0m", s);
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_red() {
        let s = String::from("Hello");
        let color = Colors::Red;
        let result = to_color(&s, color);
        assert_eq!(result, "\x1b[31mHello\x1b[0m");
    }

    #[test]
    fn test_blue() {
        let s = String::from("Hello");
        let color = Colors::Blue;
        let result = to_color(&s, color);
        assert_eq!(result, "\x1b[34mHello\x1b[0m");
    }

    #[test]
    fn test_green() {
        let s = String::from("Hello");
        let color = Colors::Green;
        let result = to_color(&s, color);
        assert_eq!(result, "\x1b[32mHello\x1b[0m");
    }

    #[test]
    fn test_white() {
        let s = String::from("Hello");
        let color = Colors::White;
        let result = to_color(&s, color);
        assert_eq!(result, "\x1b[37mHello\x1b[0m");
    }

    #[test]
    fn test_black() {
        let s = String::from("Hello");
        let color = Colors::Black;
        let result = to_color(&s, color);
        assert_eq!(result, "\x1b[30mHello\x1b[0m");
    }
}

