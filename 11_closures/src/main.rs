use std::collections::HashMap;

// Fn is a trait that is implemented by all closures
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(&v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

fn expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly...");
    std::thread::sleep(std::time::Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32, cacher: &mut Cacher<impl Fn(u32) -> u32>) {
    if intensity < 25 {
        println!("Today, do {} pushups!", cacher.value(intensity));
        println!("Next, do {} situps!", cacher.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", cacher.value(intensity));
        }
    }
}

fn main() {
    let intensity = 10;
    let random_number = 7;
    let mut expensive_result = Cacher::new(|num| expensive_calculation(num));

    // Closure that captures the intensity variable
    let equal_to_x = |z: u32| z == intensity;
    let y = 10;
    assert!(equal_to_x(y));

    generate_workout(intensity, random_number, &mut expensive_result);
    generate_workout(12, random_number, &mut expensive_result);
    generate_workout(10, random_number, &mut expensive_result);
}

mod tests {

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6);

        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4]);
    }
}
