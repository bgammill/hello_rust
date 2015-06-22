struct Request {
    number_of_times: i32,
    name: &'static str
}

fn main() {
    let request = Request { number_of_times: 5, name: "bgammill" };
    print_name_this_many_times(request);
}

fn print_name_this_many_times(request: Request) {
    for x in 0..request.number_of_times {
        println!("{}", request.name);
    }
}
