# Todo list cli

This is small project in rust as my fisrt project to get some experience in it. 

This project allows you to create (add), delete and mork done and undo it interatively in cli.

## Dependencies
```
crossterm = "0.27"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

# Keys
`k`, `j`	Move cursor up and down
`Shift+K`, `Shift+J`	Drag the current item up and down
`g, G`	Jump to the start, end of the current item list
`r`	Rename the current item
`i`	Insert a new item
`d`	Delete the current list item
`q`	Quit
`TAB`	Switch between the TODO and DONE panels
`Enter`	Perform an action on the highlighted UI element
