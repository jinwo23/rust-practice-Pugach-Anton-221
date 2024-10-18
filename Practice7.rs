fn main() {
    let triangle_count = 5;
    draw_tree(triangle_count);
}

fn draw_tree(triangle_count: usize) {
    let mut total_height = 0;
    let max_width = 2 * triangle_count + 1;

    for i in 1..=triangle_count {
        let height = i + 1;
        total_height += height;
        for row in 0..height {
            let stars = 2 * row + 1;
            let spaces = (max_width - stars) / 2;
            println!("{}{}", " ".repeat(spaces), "*".repeat(stars));
        }
    }
}
