use generator::generator::generate;

fn main() {
    let maze =generate("circle", &[9, 2], 75);

    println!("{}", maze);
}