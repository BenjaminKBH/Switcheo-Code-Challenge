use std::io;

fn main() {
    println!("Please input a number");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read input.");
    let input=input.trim().parse::<i32>().expect("invalid input");
    println!("Method 1:sum to {}==={}",input,sum_to_n_a(input));
    println!("Method 2:sum to {}==={}",input,sum_to_n_b(input));
    println!("Method 3:sum to {}==={}",input,sum_to_n_c(input));
	println!("Press enter to exit");
	pause();
	
}

fn sum_to_n_a(n: i32) -> i32 {
	let mut counter: i32=1;
	let mut total: i32=0;
	while counter<=n{
	    total+=counter;
	    counter+=1;
	}
	total
	//Flexible counter can be changed to plus more if needed
}

fn sum_to_n_b(n: i32) -> i32 {
	let mut total: i32=0;
	for counter in 1..n+1{
	    total+=counter;
	}
	total
	//shortest in terms of code lines but counter is fixed in the top so cant change in middle of code
}

fn sum_to_n_c(n: i32) -> i32 {
	let mut counter: i32=1;
	let mut total: i32=0;
	let result=loop{
	    if counter>n{
	        break total;
	    }
	    total+=counter;
	    counter+=1;
	};
	result
	//uses break to get out of loop when condition met
}

fn pause() {
	let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
	//code just to make it so that exe file wont exit immediately after showing results
}