use std::collections::HashMap; // Импортируем HashMap из стандартной библиотеки

fn main() {
    // Инициализируем вектор температур
    let temperatures = vec![-25.4, -27.0, 13.0, 19.0, 15.5, 24.5, -21.0, 32.5];

    // Вызываем функцию для сортировки температур в интервалы
    let intervals = sort_temperatures_in_intervals(&temperatures);

    // Вывод результатов
    for ((lower, upper), temps) in &intervals {
        print!("[{}, {}): {:?}; ", lower, upper, temps);
    }
}

// Функция для сортировки температур в интервалы
fn sort_temperatures_in_intervals(temperatures: &[f64]) -> HashMap<(i32, i32), Vec<f64>> {
    // Создаем хеш-таблицу для хранения интервалов и соответствующих температур
    let mut intervals: HashMap<(i32, i32), Vec<f64>> = HashMap::new();

    // Проходим по каждой температуре в векторе
    for &temp in temperatures {
        // Вычисляем нижнюю границу интервала
        let lower_bound = (temp / 10.0).floor() as i32 * 10;
        // Вычисляем верхнюю границу интервала
        let upper_bound = lower_bound + 10;

        // Заполняем хеш-таблицу
        intervals
            .entry((lower_bound, upper_bound)) // Определяем интервал как ключ
            .or_insert_with(Vec::new)          // Создаем новый вектор, если запись отсутствует
            .push(temp);                       // Добавляем температуру в вектор
    }

    intervals // Возвращаем хеш-таблицу интервалов
}
