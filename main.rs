fn e_posibil(package_weights: Vec<u32>) -> i32 {
    let mut suma:u32 = 0;
    if package_weights[package_weights.len()-1] > 15 {
        return -1;
    }
    for i in package_weights {
        suma += i;
    }
    if suma as f32 / 5.0 > 15.0 {
        return -1;
    } else {
        return 1;
    }
}

//utimul cu toate. daca e 15 - stop. dace > 15 se ia singur. daca <15 se mai aduna un element

fn bubble_sort(mut package_weights: Vec<u32>) -> Vec<u32> {
    for j in 0..package_weights.len()-1 {
        for i in 0..package_weights.len()-j {
            if package_weights[i] > package_weights[i+1] {
                let temp = package_weights[i];
                package_weights[i] = package_weights[i+1];
                package_weights[i+1] = temp;
            }
        }
    }
}

fn facem(mut package_weights: Vec<u32>) {
    let mut last = package_weights.len()-1;
    if package_weights[last] == 15 {
        println!("day1: (15)");
        if last >= 0 {
            package_weights.pop();
            days+=1;
        }
    } else /*last < 15*/ {
        let mut suma = package_weights[last];
        for i in 0..=last {
            if package_weights[0]+package_weights[last] == 15 {
                
            }
            suma += package_weights[i];

        }

    }
}

fn main() {
    let mut package_weights = [4,2,3,1,5,6,7,8,9,10];
    let mut days = 5;
    let capacitatea = 15;
    if e_posibil(package_weights) == 1 {
        package_weights = bubblesort(package_weights);

    } else {
        println!("e imposibil");
    }
}

