# My first rust code
 Simple rust grep

It's kind of a basic grep. It searches all the lines of a file for the provided string path.<br/>
This CLI takes as argument a string, a file path and optionally a output path.<br/>
If no output path is provided, it will print the results to the console.<br/>
<br/>
Using cargo just go with:

```
cargo run word file.txt 
```

Or:
```
cargo run word file.txt outputfile.txt
```

# NOTE:
This is my first time trying rust and I found a really nice [tutorial](https://rust-cli.github.io/book/index.html) on how to create a CLI in rust.
I really enjoyed that tutorial because it doesn't just take you by the hand and gives you all the code you need.<br/>
It encourages you to add features, think about how things work and even provides with small challenges so you can learn by yourself.<br/>
I skipped the testing and packagin topics, and didn't do any of the in depth ones because i didn't saw a point to it.<br/>
My goal here was to do a simple CLI that searches a given string in a provided file and output the results into a file or the terminal<br/>
I probably won't elaborate on this project, but who knows. For now, there are other projects that i intend to start, so i can learn various aspects of the language.<br/>
(That was truly no reason to create another function called "find_content" and call it in main, i did it just because)