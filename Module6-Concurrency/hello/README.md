Commit 1 Reflection Notes (Single Threaded Web Server)

So basically, the handle_connection function is used to deal with incoming connections from clients. It takes a TcpStream, which is like the connection between the server and the client where data is being sent.
Inside the function, we wrap the stream using BufReader so we can read the data more easily, especially line by line. Then we use .lines() to go through each line of the HTTP request sent by the client.
For each line, .map(|result| result.unwrap()) is used to get the actual string from the result. After that, .take_while(|line| !line.is_empty()) keeps reading lines until it hits an empty line. This is important because in HTTP, an empty line means the end of the request headers.
At the end, all the lines are stored in a vector and printed using println! with debug formatting, so we can clearly see what the request looks like in the terminal.
Overall, this function shows how a simple web server can read and understand incoming requests using Rust. It helped me understand things like how streams work, how buffering helps, and how HTTP requests are structured.