# RuntimeID

RuntimeID provides lightweight unique identifers per 'run' of a program.

Internally this is just a usize that counts up from zero using atomic instructions. This makes RuntimeIDs
extremely cheap to create and compare with the downside that they cannot be serialized.
