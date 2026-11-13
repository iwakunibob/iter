fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];
    let mut colors_iter = colors.iter();
    println!("{:#?} ", colors_iter.next());
    println!("{:#?} ", colors_iter.next());
    println!("{:#?} ", colors_iter.next());
    println!("{:#?} ", colors_iter.next());
    print_elements(&colors);
    let found_color = find_color_or(&colors, "bl", "Orange");
    println!("{} ", found_color);
    print_elements(&colors[1..3]);
    let upper_colors = to_uppercase_new(&colors);
    print_elements(&upper_colors);
    // shorten_strings(&mut colors);
    // print_elements(&colors);
    shorten_strings(&mut colors[1..3]);
    print_elements(&colors[1..3]);  
    let exploded = explode(&colors);
    println!("{:#?} ", exploded);
    let mut destination = vec![];
    move_elements(colors, &mut destination);
    print_elements(&destination);
}

fn find_color_or(elements:&[String], search: &str, fallback: &str) -> String {
    elements
        .iter()
        .find(|element| element.contains(search))
        .map_or(String::from(fallback),|element| element.to_string())
}
fn print_elements(elements: &[String]) {
    for element in elements { print!("#{}, ", element);}
    println!();
    // .for each calls next and is iterator consumer because consumes values 
    elements.iter().for_each(|element| print!("{}%, ", element));
    println!();
    // .map() is iterator adapter does not call next just a process
    elements
        .iter()
        .map(|element| format!("{},{}. ", element, element))
        .for_each(|element| print!("${} ", element));
        println!();
}
fn shorten_strings(elements: &mut [String]) {
    elements.iter_mut().for_each(|element| element.truncate(1))
}
fn to_uppercase_new(elements: &[String]) -> Vec<String> {
    elements.iter().map(|element| element.to_uppercase()).collect()
}
fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|element| vec_b.push(element));
}
fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements
        .iter()
        .map(
            |element| element.chars().map(
                |chr| chr.to_string()).collect()
        ).collect()
}
