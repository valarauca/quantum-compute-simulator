#Quantum Compute Simulator
Simulate Quantum Computing on your GPU using OpenCL


##Warning: I'm totally not a Quantum Physicist so this maybe _completely_ wrong!


#This crate is a work in progress. 

TODO:

- [X] Syntax Defined
- [ ] Command Line Interface Defined
- [ ] Backend passes
- [ ] OpenCL-C99 Code Gen
- [ ] OpenCL Backend Integration
- [ ] Actual Running
- [ ] Calculation are Correct

#License

This program is licensed under Apache-2.0

#Synatx:

This is a rough syntax guide.

```
// Comments start with //, they last an entire line.
// There is no /* comments

//Declare a qubit
//There is no space between the \"+i\"
//Floating point parsing is a bit funky at the minute
//so you always have to include a decimal
let qubit = 1.0 +i 0.0 |0> 0.0 +i -1.0 |1>;

//call a gate
had(qubit);

//display the qubit's data
display(qubit);
```

There is not anything else.  See the wiki for all supported gates. 
