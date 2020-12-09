
## self
 there are four special first
parameter values: &self, self, &mut self, and mut self. All of the forms turn
a function in a method on an instance of the type.

- The first form, &self, is the most common form. This means that our method takes
an immutable reference to the instance invoking the method. We can read the data
inside our type, but we cannot alter it. The calling code also maintains ownership so
we are just borrowing the instance.

- The second form, self, means that the method consumes self and therefore the
instancethatthemethodisbeingcalledonhasitsownershipmovedintothemethod.
This form comes usually when we are transforming a type into something else, for
example with interfaces that use the builder pattern.

- The third form, &mut self, is the mutable version of the first form. This is the second
most common thing you will encounter. Our method can read and write the data
inside our type, but it does not own the value so this access is only temporary.

- Finally the last form, mut self, means that this method consumes self and self is
mutable within the method. All parameters to functions can be declared mutable if
you wish them to be a mutable binding inside the function, and self is no different.
This has its uses, but is not that common.