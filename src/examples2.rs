
use rand::Rng;

pub fn examples2() {
    let mut rng = rand::thread_rng();

    println!("\n\ncalcula un número aleatorio entre 8 y 16");
    let num = rng.gen_range(8..17);
    println!("El número es {}", num);

    println!("\n\ndeclara un vector con num como longitud y asigna valores aleatorios entre 1 y 100");
    let mut vec = Vec::new();
    for _ in 0..num {
        vec.push(rng.gen_range(1..101));
    }
    for i in vec.iter() {
        println!("{}", i);
    }

    println!("\n\nmuestra el mayor y el menor de los enteros del vector");
    let mut max = vec[0];
    let mut min = vec[0];
    for i in 1..vec.len() {
        if vec[i] > max {
            max = vec[i];
        }
        if vec[i] < min {
            min = vec[i];
        }
    }
    println!("El mayor es {} y el menor es {}", max, min);

    println!("\n\nmuestra la suma de los enteros del vector");
    let mut sum = 0;
    for i in vec.iter() {
        sum += i;
    }
    println!("La suma es {}", sum);

    println!("\n\nmuestra la media de los enteros del vector");
    let avg = sum as f64 / vec.len() as f64;
    println!("La media es {}", avg);

    println!("\n\nmuestra los enteros del vector ordenados de menor a mayor");
    vec.sort();
    for i in vec.iter() {
        println!("{}", i);
    }

    println!("\n\nmuestra los enteros del vector ordenados de mayor a menor");
    vec.sort_by(|a, b| b.cmp(a));
    for i in vec.iter() {
        println!("{}", i);
    }

    println!("\n\nmuestra los enteros del vector ordenados de mayor a menor sin modificar el vector");
    let mut vec2 = vec.clone();
    vec2.sort_by(|a, b| b.cmp(a));
    for i in vec2.iter() {
        println!("{}", i);
    }

    println!("\n\nmuestra los enteros del vector ordenados de mayor a menor sin modificar el vector y sin clonar el vector");
    let mut vec3 = vec.iter().cloned().collect::<Vec<i32>>();
    vec3.sort_by(|a, b| b.cmp(a));
    for i in vec3.iter() {
        println!("{}", i);
    }

}