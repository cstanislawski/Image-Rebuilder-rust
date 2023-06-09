use image::{DynamicImage, GenericImageView, GenericImage};
use rand::Rng;
use std::env;

// Check if file exists
fn file_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

// Check if file is of extension .png, .jpg, .jpeg
fn is_image(path: &str) -> bool {
    let ext = std::path::Path::new(path)
    .extension()
    .unwrap()
    .to_str()
    .unwrap()
    .to_lowercase();
    ext == "png" || ext == "jpg" || ext == "jpeg"
}

// Read file
fn read_image(path: &str) -> image::DynamicImage {
    image::open(path).unwrap()
}

// Write file
fn write_image(path: &str, img: &DynamicImage) {
    img.save(path).unwrap();
}

// Read user input informing the user of what to input
fn read_input(s: &str) -> String {
    println!("{}", s);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// Create a black image of dimensions width x height and save it to path
fn create_black_image(path: &str, width: u32, height: u32) {
    let img = image::DynamicImage::new_rgb8(width, height);
    write_image(path, &img);
}

// Generate N amount of random RGBA pixels
fn generate_random_pixels(n: u32) -> Vec<(u8, u8, u8, u8)> {
    // Pre-allocate the vector with the desired capacity
    let mut pixels = Vec::with_capacity(n as usize);
    // Use a single random number generator instead of calling random() multiple times
    let mut rng = rand::thread_rng();
    for _ in 0..n {
    // Generate a random u32 and split it into four bytes
    let rgba = rng.gen::<u32>();
    let r = (rgba >> 24) as u8;
    let g = (rgba >> 16) as u8;
    let b = (rgba >> 8) as u8;
    let a = rgba as u8;
    // Push the pixel to the vector
    pixels.push((r, g, b, a));
    }
    pixels
}

// Calculate color distance between two pixels, as the Euclidean distance in RGBA space
fn calculate_distance(p1: (u8, u8, u8, u8), p2: (u8, u8, u8, u8)) -> u32 {
    let r = (p1.0 as i32 - p2.0 as i32).pow(2) as f32;
    let g = (p1.1 as i32 - p2.1 as i32).pow(2) as f32;
    let b = (p1.2 as i32 - p2.2 as i32).pow(2) as f32;
    let a = (p1.3 as i32 - p2.3 as i32).pow(2) as f32;
    (r + g + b + a).sqrt() as u32
}

// Find the closest pixel to a givel pixel out of a vector of pixels
fn find_closest_pixel(pixel: (u8, u8, u8, u8), pixels: &Vec<(u8, u8, u8, u8)>) -> (u8, u8, u8, u8) {
    if pixels.len() == 0 {
    return (0, 0, 0, 0); // Return black pixel if there are no pixels
    }
    let mut closest_pixel = pixels[0];
    let mut closest_distance = calculate_distance(pixel, pixels[0]);

    for p in pixels {
    let distance = calculate_distance(pixel, *p);
    if distance < closest_distance {
    closest_pixel = *p;
    closest_distance = distance;
    }
    // if distance < 50.0 {
    // return [closest_pixel.0, closest_pixel.1, closest_pixel.2, closest_pixel.3]
    // }
    }
    closest_pixel
}

// Calculate the average color of a vector of pixels
fn calculate_average_color(pixels: &Vec<(u8, u8, u8, u8)>) -> (u8, u8, u8, u8) {
    // Use iterators and fold to avoid mutable variables and looping
    let (r, g, b, a) = pixels.iter().fold((0u32, 0u32, 0u32, 0u32), |(r, g, b, a), p| {
    (r + p.0 as u32, g + p.1 as u32, b + p.2 as u32, a + p.3 as u32)
    });
    let n = pixels.len() as u32;
    // Use saturating_div to avoid overflow and cast to u8
    ((r.saturating_div(n)) as u8, (g.saturating_div(n)) as u8, (b.saturating_div(n)) as u8, (a.saturating_div(n)) as u8)
   }
   
   
   // Color each pixel in the output image with the closest pixel in the input image
   // the u32 in input_pixels is the x & y coordinates of the pixel in the input image
   fn color_output_image(config: Config) {
    let path = config.input;
    // Read input file
    let img = read_image(&path);
    // Create a black image of the same dimensions as the input image and return it, name it ${source_file}_output.png
    let output = format!("{}_output.{}", path.split(".").collect::<Vec<&str>>()[0], path.split(".").collect::<Vec<&str>>()[1]);
    // Create a black image if $output does not exist
    if !file_exists(&output) {
    create_black_image(&output, img.width(), img.height());
    } else {
    // Notify the user that the output image already exists
    println!("Output image already exists, it will be overwritten");
    }
    // Read the black image, which will be the output image
    let mut output_img = read_image(&output);
    // Generate N amount of random RGBA pixels, where N is the number of pixels in the input image
    let mut random_pixels = generate_random_pixels(img.width() * img.height());
    // For each pixel in the input image, find the closest pixel in the random pixels vector
    // and color the pixel in the output image with the closest pixel at the same coordinates
    let mut i = 0;
    let n = 2; // final image will be made of squares of n x n pixels
   
    // Use a hash map to store the closest pixels for each average color to avoid repeated calculations
    use std::collections::HashMap;
    let mut cache = HashMap::new();
    for x in 0..img.width()/n {
    for y in 0..img.height()/n {
    // Get the N pixels in a for loop
    let mut p = Vec::new();
    for k in 0..n {
    for l in 0..n {
    let pixel = img.get_pixel(x * n + k, y * n + l);
    p.push((pixel.0[0], pixel.0[1], pixel.0[2], pixel.0[3]));
    }
    }
    // Get the closest pixels
    let avg = calculate_average_color(&p);
    // Check if the cache contains the closest pixel for this average color
    let closest_pixel = match cache.get(&avg) {
    Some(pixel) => *pixel, // Use the cached value if it exists
    None => { // Otherwise, find the closest pixel and store it in the cache
    let pixel = find_closest_pixel(avg, &random_pixels);
    cache.insert(avg, pixel);
    pixel
    }
    };
    // Delete the closest pixel from the random pixels vector
    random_pixels.retain(|&x| x != closest_pixel);
    // Color the output image pixel with the closest pixel
    for k in 0..n {
    for l in 0..n {
    output_img.put_pixel(x * n + k, y * n + l, image::Rgba([closest_pixel.0, closest_pixel.1, closest_pixel.2, closest_pixel.3]));
    }
    }
    // Print progress as N^2
    i += n * n;
    if i % 1000 == 0 {
    println!("Progress: {} / {}", i, img.width() * img.height());
    }
    }
    }
    
    // Save the output image
    write_image(&output, &output_img);
    
    println!("Done");
    
}

// Define a struct to store the input file path, the output file path and the number of cores
struct Config {
    input: String,
    output: String,
    cores: usize,
}

// A function that handles command line arguments and returns a Config struct
fn parse_args() -> Option<Config> {
    // Get command line arguments using env module
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Please provide an input file path or use --help flag for more information");
        return None;
    }

    if args.len() > 7 {
        println!("Too many arguments provided");
        return None;
    }

    // Use variables to store the input file path, the output file path and the number of cores to use
    let mut input = String::new();
    let mut output = String::new();
    let mut cores = 1; // Default value is 1

    // Use a loop to parse the arguments
    for i in 1..args.len() {
        match args[i].as_str() {
            "--help" => {
                println!("This program takes an input image and creates an output image with randomly generated colors.\nUsage:\n\tcargo run --release --file <input_file_path> --output <output_file_path> --cpu <number_of_cores>\nExample:\n\tcargo run --release --file images/test.png --output images/output.png --cpu 4");
                return None;
            }
            "--file" => {
                // Check if there is another argument after --file
                if i + 1 < args.len() {
                    // Assign the next argument to the input variable
                    input = args[i + 1].clone();
                } else {
                    // If not, print an error message and return
                    println!("Please provide an input file path after --file flag");
                    return None;
                }
            }
            "--output" => {
                // Check if there is another argument after --output
                if i + 1 < args.len() {
                    // Assign the next argument to the output variable
                    output = args[i + 1].clone();
                } else {
                    // If not, print an error message and return
                    println!("Please provide an output file path after --output flag");
                    return None;
                }
            }
            "--cpu" => {
                // Check if there is another argument after --cpu
                if i + 1 < args.len() {
                    // Parse the next argument as a usize and assign it to the cores variable
                    cores = match args[i + 1].parse::<usize>() {
                        Ok(n) => n,
                        Err(_) => {
                            // If the argument is not a valid usize, print an error message and return
                            println!("Please provide a valid number of cores after --cpu flag");
                            return None;
                        }
                    };
                } else {
                    // If not, print an error message and return
                    println!("Please provide a number of cores after --cpu flag");
                    return None;
                }
            }
            _ => {
                // Ignore any other arguments
            }
        }
    }

    // Check if the input file path is empty
    if input.is_empty() {
        println!("Please provide an input file path using --file flag");
        return None;
    }

    // Check if the output file path is empty
    if output.is_empty() {
        println!("Please provide an output file path using --output flag");
        return None;
    }

    // Return a Config struct with the input file path, the output file path and the number of cores as fields
    Some(Config {input, output, cores})
}

// Main function that calls the parse_args function and runs the color_output_image function

fn main() {

    // Call the parse_args function and get a Config struct
    let config = match parse_args() {
        Some(c) => c,
        None => return,
    };

    // Check if the input file exists
    if !file_exists(&config.input) {
        println!("Input file does not exist");
        return;
    }

    // Check if the input file is an image
    if !is_image(&config.input) {
        println!("Input file is not an image");
        return;
    }

    color_output_image(config);

}
