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
- [ ] Running on Windows
- [ ] Running on OSX
- [ ] Running on Linux
- [ ] Calculation are Correct

#License

This program is licensed under Apache-2.0

#Synatx:

####Comments:

Comments start with //, they last an entire line. There is no `/* */`comments.
One should know the space after `;` before the line break can also be used for comments.

####Number Point Parsing

Just always include a decimal point and you'll have no errors. I suck at nom (the parser library I'm using).

####Identifiers

Varaibles must only contain alphabetic letters `[a-zA-z]`. No numbers, spaces, or special things.

####Declaring Qubits

```
let qubit = 1.0 +i 0.0 |0> 0.0 +i -1.0 |1>;
```

There is no space between the `+i`, `|0>`, and `|1>`. The `|0>` must procede `|1>`. 

####Gate List


```
had( qubit ); 		Hadamard
not( qubit ); 		Not
paulix( qubit );	Pauli-X
pauliy( qubit );	Pauli-Y
pauliz( qubit );	Pauli-Z
sqrtnot( qubit );	Sqrt-Not
display(qubit);		Print a Qubit Value to your terminal
```

The second bit should be considered the _top_ to directly compare them to the standard fenymann way of writing things.

```
swap( qu, bit);		Swap
sqrt_swap( qu, bit);	SquareRoot Swap
control_not( qu, bit);	Controlled Not
control_x(qu, bit);	Controlled Pauli-X
control_y(qu, bit);	Controlled Pauli-Y
control_z(qu, bit);	Controlled Pauli-Z
```

####Phase gates are special

```
control_phase(qu, bit, NUMERATOR, DENOMINATOR);`
phase(qubit, NUMERATOR, DENOMINATOR);
```

The phase is represenative of `pi*NUMERATOR/DENOMINATOR` of a radian value. `NUMERATOR` and `DENOMINATOR` follow the same buggy floating point parsing so always use a decimal, even if you are working with purely integer values. 

#Compile Guide:


####Set up Fedora

TODO

####Set up Ubuntu

TODO

####Set up OSX

TODO

####Set up Windows MSVC

TODO

####Set up WIndows GNU

TODO

#Operation
This program expects `OpenCL` to be installed locally. Please ensure your Vendor (AMD, Nvidia, or Intel's) OpenCL-SDK is installed. This program will also assume both `Rust` and `Cargo` are installed locally Please report all bugs you have.


#####Compile and run

```
quantumcompiler -i [YOUR FILE]
```

#####Compile a stand alone

```
quantumcompiler -c [YOUR FILE]
```

#####Get help

```
quantumcompiler -h
```


