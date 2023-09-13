fn main(){
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Not value provided");
        return;
    }
    let x_args: &String = &args[1];
    let x: i32 = x_args.parse().unwrap();
    println!("<style>");
    println!("table, td {{");
    println!("   border: 1px solid #000000;\n   border-collapse: collapse;\n}}");
    println!("</style>\n");
    println!("<table>");
    println!("   <tr>");
    println!("       <th>x</th>");
    println!("       <th>x^2</th>");
    println!("       <th>x^3</th>");
    println!("   </tr>");
    println!("   <tr>");
    println!("       <td>{}</td>", x);
    println!("       <td>{}</td>", x*x);
    println!("       <td>{}</td>", x*x*x);
    println!("   </tr>");
    println!("</table>")
}