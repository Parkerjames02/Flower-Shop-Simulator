use std::io;
// Adds color and rect (rectangle) properties from the sdl2 library
use sdl2::{pixels::Color, rect::Rect};
// Adds the duration property from the std (standard) library for time
use std::time::Duration;
// Tells the program to use the rand (random) property from the rand crate/dependeny.
extern crate rand;
// Sets the rand property to generate a random value.
use rand::Rng;

// This creates a function that contains the welcome message
fn welcome_message() {
    // This line displays the title of the program to the user
    println!("\n\n\t\tWelcome to the Flower Shop Simulator!");
    // This line displays a bugger underneath the welcome message.
    println!("\t\t-------------------------------------");
}

// This creates a function that contains the program description
fn description_message() {
    // This line displays the description of the program to the user
    println!("\nThis simulator allows you to purchase a beautiful flower from our shop.");
}

// This creates a function that allows the user to view the flower from the flower shop.
fn view_flower() {
    // This creates a mutable (changeable) string variable called view_choice.
    let mut view_choice = String::new();
    // This displays a message asking the user if they would like to take a look at the flower.
    println!("\nWould you like to look at a flower? (Yes or No)");
    // This allows the user to input their answer to the question that was asked.
    io::stdin()
        // This part specifically is for reading the line and assigning the string that was entered by the user to the string variable called view_choice.
        .read_line(&mut view_choice)
        // This part specifically is for exception handling.
        .expect("Failed to read customer's purchase confirmation");
    // This sets a new bool (boolean) variable called view_confirmation equal to the trim and lowercase version of the answer that was entered by the user.
    let view_confirmation: bool = match view_choice.trim().to_lowercase().as_ref() {
        // If the user enters "Yes" or "yes" then the view_confirmation variable will be set to true.
        "yes" => true,
        // If the user enters "No" or "no" then the view_confirmation variable will be set to false.
        "no" => false,
        // This is a panic message that is displayed to the user when they don't enter one of the two valid inputs being "Yes" or "No".
        _ => panic!("Please type either Yes or No."),
    };
    // Yes -----------------------------------------------------------------------------------------------------------------------------------------------
    // This if statement only occurs when the user enters "Yes" for the first question and the view_confirmation variable is set to true.
    if view_confirmation == true {
        // This displays a message to the user saying that the current flower that they are looking at is a arnica flower.
        println!("\nTake a look at this flower. Its an arnica flower:\n");
        // This initializes the sdl library. This must be called before using any other sdl function.
        let sdl_context = sdl2::init().unwrap();
        // This creates a video_subsystem from the sdl library.
        let video_subsytem = sdl_context.video().unwrap();
        // This generates the window that is used to house and canvas.
        let window = video_subsytem
            // This is where the title, width, and height are specified for the window
            .window("Arnica Flower", 930, 930)
            .position_centered()
            .build()
            .unwrap();
        // This sticks the canvas in the window that was created.
        let mut canvas = window.into_canvas().build().unwrap();
        // This is where the width of the canvas is specified
        let width: u32 = 910;
        // This is where the height of the canvas is specified
        let height: u32 = 910;
        // This creates the canvas. The position, height, and weight of the canvas is set here.
        let white_canvas = Rect::new(10, 10, width, height);
        // This sets the color of the canvas to white.
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        // This draws the canvas and fills it all in.
        canvas.fill_rect(white_canvas).unwrap();

        // This line sets the middle integer varaible to 280.
        let middle: u32 = 280;
        // This creates a square in the middle of the canvas. The middle variable is used here to set the width and height.
        let middle = Rect::new(320, 290, middle, middle);

        // This line sets the color of the middle square.
        canvas.set_draw_color(Color::RGB(254, 190, 101));
        // This draws the middle square and fills it all in.
        canvas.fill_rect(middle).unwrap();

        // This line sets the dot integer varaible to 45.
        let dot: u32 = 45;
        // This creates a dot in the middle of the canvas. The dot variable is used here to set the width and height.
        let middle_dot_1 = Rect::new(370, 380, dot, dot);

        // This line sets the color of the dot.
        canvas.set_draw_color(Color::RGB(250, 254, 92));
        // This draws the dot and fills it all in.
        canvas.fill_rect(middle_dot_1).unwrap();

        // This creates a dot in the middle of the canvas. The dot variable is used here to set the width and height.
        let middle_dot_2 = Rect::new(415, 335, dot, dot);

        // This line sets the color of the dot.
        canvas.set_draw_color(Color::RGB(250, 254, 92));
        // This draws the dot and fills it all in.
        canvas.fill_rect(middle_dot_2).unwrap();

        // This creates a dot in the middle of the canvas. The dot variable is used here to set the width and height.
        let middle_dot_3 = Rect::new(460, 380, dot, dot);

        // This line sets the color of the dot.
        canvas.set_draw_color(Color::RGB(250, 254, 92));
        // This draws the dot and fills it all in.
        canvas.fill_rect(middle_dot_3).unwrap();

        // This creates a dot in the middle of the canvas. The dot variable is used here to set the width and height.
        let middle_dot_4 = Rect::new(505, 335, dot, dot);

        // This line sets the color of the dot.
        canvas.set_draw_color(Color::RGB(250, 254, 92));
        // This draws the dot and fills it all in.
        canvas.fill_rect(middle_dot_4).unwrap();

        // This creates a dot in the middle of the canvas. The dot variable is used here to set the width and height.
        let middle_dot_5 = Rect::new(505, 425, dot, dot);

        // This line sets the color of the dot.
        canvas.set_draw_color(Color::RGB(250, 254, 92));
        // This draws the dot and fills it all in.
        canvas.fill_rect(middle_dot_5).unwrap();

        // This creates a dot in the middle of the canvas. The dot variable is used here to set the width and height.
        let middle_dot_6 = Rect::new(415, 481, dot, dot);

        // This line sets the color of the dot.
        canvas.set_draw_color(Color::RGB(250, 254, 92));
        // This draws the dot and fills it all in.
        canvas.fill_rect(middle_dot_6).unwrap();

        // This creates a rectangle in the middle of the canvas.
        let middle_top = Rect::new(415, 245, 90, 45);

        // This line sets the color of the rectangle.
        canvas.set_draw_color(Color::RGB(243, 157, 51));
        // This draws the rectangle and fills it all in.
        canvas.fill_rect(middle_top).unwrap();

        // This creates a rectangle in the middle of the canvas.
        let middle_top_left = Rect::new(320, 290, 95, 44);

        // This line sets the color of the rectangle.
        canvas.set_draw_color(Color::RGB(243, 157, 51));
        // This draws the rectangle and fills it all in.
        canvas.fill_rect(middle_top_left).unwrap();

        // This creates a rectangle in the middle of the canvas.
        let middle_top_right = Rect::new(505, 290, 95, 44);

        // This line sets the color of the rectangle.
        canvas.set_draw_color(Color::RGB(243, 157, 51));
        // This draws the rectangle and fills it all in.
        canvas.fill_rect(middle_top_right).unwrap();

        // This creates a rectangle in the middle of the canvas.
        let middle_top_left_2 = Rect::new(320, 334, 50, 45);

        // This line sets the color of the rectangle.
        canvas.set_draw_color(Color::RGB(243, 157, 51));
        // This draws the rectangle and fills it all in.
        canvas.fill_rect(middle_top_left_2).unwrap();

        // This creates a rectangle in the middle of the canvas.
        let middle_top_right_2 = Rect::new(550, 334, 50, 45);

        // This line sets the color of the rectangle.
        canvas.set_draw_color(Color::RGB(243, 157, 51));
        // This draws the rectangle and fills it all in.
        canvas.fill_rect(middle_top_right_2).unwrap();

        // This creates a rectangle in the middle of the canvas.
        let middle_left = Rect::new(275, 379, 45, 91);

        // This line sets the color of the rectangle.
        canvas.set_draw_color(Color::RGB(243, 157, 51));
        // This draws the rectangle and fills it all in.
        canvas.fill_rect(middle_left).unwrap();

        // This creates a rectangle in the middle of the canvas.
        let middle_bottom_left = Rect::new(320, 526, 95, 44);

        // This line sets the color of the rectangle.
        canvas.set_draw_color(Color::RGB(243, 157, 51));
        // This draws the rectangle and fills it all in.
        canvas.fill_rect(middle_bottom_left).unwrap();

        // This creates a rectangle in the middle of the canvas.
        let middle_bottom_left_2 = Rect::new(320, 470, 50, 60);

        // This line sets the color of the rectangle.
        canvas.set_draw_color(Color::RGB(243, 157, 51));
        // This draws the rectangle and fills it all in.
        canvas.fill_rect(middle_bottom_left_2).unwrap();

        // This creates a rectangle in the middle of the canvas.
        let middle_bottom = Rect::new(415, 570, 90, 45);

        // This line sets the color of the rectangle.
        canvas.set_draw_color(Color::RGB(243, 157, 51));
        // This draws the rectangle and fills it all in.
        canvas.fill_rect(middle_bottom).unwrap();

        // This creates a rectangle in the middle of the canvas.
        let middle_bottom_right = Rect::new(505, 525, 95, 44);

        // This line sets the color of the rectangle.
        canvas.set_draw_color(Color::RGB(243, 157, 51));
        // This draws the rectangle and fills it all in.
        canvas.fill_rect(middle_bottom_right).unwrap();

        // This creates a rectangle in the middle of the canvas.
        let middle_bottom_right_2 = Rect::new(550, 470, 50, 60);

        // This line sets the color of the rectangle.
        canvas.set_draw_color(Color::RGB(243, 157, 51));
        // This draws the rectangle and fills it all in.
        canvas.fill_rect(middle_bottom_right_2).unwrap();

        // This creates a rectangle in the middle of the canvas.
        let middle_right = Rect::new(600, 379, 45, 91);

        // This line sets the color of the rectangle.
        canvas.set_draw_color(Color::RGB(243, 157, 51));
        // This draws the rectangle and fills it all in.
        canvas.fill_rect(middle_right).unwrap();

        // This creates a rectangle in the middle of the canvas.
        let middle_top_petal = Rect::new(415, 97, 90, 150);

        // This line sets the color of the rectangle.
        canvas.set_draw_color(Color::RGB(250, 241, 199));
        // This draws the rectangle and fills it all in.
        canvas.fill_rect(middle_top_petal).unwrap();

        // This creates a rectangle in the middle of the canvas.
        let middle_top_petal_top = Rect::new(415, 52, 90, 45);

        // This line sets the color of the rectangle.
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // This draws the rectangle and fills it all in.
        canvas.fill_rect(middle_top_petal_top).unwrap();

        // This creates a rectangle in the middle of the canvas.
        let middle_top_petal_left = Rect::new(370, 97, 45, 110);

        // This line sets the color of the rectangle.
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // This draws the rectangle and fills it all in.
        canvas.fill_rect(middle_top_petal_left).unwrap();

        // This creates a rectangle in the middle of the canvas.
        let middle_top_petal_right = Rect::new(505, 97, 45, 110);

        // This line sets the color of the rectangle.
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // This draws the rectangle and fills it all in.
        canvas.fill_rect(middle_top_petal_right).unwrap();

        // This creates a rectangle in the middle of the canvas.
        let middle_left_petal = Rect::new(125, 379, 150, 91);

        // This line sets the color of the rectangle.
        canvas.set_draw_color(Color::RGB(250, 241, 199));
        // This draws the rectangle and fills it all in.
        canvas.fill_rect(middle_left_petal).unwrap();

        // This creates a rectangle in the middle of the canvas.
        let middle_left_petal_top = Rect::new(80, 379, 45, 91);

        // This line sets the color of the rectangle.
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // This draws the rectangle and fills it all in.
        canvas.fill_rect(middle_left_petal_top).unwrap();

        // This creates a rectangle in the middle of the canvas.
        let middle_left_petal_right = Rect::new(125, 334, 110, 45);

        // This line sets the color of the rectangle.
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // This draws the rectangle and fills it all in.
        canvas.fill_rect(middle_left_petal_right).unwrap();

        // This creates a rectangle in the middle of the canvas.
        let middle_left_petal_left = Rect::new(125, 470, 110, 45);

        // This line sets the color of the rectangle.
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // This draws the rectangle and fills it all in.
        canvas.fill_rect(middle_left_petal_left).unwrap();

        // This creates a rectangle in the middle of the canvas.
        let middle_bottom_petal = Rect::new(415, 615, 90, 150);

        // This line sets the color of the rectangle.
        canvas.set_draw_color(Color::RGB(250, 241, 199));
        // This draws the rectangle and fills it all in.
        canvas.fill_rect(middle_bottom_petal).unwrap();

        // This creates a rectangle in the middle of the canvas.
        let middle_bottom_petal_top = Rect::new(415, 765, 90, 45);

        // This line sets the color of the rectangle.
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // This draws the rectangle and fills it all in.
        canvas.fill_rect(middle_bottom_petal_top).unwrap();

        // This creates a rectangle in the middle of the canvas.
        let middle_bottom_petal_left = Rect::new(370, 655, 45, 110);

        // This line sets the color of the rectangle.
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // This draws the rectangle and fills it all in.
        canvas.fill_rect(middle_bottom_petal_left).unwrap();

        // This creates a rectangle in the middle of the canvas.
        let middle_bottom_petal_right = Rect::new(505, 655, 45, 110);

        // This line sets the color of the rectangle.
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // This draws the rectangle and fills it all in.
        canvas.fill_rect(middle_bottom_petal_right).unwrap();

        // This creates a rectangle in the middle of the canvas.
        let middle_right_petal = Rect::new(647, 379, 150, 91);

        // This line sets the color of the rectangle.
        canvas.set_draw_color(Color::RGB(250, 241, 199));
        // This draws the rectangle and fills it all in.
        canvas.fill_rect(middle_right_petal).unwrap();

        // This creates a rectangle in the middle of the canvas.
        let middle_right_petal_top = Rect::new(797, 379, 45, 91);

        // This line sets the color of the rectangle.
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // This draws the rectangle and fills it all in.
        canvas.fill_rect(middle_right_petal_top).unwrap();

        // This creates a rectangle in the middle of the canvas.
        let middle_right_petal_right = Rect::new(687, 334, 110, 45);

        // This line sets the color of the rectangle.
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // This draws the rectangle and fills it all in.
        canvas.fill_rect(middle_right_petal_right).unwrap();

        // This creates a rectangle in the middle of the canvas.
        let middle_right_petal_left = Rect::new(687, 470, 110, 45);

        // This line sets the color of the rectangle.
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // This draws the rectangle and fills it all in.
        canvas.fill_rect(middle_right_petal_left).unwrap();

        // This creates a dot in the middle of the canvas. The dot variable is used here to set the width and height.
        let top_left_diagonal_dot = Rect::new(325, 207, dot, dot);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // Fill up the square    with color
        canvas.fill_rect(top_left_diagonal_dot).unwrap();

        // This creates a dot in the middle of the canvas. The dot variable is used here to set the width and height.
        let top_left_diagonal_dot_2 = Rect::new(280, 162, dot, dot);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // Fill up the square    with color
        canvas.fill_rect(top_left_diagonal_dot_2).unwrap();

        // Create a square that contains the position and size of our
        let top_left_diagonal_line = Rect::new(166, 117, 114, 45);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // Fill up the square    with color
        canvas.fill_rect(top_left_diagonal_line).unwrap();

        // Create a square that contains the position and size of our
        let top_left_diagonal_line_2 = Rect::new(146, 117, 45, 127);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // Fill up the square    with color
        canvas.fill_rect(top_left_diagonal_line_2).unwrap();

        // This creates a dot in the middle of the canvas. The dot variable is used here to set the width and height.
        let top_left_diagonal_dot_3 = Rect::new(191, 244, dot, dot);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // Fill up the square    with color
        canvas.fill_rect(top_left_diagonal_dot_3).unwrap();

        // This creates a dot in the middle of the canvas. The dot variable is used here to set the width and height.
        let top_left_diagonal_dot_4 = Rect::new(236, 289, dot, dot);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // Fill up the square    with color
        canvas.fill_rect(top_left_diagonal_dot_4).unwrap();

        // This creates a dot in the middle of the canvas. The dot variable is used here to set the width and height.
        let bottom_left_diagonal_dot = Rect::new(325, 610, dot, dot);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // Fill up the square    with color
        canvas.fill_rect(bottom_left_diagonal_dot).unwrap();

        // This creates a dot in the middle of the canvas. The dot variable is used here to set the width and height.
        let bottom_left_diagonal_dot_2 = Rect::new(280, 655, dot, dot);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // Fill up the square    with color
        canvas.fill_rect(bottom_left_diagonal_dot_2).unwrap();

        // Create a square that contains the position and size of our
        let bottom_left_diagonal_line = Rect::new(166, 700, 114, 45);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // Fill up the square    with color
        canvas.fill_rect(bottom_left_diagonal_line).unwrap();

        // Create a square that contains the position and size of our
        let bottom_left_diagonal_line_2 = Rect::new(146, 618, 45, 127);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // Fill up the square    with color
        canvas.fill_rect(bottom_left_diagonal_line_2).unwrap();

        // This creates a dot in the middle of the canvas. The dot variable is used here to set the width and height.
        let bottom_left_diagonal_dot_3 = Rect::new(191, 573, dot, dot);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // Fill up the square    with color
        canvas.fill_rect(bottom_left_diagonal_dot_3).unwrap();

        // This creates a dot in the middle of the canvas. The dot variable is used here to set the width.
        let bottom_left_diagonal_dot_4 = Rect::new(236, 515, dot, 58);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // Fill up the square    with color
        canvas.fill_rect(bottom_left_diagonal_dot_4).unwrap();

        // This creates a dot in the middle of the canvas. The dot variable is used here to set the width and height.
        let bottom_right_diagonal_dot = Rect::new(550, 610, dot, dot);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // Fill up the square    with color
        canvas.fill_rect(bottom_right_diagonal_dot).unwrap();

        // This creates a dot in the middle of the canvas. The dot variable is used here to set the width and height.
        let bottom_right_diagonal_dot_2 = Rect::new(595, 655, dot, dot);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // Fill up the square    with color
        canvas.fill_rect(bottom_right_diagonal_dot_2).unwrap();

        // Create a square that contains the position and size of our
        let bottom_right_diagonal_line = Rect::new(640, 700, 132, 45);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // Fill up the square    with color
        canvas.fill_rect(bottom_right_diagonal_line).unwrap();

        // Create a square that contains the position and size of our
        let bottom_right_diagonal_line_2 = Rect::new(732, 618, 45, 127);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // Fill up the square    with color
        canvas.fill_rect(bottom_right_diagonal_line_2).unwrap();

        // This creates a dot in the middle of the canvas. The dot variable is used here to set the width and height.
        let bottom_right_diagonal_dot_3 = Rect::new(687, 573, dot, dot);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // Fill up the square    with color
        canvas.fill_rect(bottom_right_diagonal_dot_3).unwrap();

        // This creates a dot in the middle of the canvas. The dot variable is used here to set the width
        let bottom_right_diagonal_dot_4 = Rect::new(642, 515, dot, 58);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // Fill up the square    with color
        canvas.fill_rect(bottom_right_diagonal_dot_4).unwrap();

        // This creates a dot in the middle of the canvas. The dot variable is used here to set the width and height.
        let top_right_diagonal_dot = Rect::new(550, 207, dot, dot);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // Fill up the square    with color
        canvas.fill_rect(top_right_diagonal_dot).unwrap();

        // This creates a dot in the middle of the canvas. The dot variable is used here to set the width and height.
        let top_right_diagonal_dot_2 = Rect::new(595, 162, dot, dot);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // Fill up the square    with color
        canvas.fill_rect(top_right_diagonal_dot_2).unwrap();

        // Create a square that contains the position and size of our
        let top_right_diagonal_line = Rect::new(640, 117, 134, 45);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // Fill up the square    with color
        canvas.fill_rect(top_right_diagonal_line).unwrap();

        // Create a square that contains the position and size of our
        let top_right_diagonal_line_2 = Rect::new(732, 117, 45, 127);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // Fill up the square    with color
        canvas.fill_rect(top_right_diagonal_line_2).unwrap();

        // This creates a dot in the middle of the canvas. The dot variable is used here to set the width and height.
        let top_right_diagonal_dot_3 = Rect::new(687, 244, dot, dot);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // Fill up the square    with color
        canvas.fill_rect(top_right_diagonal_dot_3).unwrap();

        // This creates a dot in the middle of the canvas. The dot variable is used here to set the width and height.
        let top_right_diagonal_dot_4 = Rect::new(642, 289, dot, dot);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(245, 215, 77));
        // Fill up the square    with color
        canvas.fill_rect(top_right_diagonal_dot_4).unwrap();

        // This creates a dot in the middle of the canvas. The dot variable is used here to set the width.
        let top_left_diagonal_dot_inside = Rect::new(325, 252, dot, 38);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(250, 241, 199));
        // Fill up the square    with color
        canvas.fill_rect(top_left_diagonal_dot_inside).unwrap();

        // Create a square that contains the position and size of our
        let top_left_diagonal_dot_inside_2 = Rect::new(283, 289, 36, dot);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(250, 241, 199));
        // Fill up the square    with color
        canvas.fill_rect(top_left_diagonal_dot_inside_2).unwrap();

        // Create a square that contains the position and size of our
        let top_left_diagonal_dot_inside_3 = Rect::new(236, 207, 90, 82);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(250, 241, 199));
        // Fill up the square    with color
        canvas.fill_rect(top_left_diagonal_dot_inside_3).unwrap();

        // Create a square that contains the position and size of our
        let top_left_diagonal_dot_inside_4 = Rect::new(191, 162, 90, 82);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(250, 241, 199));
        // Fill up the square    with color
        canvas.fill_rect(top_left_diagonal_dot_inside_4).unwrap();

        // Create a square that contains the position and size of our
        let bottom_left_diagonal_dot_inside = Rect::new(319, 570, 51, 40);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(250, 241, 199));
        // Fill up the square    with color
        canvas.fill_rect(bottom_left_diagonal_dot_inside).unwrap();

        // Create a square that contains the position and size of our
        let bottom_left_diagonal_dot_inside_2 = Rect::new(283, 515, 36, 58);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(250, 241, 199));
        // Fill up the square    with color
        canvas.fill_rect(bottom_left_diagonal_dot_inside_2).unwrap();

        // Create a square that contains the position and size of our
        let bottom_left_diagonal_dot_inside_3 = Rect::new(236, 573, 90, 82);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(250, 241, 199));
        // Fill up the square    with color
        canvas.fill_rect(bottom_left_diagonal_dot_inside_3).unwrap();

        // Create a square that contains the position and size of our
        let bottom_left_diagonal_dot_inside_4 = Rect::new(191, 618, 90, 82);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(250, 241, 199));
        // Fill up the square    with color
        canvas.fill_rect(bottom_left_diagonal_dot_inside_4).unwrap();

        // Create a square that contains the position and size of our
        let bottom_right_diagonal_dot_inside = Rect::new(550, 570, 51, 40);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(250, 241, 199));
        // Fill up the square    with color
        canvas.fill_rect(bottom_right_diagonal_dot_inside).unwrap();

        // Create a square that contains the position and size of our
        let bottom_right_diagonal_dot_inside_2 = Rect::new(600, 515, 44, 58);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(250, 241, 199));
        // Fill up the square    with color
        canvas
            .fill_rect(bottom_right_diagonal_dot_inside_2)
            .unwrap();

        // Create a square that contains the position and size of our
        let bottom_right_diagonal_dot_inside_3 = Rect::new(595, 573, 90, 82);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(250, 241, 199));
        // Fill up the square    with color
        canvas
            .fill_rect(bottom_right_diagonal_dot_inside_3)
            .unwrap();

        // Create a square that contains the position and size of our
        let bottom_right_diagonal_dot_inside_4 = Rect::new(642, 618, 90, 82);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(250, 241, 199));
        // Fill up the square    with color
        canvas
            .fill_rect(bottom_right_diagonal_dot_inside_4)
            .unwrap();

        // Create a square that contains the position and size of our
        let top_right_diagonal_dot_inside = Rect::new(550, 252, dot, 38);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(250, 241, 199));
        // Fill up the square    with color
        canvas.fill_rect(top_right_diagonal_dot_inside).unwrap();

        // Create a square that contains the position and size of our
        let top_right_diagonal_dot_inside_2 = Rect::new(600, 289, 44, dot);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(250, 241, 199));
        // Fill up the square    with color
        canvas.fill_rect(top_right_diagonal_dot_inside_2).unwrap();

        // Create a square that contains the position and size of our
        let top_right_diagonal_dot_inside_3 = Rect::new(595, 207, 90, 82);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(250, 241, 199));
        // Fill up the square    with color
        canvas.fill_rect(top_right_diagonal_dot_inside_3).unwrap();

        // Create a square that contains the position and size of our
        let top_right_diagonal_dot_inside_4 = Rect::new(642, 162, 90, 82);

        // Set the draw color for the
        canvas.set_draw_color(Color::RGB(250, 241, 199));
        // Fill up the square    with color
        canvas.fill_rect(top_right_diagonal_dot_inside_4).unwrap();

        // Draw the canvas on the screen
        canvas.present();

        // Loop until the user closes the window by pressing escape
        let mut event_pump = sdl_context.event_pump().unwrap();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. }
                    | sdl2::event::Event::KeyDown {
                        // This line specifically allows the user to close the window with the escape key.
                        keycode: Some(sdl2::keyboard::Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
            std::thread::sleep(Duration::from_millis(16));
        }
    // No -----------------------------------------------------------------------------------------------------------------------------------------------
    // This else if statement only occurs when the user enters "No" for the first question and the view_confirmation variable is set to false.
    } else if view_confirmation == false {
        // This displays an apology to the user asking them to visit the shop again.
        println!("\nI'm sorry. Please visit again if you ever need a flower.");
        // This displays the goodybe message for the sim to the user.
        goodbye_message();
    }
}

// This is the function that confirms the purchase of the flower for the user. It is only called if the user answers yes to the first question.
fn purchase_confirmation() {
    // This creates a mutable (changeable) string variable called purchase_choice.
    let mut purchase_choice = String::new();
    // This displays a message asking the user if they would like to purchase the flower.
    println!("Would you like to purchase this flower? (Yes or No)");
    // This allows the user to input their answer to the question that was asked.
    io::stdin()
        // This part specifically is for reading the line and assigning the string that was entered by the user to the string variable called purchase_choice.
        .read_line(&mut purchase_choice)
        // This part specifically is for exception handling.
        .expect("Failed to read customer's purchase confirmation");
    // This sets a new bool (boolean) variable called purchase_confirmation equal to the trim and lowercase version of the answer that was entered by the user.
    let purchase_confirmation: bool = match purchase_choice.trim().to_lowercase().as_ref() {
        // If the user enters "Yes" or "yes" then the purchase_confirmation variable will be set to true.
        "yes" => true,
        // If the user enters "No" or "no" then the purchase_confirmation variable will be set to false.
        "no" => false,
        // This is a panic message that is displayed to the user when they don't enter one of the two valid inputs being "Yes" or "No".
        _ => panic!("Please type either Yes or No."),
    };
    // This if statement only occurs when the user enters "Yes" for the second question and the view_confirmation variable is set to true.
    if purchase_confirmation == true {
        // This displays the title of the purchase summary to the user.
        println!("\nPurchase Summary");
        // This displays a buffer right underneath the title of the purchase summary to the user.
        println!("----------------\n");
        // This displays a message to the user showing that they have purchased one arnica flower.
        println!("1x Arnica Flower\n");
        // This displays a buffer right underneath the purchase summary to the user.
        println!("----------------");
        // Sets the variable equal to the random number generator with a minimum of 5 and maximum of 16.
        let random_number = rand::thread_rng().gen_range(5..=16);
        // Prints the price of the flower with the random number that was generated
        println!("Total: ${}", random_number);
        // This displays a thank you message to the user after the purchase summary.
        println!("\nThank you for visiting our shop and please come again!");
    } 
    // This if statement only occurs when the user enters "No" for the second question and the purchase_confirmation variable is set to false.
    else if purchase_confirmation == false {
        // This displays an apology to the user and tells them to visit again if they are looking for a flower.
        println!("\nI'm sorry. Please visit again if you ever need a flower.");
        // This displays the goodbye message for the flower shop simulator.
        goodbye_message();
    }
}

// This creates a function that contains the goodbye message
fn goodbye_message() {
    // This displays a buffer for the goodbye message to the user.
    println!("\n---------------------------------------------------------------------------");
    // This displays the goodbye message for the simulator after the program has completed all of its processes.
    println!("Thank you for using my Flower Shop Simulator! We hope that you visit again.\n");
    // This line exits and stops the program once the goodbye message has been displayed.
    std::process::exit(0);
}

// This is the main function and the spot that the program points to upon startup.
fn main() {
    // This line calls the welcome_message() function.
    welcome_message();
    // This line calls the description_message() function.
    description_message();
    // This line calls the view_flower() function.
    view_flower();
    // This line calls the purchase_confirmation() function.
    purchase_confirmation();
    // This line calls the goodbye_message() function.
    goodbye_message();
}
