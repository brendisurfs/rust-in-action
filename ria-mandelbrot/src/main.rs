use num::complex::Complex;
fn main() {
    println!("mandelbrot");
    let mand = calculate(2.0, 1.0, -1.0, 1.0, 100, 24, 1000);
    render(mand);
}

fn calculate(
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    width: usize,
    height: usize,
    max_iters: usize,
) -> Vec<Vec<usize>> {
    let mut rows: Vec<_> = Vec::with_capacity(width);
    for img_y in 0..height {
        let mut row: Vec<usize> = Vec::with_capacity(height);
        for img_x in 0..width {
            let xp = (img_x as f64 / width as f64);
            let yp = (img_y as f64 / height as f64);
            let cx = x_min + (x_max - x_min) * xp;
            let cy = y_min + (y_max - y_min) * yp;
            let escaped_at = mb_at_point(max_iters, cx, cy);
            row.push(escaped_at);
        }
        rows.push(row);
    }
    rows
}
fn mb_at_point(max_iters: usize, cx: f64, cy: f64) -> usize {
    let mut z = Complex::new(0.0, 0.0);
    let c = Complex::new(cx, cy);

    for i in 0..=max_iters {
        if z.norm() > 2.0 {
            return i;
        }
        z = z * z + c; // repeatedly mutates z to check whether c lies within the mandelbrot set.
    }
    max_iters
}

fn render(escape_vals: Vec<Vec<usize>>) {
    for row in escape_vals {
        let mut line = String::with_capacity(row.len());
        for col in row {
            let val = match col {
                0..=2 => ' ',
                2..=5 => '.',
                5..=10 => '🤏',
                11..=30 => '👽',
                30..=100 => '🤪',
                100..=200 => '😡',
                200..=400 => '🤑',
                400..=700 => '😂',
                _ => '🤠',
            };
            line.push(val);
        }
        println!("{}", line);
    }
}
