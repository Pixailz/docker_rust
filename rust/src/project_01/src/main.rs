
fn do_math(a : i32, b : i32, mode : &str)
{
	let result : i32;

	match mode
	{
		"+" => result = a + b,
		"-" => result = a - b,
		"*" => result = a * b,
		"/" => result = a / b,
		"%" => result = a % b,
		&_  => todo!(),
	}
	println!("{a} {mode} {b} = {result}");
}

fn test_do_math()
{
	let a = 5;
	let b = 15;

	do_math(a, b, "+");
	do_math(a, b, "-");
	do_math(a, b, "*");
	do_math(a, b, "/");
	do_math(a, b, "%");
}

fn test_types()
{

	let u_x :	u32		= 32;
	let s_x:	i32		= -5;

	let x_128 :	i128	= 1_128;
	let x_64 :	i64		= 64;
	let x_32 :	i32		= 32;
	let x_16 :	i16		= 16;
	let x_8 :	i8		= 8;

	let f_64 :	f64		= 6.4;
	let f_32 :	f32		= 3.2;

	let boolean : bool	= true;

	let ch : char		= 'c';
	let msg : &str		= "Hello Worlds!";

	println!("u_x     {u_x}");
	println!("s_x     {s_x}");
	// println!("u_x + s_x {u_x + s_x}");

	println!("x_128   {x_128}");
	println!("x_64    {x_64}");
	println!("x_32    {x_32}");
	println!("x_16    {x_16}");
	println!("x_8     {x_8}");

	println!("f_64    {f_64}");
	println!("f_32    {f_32}");

	println!("boolean {boolean}");

	println!("ch      {ch}");
	println!("msg     {msg}");
}

fn test_operator()
{
	let x: i32 = 5;
	let mut y: i32 = 10;

	y += 1;
	y -= 1;
	y *= 1;
	y /= 1;
	y %= 1;

	println!("x == y -> {}", x == y);
	println!("x != y -> {}", x != y);
	println!("x <  y -> {}", x <  y);
	println!("x <= y -> {}", x <= y);
	println!("x >  y -> {}", x >  y);
	println!("x >= y -> {}", x >= y);

	println!("\"123\" >  \"1234\" -> {}", "123" >  "1234");
	println!("\"123\" <  \"1234\" -> {}", "123" <  "1234");
	println!("\"1235\" == \"1234\" -> {}", "1235" == "1234");
}

fn test_condition()
{
	let var_1 : &str	= "test";
	let mut age_1 : i32	= 10;
	let is_major : &str	= if age_1 >= 18 { "major" } else { "minor" };
	let mut n : i32		= 0;

	if var_1 >= "" {
		println!("{var_1} == ''");
	}
	else {
		println!("{var_1} != ''");
	}
	println!("{var_1} == '' -> {}", var_1 == "");

	println!("is_major : {is_major}");
	while age_1 < 18 {
		age_1 += 1;
		println!("age : {age_1}");
	}

	// while true {
	loop {
		println!("{} * {} = {}", n, n, n * n);
		n += 1;
		if n * n > 255 { break; }
	}

	for x in 1..=11 {
		println!("x = {x}");
	}
	for x in 1..11 {
		println!("x = {x}");
	}
}

fn debug_array(arr : &[&str])
{
	for i in 0..arr.len() {
		println!("arr[{i}] = {} ", arr[i]);
	}
	println!("array length : {}\n", arr.len());
}

fn test_array()
{
	let	arr_1 = ["This", "display", "hello", "world", "!"];
	let mut arr_2 = ["str"; 15];

	debug_array(&arr_1);
	debug_array(&arr_2);
	arr_2[3] = "Pix";
	debug_array(&arr_2);

	println!("{:#?}\n{:#?}", arr_1, arr_2);
}

fn debug_vector(vector: Vec<&str>)
{
	for i in 0..vector.len() {
		println!("vector[{i}] = {}", vector[i]);
	}
	println!("vector length : {}\n", vector.len());
}

fn test_vector()
{
	let mut vector = vec!["Hello", "World", "!"];

	debug_vector(vector.clone());
	vector.push("and");
	debug_vector(vector.clone());
	vector.insert(0, "to");
	debug_vector(vector.clone());
	vector.remove(0);
	vector.push("everybody");
	vector.push("else");
	debug_vector(vector.clone());
	vector.pop();

	println!("{:#?}", vector);
}

fn print_choice()
{
	println!("Available choices:");
	println!("- (1) do math");
	println!("- (2) types");
	println!("- (3) operator");
	println!("- (4) condition");
	println!("- (5) array");
	println!("- (6) vector");
	println!(" - (q) quit.");
}


fn main()
{
	let stdin = std::io::stdin();

	print_choice();
	for line in stdin.lines()
	{
		print!("\x1b[2J");
		match &*line.unwrap()
		{
			"1" => test_do_math(),
			"2" => test_types(),
			"3" => test_operator(),
			"4" => test_condition(),
			"5" => test_array(),
			"6" => test_vector(),
			"q" => break,
			&_  => println!("wrong choice"),
		}
		println!();
		print_choice();
	}
}

