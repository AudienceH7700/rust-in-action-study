fn main(){
    let needle = 0o204;
    let haystack = [1, 1, 2, 5, 15, 52, 203, 877, 4140, 21147];

    println!("needle value = {}", needle);

    for item in &haystack {     
        if *item == needle {
            println!("matched = {}", item);
        }else{
            println!("not matched = {}", item);
        }
    }
}