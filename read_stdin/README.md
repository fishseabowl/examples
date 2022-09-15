
You cannot return a '&str' if you've allocated the String in the function. There's further discussion about why, as well as the fact that it's not limited to strings. That makes your choice much easier: return the String.

'Strings' are heap-allocated and built to be mutable.

'Strings' are heap-allocated because they have an unknown length. Since that allocation is solely owned by the String, that's what grants the ability to mutate the string.

My function just returns a filepath for reference purposes, and I'd rather leave it up to the caller to decide if they need a heap-stored mutable string.

This isn't possible. Your function has performed an allocation. If you don't return the allocation to the caller, then the value must be deallocated to prevent memory leaks. If it was returned after deallocation, that would be an invalid reference, leading to memory safety violations.