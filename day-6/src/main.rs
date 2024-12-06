pub fn longer_wish<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    let string1 = s1.trim().chars().count();
    let string2 = s2.trim().chars().count();

    if string1 > string2 {
        Some(s1)
    } else if string2 > string1 {
         Some(s2)
    } else {
        None
    }
}

fn main() {

}
