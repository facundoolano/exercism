pub fn raindrops(n: u32) -> String {
    let factor3 = is_factor(n, 3);
    let factor5 = is_factor(n, 5);
    let factor7 = is_factor(n, 7);

    let mut output = String::from("");
    if factor3 {
        output += "Pling";
    }
    if factor5 {
        output += "Plang";
    }
    if factor7 {
        output += "Plong";
    }
    if !factor3 && !factor5 && !factor7 {
        output = n.to_string();
    }

    output
}

fn is_factor(n: u32, factor: u32) -> bool {
    n % factor == 0
}
