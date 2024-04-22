// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn trim_me(input: &str) -> String {
     // TODO: Remove whitespace from both ends of a string!
     let mut i = 0;
     let mut j = -1;
     let mut cnt = 0;
     for c in input.chars() {
        cnt += 1;
        if c != ' ' && j == -1 {
            j = cnt;
        }
        if i == 0 && c == ' ' {
            i = cnt;
        }
        else if i != 0 && c != ' ' {
            i = 0;
        }
     }
     cnt = 0;
     let mut ans = "";
     for c in input.chars() {
        cnt += 1;
        if cnt < i && cnt >= j {
            ans.to_string().push_str(&c.to_string());
        }
    }
    ans.to_string()

}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    input.to_string().push_str(" world!");
    input.to_string()
}

fn replace_me(input: &str) -> String {
    // TOD: Replace "cars" in the string with "balloons"!O
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
