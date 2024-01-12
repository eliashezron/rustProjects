extern crate zip;
use std::io;
use std::env::args;
use std::fs;

fn main() {
    std::process::exit(unzip())
}

fn unzip() -> i32{
    let args: Vec<String> = args().collect();
    if args.len()> 2 {
        eprint!("Usage: {} ", args[0]);
        return 1;
    }
    let fname = std::path::Path::new(&*args[1]); //get file name from args
    let file = fs::File::open(fname).unwrap(); //open file
    let mut archive = zip::ZipArchive::new(file).unwrap(); //create zip archive

    for i in 0..archive.len(){ //iterate through the archive
        let mut file = archive.by_index(i).unwrap(); //get file from archive

        let outpath = match file.enclosed_name(){ //get file name
            Some(path) => path.to_owned(), //if file name exists, get it
            None => continue, //if file name doesn't exist, continue
        };
        //check for comments in the file
        {
            let comment = file.comment(); //get comment
            if !comment.is_empty(){
                println!("File {} comment: {}", i, comment);
            }
        }
        if(&*file.name()).ends_with('/'){ //check if file is a directory
            println!("File {} extracted to \"{}\"", i, outpath.as_path().display()); //print directory name
            fs::create_dir_all(&outpath).unwrap(); //create directory
        } else {
            println!("File {} extracted to \"{}\" ({} bytes)", i, outpath.as_path().display(), file.size()); //print file name and size
            if let Some(p) = outpath.parent(){ //check if file has a parent
                if !p.exists(){ //check if parent exists
                    fs::create_dir_all(&p).unwrap(); //create parent directory
                }
            }
            let mut outpath = fs::File::create(&outpath).unwrap(); //create file
            io::copy(&mut file, &mut  outpath).unwrap(); //copy file to outpath
        }
        #[cfg(unix)] //check if OS is unix
        {
            use std::os::unix::fs::PermissionsExt; //import unix permissions
            if let Some(mode) = file.unix_mode(){ //check if file has unix permissions
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap(); //set file permissions
            }
        } 
    }
    0 //return 0 int32
}