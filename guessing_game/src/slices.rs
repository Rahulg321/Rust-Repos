fn sample_enumerate(s: &String) {
    let bytes_array = s.as_bytes();
    println!("bytes are {:?}", bytes_array);
    for (i, &byte) in bytes_array.iter().enumerate() {
        // Print the byte's numeric value
        println!("Byte value: {:?}", byte);

        // Check if the byte is a valid ASCII character
        if byte.is_ascii() {
            // Convert the byte to a char and print it
            println!("Character representation: {:?}", byte as char);
        } else {
            println!("Non-ASCII byte");
        }

        // Check for a space character
        if byte == b' ' {
            println!("Found a whitespace at index {:?}", i);
        }

        if byte == b'!' {
            println!("Found a bang at index {:?}", i);
        }
    }
}

fn find_space_index(s: &String) {
    let bytes_array = s.as_bytes();
    for tup in bytes_array.iter().enumerate() {
        println!("tup {:?}", tup.1);
        println!("tup reference {}", *tup.1);
        if *tup.1 == b' ' {
            println!("found a whitespace at index {:?}", tup.0);
        }
    }
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn first_word_in_string(s: &String) {
    for c in s.chars() {
        if (c.is_whitespace() == true) {
            println!("Found a space at {c}");
        }

        println!("{c}");
    }
}

fn simple_slice() {
    let nums = [1, 2, 3, 4];

    // we reference only part of a collection, making it fast and lightweight to work with
    let slices = &nums[1..4];

    println!("{:?}", slices);

    for num in slices {
        println!("{:?}", num)
    }
}
