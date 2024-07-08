# firstgrep
 Simple rust grep

This is my first time trying rust and I found a really nice [tutorial](https://rust-cli.github.io/book/index.html) on how to create a CLI in rust.
<br/>
It's kind of a basic grep and i'm still going through it on my free time.<br/>
This just go through a text file and check if the provided word is in that line;
<br/><br/>
Update:<br/>
Fixed the progressbar, kindof.<br/>
Also added the option to write the output to a file or wirite it directly to the console.<br/>
<br/><br/>
I'm sure that it's not optimal at all, but i think it's some progress.
<br/><br/>
Using cargo just go with:
```
cargo run word filepath 
```
Or:
```
cargo run word filepath outputpath.txt
```