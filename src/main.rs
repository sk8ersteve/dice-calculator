use std::vec::Vec;
use std::io;

fn find_combos(total: i32, num_dice: i32, min_die: i32) -> Vec<Vec<i32>> {
    let mut results = Vec::new();
    if num_dice <= 1 {
        if num_dice == 1 && min_die <= total && total <= 6 {
            let mut arr = Vec::new();
            arr.push(total);
            results.push(arr)
        }
        return results;
    }
    for i in min_die..7 {
        let temp = find_combos(total - i, num_dice - 1, i);
        for mut arr in temp {
            arr.insert(0, i); // yes this is "inefficient" but we don't care, increasing order is more important
            results.push(arr);
        }
    }
    return results;
}

fn main() -> io::Result<()> {
    println!("How many dice?: ");
    let num_dice;
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        if let Ok(num) = buffer.trim().parse::<i32>() {
            if num > 0 {
                num_dice = num;
                break;
            }
        }
        println!("Invalid number. Try again.");
    }
    loop {
        println!("Enter total:");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        if let Ok(total) = buffer.trim().parse::<i32>() {
            let res = find_combos(total, num_dice, 1);
            println!("Found {} combinations", res.len());
            for arr in res {
                println!("{:?}", &arr);
            }    
        } else {
            println!("All done!");
            return Ok(());
        }
    }
}
