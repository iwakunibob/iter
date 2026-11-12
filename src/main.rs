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
    print_elements(&colors[1..3]);
    let upper_colors = to_uppercase_new(&colors);
    print_elements(&upper_colors);
    // shorten_strings(&mut colors);
    // print_elements(&colors);
    shorten_strings(&mut colors[1..3]);
    print_elements(&colors[1..3]);           
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
