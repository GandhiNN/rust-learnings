# Borrowed vs Owned Type

## Borrowed Type
Rust Borrowed types are types that do not follow the rules for Ownership.  
Variables of this kind have an ampersand `&` in front of them.  
Borrowed types have the location addresses of the values they represent.
  
## Owned Type
Rust Owned type follows the Rust Ownership rules.  
These variables contain the actual values they represent.  

## Now What?
We could return Owned values from functions and methods without resorting to global static variables.   
In the same token, we know that we should not return a *reference type* from a function or method.  
