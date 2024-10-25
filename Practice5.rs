const SIZE: usize = 13; // Розмір квадрата

fn main() {
    for i in 0..SIZE {
        for j in 0..SIZE {
            // Малюємо квадрат і перехрестя по діагоналі
            if i == 0 || i == SIZE - 1 || j == 0 || j == SIZE - 1 {
                print!("*"); // Стінки квадрата
            } else if i == j || i + j == SIZE - 1 {
                print!("*"); // Діагоналі
            } else {
                print!(" "); // Пробіл
            }
        }
        println!(); // Перехід на новий рядок
    }
}

