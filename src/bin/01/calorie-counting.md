# Calorie Counting

## Part One

This was a pretty straightforward problem. When I learned about the `lines` method in Rust core for strings, I decided I would return the `String` from the `fs::read_to_string` operation and iterate over each line of it.

The simplest approach I could think of was that since an empty line separated each of the elve's food, I could sum up each elve's calories as I iterated. When I hit an empty line, I knew that I had the elve's calorie sum.

The next step would then be to then check the sum against the largest sum I had encountered up until that point. If it was greater, then it would become the new largest sum, otherwise I would continue to the next iteration.

After one iteration through each line of the input, I would have the largest sum.

Part one done!

## Part Two

I had to think a little bit more for part two. My first thought was to keep track of the three largest sums I had encountered in a vector. I relized that this would require a lot of checking against that vector and shifting/inserting its values.

My next thought was much simpler. I could create a vector of all the calorie sums, sort it, then take the last three entires and sum them up.

Part two done!

## Final Thoughts

I spent a lot of time referencing the Rust documentation but it wasn't too bad. The compiler was helpful in pointing out where I needed a reference or when I needed to mark a variable as mutable. So far Rust has been great to write.
