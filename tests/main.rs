use elor::prelude::*;

#[cfg(any(feature = "std", feature = "alloc"))]
#[test]
fn split () {
    use elor::iter::*;

    let alpha = vec![Left(1), Right(2.0), Left(3), Left(5), Right(4.0), Right(6.0)];
    let (left, right) = alpha.into_iter().split_either();

    println!("Left");
    for num in left {
        std::thread::sleep(std::time::Duration::from_secs_f32(rand::random()));
        println!("{}", num);
    }

    println!("Right");
    for num in right {
        std::thread::sleep(std::time::Duration::from_secs_f32(rand::random()));
        println!("{}", num);
    }
}

#[cfg(feature = "std")]
#[test]
fn split_arc () {
    use elor::iter::*;
    
    let alpha = vec![Left(1), Right(2.0), Left(3), Left(5), Right(4.0), Right(6.0)];
    let (left, right) = alpha.into_iter().split_either_arc();

    let handle = std::thread::spawn(move || {
        for num in left {
            std::thread::sleep(std::time::Duration::from_secs_f32(rand::random()));
            println!("Left: {}", num);
        }
    });

    for num in right {
        std::thread::sleep(std::time::Duration::from_secs_f32(rand::random()));
        println!("Right: {}", num);
    }

    handle.join().unwrap();
}