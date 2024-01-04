part2: 120 seconds, 0.3MB RAM
part2_multithread: 45 seconds, 0.9MB RAM
I could feasibly match the amount of RAM (iterate over the same map somehow) 
and decrease solving time to like 15 seconds on my PC (by splitting up large ranges into multiple parts)
but I don't really want to!

part2_seed_per_lines was an attempt to get marginal gains, but I underestimated how much
the break; helps in the inner for loop