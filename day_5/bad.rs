// for seed_start in (0..seeds.len()).step_by(2) {
    //     let item: Vec<_> = (seeds[seed_start]..(seeds[seed_start] + seeds[seed_start+1])).map(|seed| {
    //         let maps = &maps;
    //         tokio::spawn(async move{
    //             let mut seed_val = seed;
    //             for map in maps.iter() {
    //                 for map_line in map {
    //                     if 
    //                         seed_val >= map_line.source &&
    //                         seed_val < map_line.end
    //                     {
    //                         seed_val = map_line.start + (seed_val - map_line.source);
    //                         break;
    //                     }
    //                 }
    //             }
    //             return min(minimum, seed_val)
    //             }
                  
    //         );
                
    //     }).collect();
    // }
    
    // let tasks: Vec<_> = (0..seeds.len())
    //     .step_by(2)
    //     .flat_map(|seed_start| {
    //         let maps_ref = Arc::clone(&maps);

            // (seeds[seed_start]..(seeds[seed_start] + seeds[seed_start+1])).map(move |seed| {
            //     tokio::spawn(async move {
            //         let mut seed_val = seed;
            //         for map in maps_ref.iter() {
            //             for map_line in map {
            //                 if seed_val >= map_line.source && seed_val < map_line.end {
            //                     seed_val = map_line.start + (seed_val - map_line.source);
            //                     break;
            //                 }
            //             }
            //         }
            //         min(minimum, seed_val);
            //     })
            // })
            
    //     }).collect();
    
    // join_all(tasks).await;