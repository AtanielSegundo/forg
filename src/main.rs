    use std::default;
    use std::env;
    use std::fs;

#[derive(Default,Debug)]
struct Directory{
    file_names : Vec<String>, 
    extensions : Vec<String>,
}

impl Directory{

    fn push(&mut self,tuple : (String,String)){
        self.file_names.push(tuple.0);
        self.extensions.push(tuple.1);
    }
}

fn FileNameParser(file_name:std::ffi::OsString) -> (String,String){
    let aux = file_name.into_string().unwrap();
    let mut splited = aux.split(".");
    let name : String =  String::from(splited.next().unwrap_or(""));
    let extension : String = String::from(splited.next().unwrap_or(""));

    return (name,extension)
}

fn main(){
// Iniicializando váriaveis.
let mut actual_dir = Directory::default();
let file_extensions: [&str; 7] = ["pdf", "docx", "txt", "jpg", "xlsx", "mp3", "mp4"];
let input_args : Vec<String> = env::args().collect();
if input_args.len() < 2 {eprintln!("Sem Argumentos"); return};
let input_path = input_args[1].clone();

// Processando o diretorio
let dir = fs::read_dir(input_path).unwrap();

for file in dir{
    match file{
        Ok(file) => {
            if file.file_type().unwrap().is_file(){
                let aux = FileNameParser(file.file_name());
                actual_dir.push(aux);
            }    
        }
        Err(err) => println!("Erro"),
    }


}
}