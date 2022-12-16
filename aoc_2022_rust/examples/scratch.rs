#[derive(Debug)]
struct Dir {
    name: String,
    files: Vec<File>,
    dirs: Vec<Dir>
}



#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}


fn main() {
    let name = "Ok";
    let mut main: Dir = Dir { name: "/".to_string(), files: vec![], dirs: vec![]};
    let mut sub: Dir = Dir { name: "ok".to_string(), files: vec![], dirs: vec![]};
    let mut sub2: Dir = Dir { name: "ok2".to_string(), files: vec![], dirs: vec![]};

    let mut current = &mut main;

    sub.files.push(File { name: name.to_string(), size: 300 });
    sub.files.push(File { name: name.to_string(), size: 200 });
    sub.files.push(File { name: name.to_string(), size: 0 });

    
    sub2.files.push(File { name: name.to_string(), size: 300 });
    sub2.files.push(File { name: name.to_string(), size: 200 });
    sub2.files.push(File { name: name.to_string(), size: 100 });

    current.files.push(File { name: name.to_string(), size: 300 });
    current.files.push(File { name: name.to_string(), size: 200 });
    current.files.push(File { name: name.to_string(), size: 0 });
    current.dirs.push(sub);
    current.dirs.push(sub2);
    current.files.push(File { name: name.to_string(), size: 300 });
    current.files.push(File { name: name.to_string(), size: 200 });
    current.files.push(File { name: name.to_string(), size: 100 });


    let mut sub3: Dir = Dir { name: "ok3".to_string(), files: vec![], dirs: vec![]};

    sub3.files.push(File { name: "DAMN".to_string(), size: 99});

    let newone = &mut main;

    current = &mut sub3;

    println!("\n\n{:?}", current);

    println!("\n\n{:?}", newone);

    let num = size_total(&current);



    
    println!("Totals: {}", size_total(&main)); 

    println!("Size: {}", num);

    
}

fn size_total(current: &Dir) -> u32 {
    if current.files.len() == 0 && !current.dirs.len() == 0 {
        return 0
    }
    let file_sum: u32 = current.files.iter().map(|x| x.size).sum();
    let dir_sum = {
        let mut totals: u32 = 0;
        for dir in &current.dirs {
            totals += size_total(&dir)
        }
        totals
    };
    file_sum + dir_sum

}