fn main() {

    let mut s  = String::from("hello ");


    let s1 = addtext(&mut s);

    let _hello = &s1[..5];
    let _world = &s1[6..];


    let slice = firstword(&s1);

    println!{"{slice}"}
}


fn firstword(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[..i];
        }
    }
    &s[..]
}

fn addtext(s: &mut String) -> &mut String {

    s.push_str("world");

    s

}