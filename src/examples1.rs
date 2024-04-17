
use rand::Rng;

pub fn examples1() {
    

    let mut arr = fun_decl();
    
    let (max,min) = fun_maxmin(arr);
    println!("El mayor es {} y el menor es {}", max, min);

    let sum = fun_sum(arr);
    println!("La suma es {}", sum);

    let avg= fun_avg(sum, arr);
    println!("La media es {}", avg);

    fun_sortasc(&mut arr);

    fun_sortdesc(arr);
}

type TypeArr5i = [i32; 5];

fn fun_sortdesc(mut arr: TypeArr5i) {
    println!("\n\nmuestra los enteros del array ordenados de mayor a menor");
    arr.sort_by(|a, b| b.cmp(a));
    for i in arr.iter() {
    println!("{}", i);
    }
}

fn fun_sortasc(arr: &mut TypeArr5i) {
    println!("\n\nmuestra los enteros del array ordenados de menor a mayor");
    arr.sort();
    for i in arr.iter() {
        println!("{}", i);
    }
}

fn fun_avg(sum: i32, arr: TypeArr5i) -> f64 {
    println!("\n\nmuestra la media de los enteros del array");
    let avg = sum as f64 / arr.len() as f64;
    avg
}

fn fun_sum(arr: TypeArr5i) -> i32 {
    println!("\n\nmuestra la suma de los enteros del array");
    let mut sum = 0;
    for i in arr.iter() {
        sum += i;
    }
    sum
}

fn fun_maxmin(arr: TypeArr5i) -> (i32, i32){
    println!("\n\nmuestra el mayor y el menor de los enteros del array");
    let mut max = arr[0];
    let mut min = arr[0];
    for i in 1..arr.len() {
        if arr[i] > max {
            max = arr[i];
        }
        if arr[i] < min {
            min = arr[i];
        }
    }
    (max, min)
}

fn fun_decl() -> TypeArr5i {
    let mut rng = rand::thread_rng();
    println!("\n\ndeclara un array de 5 enteros aleatorios y muestra su contenido");
    let mut arr = [0; 5];
    for i in 0..arr.len() {
        arr[i] = rng.gen_range(1..101);
    }
    for i in arr.iter() {
        println!("{}", i);
    }
    arr
}