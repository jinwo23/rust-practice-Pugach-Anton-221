fn main() {
    let mut solutions = vec![];

    // Генеруємо всі унікальні перестановки чисел від 1 до 8
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];

    // Створюємо усі перестановки за допомогою рекурсії
    fn permute(nums: &mut Vec<i32>, start: usize, solutions: &mut Vec<(i32, i32, i32)>) {
        if start == nums.len() {
            // Оскільки ми маємо перестановку, перевіряємо її умови
            let [m, u, x, a, s, l, o, n] = [
                nums[0], nums[1], nums[2], nums[3], nums[4], nums[5], nums[6], nums[7]
            ];

            let muxa = m * 1000 + u * 100 + x * 10 + a;
            let slon = s * 1000 + l * 100 + o * 10 + n;

            if muxa * a == slon {
                solutions.push((muxa, a, slon));
            }
        } else {
            for i in start..nums.len() {
                nums.swap(start, i);  // Обмін елементів
                permute(nums, start + 1, solutions);  // Рекурсивний виклик
                nums.swap(start, i);  // Повернення на попередній стан
            }
        }
    }

    // Генеруємо всі перестановки і перевіряємо умови
    let mut nums = numbers.clone();
    permute(&mut nums, 0, &mut solutions);

    // Виведення результатів
    for (muxa, a, slon) in &solutions {
        println!("{:4} x {:4} = {:4}", muxa, a, slon);
    }
    println!("Загальна кількість розв'язків: {}", solutions.len());
}
